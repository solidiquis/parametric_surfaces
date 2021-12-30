use js_sys::JsString;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::HtmlCanvasElement;
use web_sys::WebGlRenderingContext as GL;

pub fn init_gl_context(canvas_id: JsString) -> Result<GL, JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("Failed to get window object."))?;

    let document = window.document()
        .ok_or_else(|| JsValue::from_str("Failed to get document object."))?;

    let element_id = String::from(canvas_id);

    let element = document
        .get_element_by_id(&element_id)
        .ok_or_else(|| {
            JsValue::from(format!("Failed to get canvas element with ID '{}'", element_id))
        })?;

    let canvas = element.dyn_into::<HtmlCanvasElement>()?;
    let gl_context = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?; 

    // Basic global state settings:
    gl_context.enable(GL::DEPTH_TEST);
    gl_context.depth_func(GL::LEQUAL);

    Ok(gl_context)
}
