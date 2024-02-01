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

// https://rustwasm.github.io/wasm-bindgen/examples/websockets.html

const CUSTOM_HTML_TAG: &str = "widget-chat";
const CHAT_WS_ADDRESS_PROP: &str = "CHAT_WS_ADDRESS";

// The DOM component
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

                if let ProtocolMessage::ClientBound(message) = message {
                    match message {
                        ClientBoundPacket::ServeMsg(id, msg) => {
                            let msg = format!("{:?}: {msg}", id.0);
                            let txt_node = document.create_text_node(&msg);
            
                            msg_container.append_child(&txt_node).unwrap();
                            msg_container.append_child(&document.create_element("br").unwrap());
                        },
                        _ => {},
                    }
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
    ) { }

    fn connected_callback(&mut self, _this: &HtmlElement) { }

    fn disconnected_callback(&mut self, _this: &HtmlElement) { }

    fn adopted_callback(&mut self, _this: &HtmlElement) { }
}

// wasm_bindgen entry point defines the custom element
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    ChatComponent::define(CUSTOM_HTML_TAG);

    Ok(())
}

fn init_socket() -> WebSocket {
    // Read the websocket address
    let window = window().unwrap();
    let address = js_sys::Reflect::get(
        &JsValue::from(web_sys::window().unwrap()),
        &JsValue::from(CHAT_WS_ADDRESS_PROP),
    )
    .unwrap()
    .as_string()
    .unwrap();

    let ws_address = format!("ws://{}/widget_ws/chat", address);
    console_log!("The address is {ws_address}");

    let ws = WebSocket::new(&ws_address).unwrap();
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let _cloned_ws = ws.clone();
    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");

        // Connect packet
        let uuid = Uuid::new_v4();
        console_log!("Generating UUID: {uuid}");

        let ser = SerializableUuid(uuid);
        let packet = ProtocolMessage::ServerBound(ServerBoundPacket::Connect(ser));
        let bytes = packet.raw_bytes(&Settings::default()).unwrap();
        let _ = cloned_ws.send_with_u8_array(&bytes);

        // Send QueryMsg packet
        let packet = ProtocolMessage::ServerBound(ServerBoundPacket::QueryMsg);
        let bytes = packet.raw_bytes(&Settings::default()).unwrap();
        let _ = cloned_ws.send_with_u8_array(&bytes);
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    ws
}
