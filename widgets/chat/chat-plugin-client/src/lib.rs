use custom_elements::{inject_style, CustomElement};
use hutopia_plugin_client::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::js_sys;
use web_sys::{window, Event, HtmlElement, Node};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use chat_plugin_protocol as protocol;
use protocol::*;
use uuid::Uuid;
use chat_plugin_protocol::protocol::{Parcel, Settings};
use protocol::message::*;


const CUSTOM_HTML_TAG: &str = "widget-chat";

// The boring part: a basic DOM component
struct ChatComponent {
    ws: Rc<WebSocket>,
}

impl ChatComponent {
    fn new() -> Self {
        let ws = Rc::new(init_socket());

        Self { ws }
    }

    fn view(&self) -> Node {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let container = document.create_element("div").unwrap();
        let msg_container = document.create_element("div").unwrap();
        let btn = document.create_element("button").unwrap();
        let input = document.create_element("input").unwrap();
        let btn_text = document.create_text_node("Send msg");

        input.set_attribute("type", "text");

        btn.append_child(&btn_text).unwrap();
        container.append_child(&msg_container);
        container.append_child(&document.create_element("br").unwrap());
        container.append_child(&input);
        container.append_child(&btn);

        let input_cast = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        // Send messages

        let ws = self.ws.clone();
        let onclick = Closure::wrap(Box::new(move |_event: Event| {
            let msg = input_cast.value();
            input_cast.set_value("");

            // Send SendMsg packet
            let packet = ProtocolMessage::ServerBound(ServerBoundPacket::SendMsg(msg));
            let bytes = packet.raw_bytes(&Settings::default()).unwrap();
            let _ = ws.send_with_u8_array(&bytes);
        }) as Box<dyn FnMut(Event)>);
        // Forget the closure to avoid dropping it prematurely

        // Add the onclick event listener to the button
        btn.set_attribute("onclick", "").unwrap();
        btn.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())
            .unwrap();

        // Receive messages

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let array = js_sys::Uint8Array::new(&abuf);
                let vec = array.to_vec(); // TODO maybe use something that doesn't copy it?
                let message = {
                    let settings = &Settings::default();
                    let res = ProtocolMessage::from_raw_bytes(&vec, settings);
                    res.unwrap()
                };

                if let ProtocolMessage::ClientBound(ClientBoundPacket::ServeMsg(id, msg)) = message {
                    let msg = format!("{:?}: {msg}", id.0);
                    let txt_node = document.create_text_node(&msg);
    
                    msg_container.append_child(&txt_node).unwrap();
                    msg_container.append_child(&document.create_element("br").unwrap());
                }
            }
        });
        self.ws
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        onmessage_callback.forget();
        onclick.forget();

        container.unchecked_into()
    }
}

impl Default for ChatComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ChatComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(this, "p { color: green; }");
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["name"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        _name: String,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        log("attribute changed");
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        log("connected");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        log("disconnected");
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        log("adopted");
    }
}

// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ChatComponent::define(CUSTOM_HTML_TAG);

    Ok(())
}

fn init_socket() -> WebSocket {
    let ws = WebSocket::new("ws://localhost:8080/widget_ws/chat").unwrap();
    // TODO: you should switch to blob type for big transfers like files
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let _cloned_ws = ws.clone();
    /*let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
            // here you can for example use Serde Deserialize decode the message
            // for demo purposes we switch back to Blob-type and send off another binary message
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
            match cloned_ws.send_with_u8_array(&[5, 6, 7, 8]) {
                Ok(_) => console_log!("binary message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);
            // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            // create onLoadEnd callback
            let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let len = array.byte_length() as usize;
                console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
                // here you can for example use the received image/png data
            });
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", txt);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    });
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();*/

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");

        // TOOD: Connect packet
        let uuid = Uuid::new_v4();
        console_log!("Generating UUID: {uuid}");

        let ser = SerializableUuid(uuid);
        let packet = ProtocolMessage::ServerBound(ServerBoundPacket::Connect(ser));
        let bytes = packet.raw_bytes(&Settings::default()).unwrap();
        let _ = cloned_ws.send_with_u8_array(&bytes);
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    ws
}
