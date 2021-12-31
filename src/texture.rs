use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::closure::Closure;
use web_sys::WebGlRenderingContext as GL;
use web_sys::HtmlImageElement;
use web_sys::WebGlTexture;

type TextureError<T> = Result<T, JsValue>;

fn load_texture(gl: &GL, url: &str) -> TextureError<WebGlTexture> {
    let texture = gl.create_texture()
        .ok_or_else(|| JsValue::from_str("Failed to initialize texture."))?;

    let pixel: [u8; 4] = [0, 0, 0, 255];
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D, 0, GL::RGBA as i32,
        1, 1, 0,
        GL::RGBA, GL::UNSIGNED_BYTE, Some(&pixel)
    )?;

    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("Failed to get window object."))?;

    let document = window.document()
        .ok_or_else(|| JsValue::from_str("Failed to get document object."))?;

    let img = document.create_element("img")?.dyn_into::<HtmlImageElement>()?;

    let t = texture.clone();
    let i = img.clone();
    let g = gl.clone();
    let onload = Closure::wrap(Box::new( move || {
        g.bind_texture(GL::TEXTURE_2D, Some(&t));
        if let Err(e) = g.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D, 0, GL::RGBA as i32,
            GL::RGBA, GL::UNSIGNED_BYTE, &i
        ) {
            wasm_bindgen::throw_val(e)
        }

        if is_power_of_2(i.width()) && is_power_of_2(i.height()) {
            g.generate_mipmap(GL::TEXTURE_2D);
        } else {
            g.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
            g.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
            g.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        }
    }) as Box<dyn Fn()>);

    img.set_onload(Some(onload.as_ref().unchecked_ref()));

    img.set_src(url);

    Ok(texture)
}

fn is_power_of_2(num: u32) -> bool {
    num & (num - 1) == 0
}
