use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, HtmlElement, Window};

pub struct JsGlobal {
    pub window: Window,
    pub body: HtmlElement,
    pub document: Document,
}

impl JsGlobal {
    pub fn create() -> Result<JsGlobal, JsValue> {
        let window =
            web_sys::window().ok_or(&JsValue::from_str("Can't initialize global window object"))?;

        let document = window
            .document()
            .ok_or(&JsValue::from_str("Can't initialize window.document"))?;

        let body = document
            .body()
            .ok_or(&JsValue::from_str("Can't initialize body element"))?;

        Ok(JsGlobal {
            window,
            document,
            body,
        })
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Can't register callback with `requestAnimationFrame`")
}

pub fn create_canvas(
    document: &Document,
    height: u32,
    width: u32,
) -> Result<HtmlCanvasElement, JsValue> {
    let canvas = document
        .create_element("canvas")
        .map_err(|_| JsValue::from_str("Can't create canvas element"))?;

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Cannot cast canvas element"))?;

    canvas.set_height(height);
    canvas.set_width(width);
    canvas.set_tab_index(1);

    Ok(canvas)
}
