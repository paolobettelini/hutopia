use custom_elements::{inject_style, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, Text, Event};
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use std::rc::Rc;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// The boring part: a basic DOM component
struct ExampleComponent {
    ws: Rc<WebSocket>
}

impl ExampleComponent {
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
            let _ = ws.send_with_str(&msg);
        }) as Box<dyn FnMut(Event)>);
        // Forget the closure to avoid dropping it prematurely
    
        // Add the onclick event listener to the button
        btn.set_attribute("onclick", "").unwrap();
        btn.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref()).unwrap();
    
        // Receive messages

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(msg) = e.data().dyn_into::<js_sys::JsString>() {
                let msg = format!("{msg}");
                let txt_node = document.create_text_node(&msg);

                msg_container.append_child(&txt_node).unwrap();
                msg_container.append_child(&document.create_element("br").unwrap());

                console_log!("Received text: {:?}", msg);
            }
        });
        self.ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        
        onmessage_callback.forget();
        onclick.forget();

        container.unchecked_into()
    }
}

impl Default for ExampleComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for ExampleComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(&this, "p { color: green; }");
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["name"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
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
    ExampleComponent::define("custom-vanilla");

    Ok(())
}

fn init_socket() -> WebSocket {
    let ws = WebSocket::new("ws://localhost:8080/widget_ws/example").unwrap();
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    //ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let cloned_ws = ws.clone();
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

        //let _ = cloned_ws.send_with_str("Hello from the client. Can you hear me 1?");
        //let _ = cloned_ws.send_with_str("Hello from the client. Can you hear me 2?");
        //let _ = cloned_ws.send_with_str("Hello from the client. Can you hear me 3?");

        // send off binary message
        //match cloned_ws.send_with_u8_array(&[0, 1, 2, 3]) {
        //    Ok(_) => console_log!("binary message successfully sent"),
        //    Err(err) => console_log!("error sending message: {:?}", err),
        //}
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    ws
}