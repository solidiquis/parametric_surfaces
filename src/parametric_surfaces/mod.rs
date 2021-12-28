pub mod torus;
pub mod triforce;
use js_sys::JsString;
use std::collections::HashMap;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::HtmlCanvasElement;
use web_sys::WebGlProgram as Program;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader as Shader;
use web_sys::WebGlUniformLocation as UniformLocation;

trait ParametricSurface {
    fn init_vertices(gl: &GL, program: &Program) -> Result<Option<i32>, JsValue>;

    fn init_context(canvas_id: JsString) -> Result<GL, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element_id = String::from(canvas_id);

        let element = match document.get_element_by_id(&element_id) {
            Some(e) => e,
            None => {
                let err = JsValue::from_str(&format!(
                    "No canvas found with id: {}", element_id
                ));
                return Err(err)
            }
        };

        let canvas = element.dyn_into::<HtmlCanvasElement>()?;
        let gl_context = canvas.get_context("webgl")?.unwrap().dyn_into::<GL>()?; 

        Ok(gl_context)
    }

    fn init_shader_program(gl: &GL) -> Result<Program, JsValue> {
        match gl.create_program() {
            Some(prog) => Ok(prog),
            None => Err(JsValue::from_str("Failed to initialize shader program."))
        }
    }

    fn compile_shader(gl: &GL, shader: Shader, shader_src: &'static str) -> Result<Shader, JsValue> {
        gl.shader_source(&shader, shader_src);
        gl.compile_shader(&shader);

        if let false = gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap() {
            let log = gl.get_shader_info_log(&shader).unwrap();
            let err = JsValue::from(format!("An error occurred compiling shader: {}", log));
            gl.delete_shader(Some(&shader));
            return Err(err)
        }

        Ok(shader)
    }

    fn init_shaders(gl: &GL, program: &Program, vs_src: &'static str, fs_src: &'static str) -> Result<(), JsValue> {
        let vertex_shader = match gl.create_shader(GL::VERTEX_SHADER) {
            None => return Err(JsValue::from_str("Failed to initialize vertex shader.")),
            Some(shader) => match Self::compile_shader(gl, shader, vs_src) {
                Ok(s) => s,
                Err(e) => return Err(e)
            }
        };

        let fragment_shader = match gl.create_shader(GL::FRAGMENT_SHADER) {
            None => return Err(JsValue::from_str("Failed to initialize fragment_shader.")),
            Some(shader) => match Self::compile_shader(gl, shader, fs_src) {
                Ok(s) => s,
                Err(e) => return Err(e)
            }
        };

        gl.attach_shader(program, &vertex_shader);
        gl.attach_shader(program, &fragment_shader);
        gl.link_program(program);

        if let false = gl.get_program_parameter(program, GL::LINK_STATUS).as_bool().unwrap() {
            let log = gl.get_program_info_log(program).unwrap();
            let err = JsValue::from_str(&format!("An error occurred compiling shader program: {}", log));
            return Err(err)
        }

        Ok(())
    }

    fn map_uniform_locations(gl: &GL, program: &Program) -> Result<HashMap<String, UniformLocation>, JsValue> {
        let mut unilocs = HashMap::<String, UniformLocation>::new();

        for uniform in ["p", "v", "m"].iter() {
            if let Some(u) = gl.get_uniform_location(program, uniform) {
                unilocs.insert(uniform.to_string(), u);
            } else {
                let err = format!("Failed to get uniform, {}, location", uniform);
                return Err(JsValue::from_str(&err));
            }
        }

        Ok(unilocs)
    }
}
