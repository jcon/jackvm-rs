use wasm_bindgen::prelude::JsValue;
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

    Ok(canvas)
}
