mod geometry;
mod shaders;
mod transform;
use js_sys::{JsString, Float32Array, Number};
use std::collections::HashMap;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlProgram as Program;
use web_sys::WebGlShader as Shader;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation as UniformLocation;

#[wasm_bindgen]
pub struct Torus {
    gl: GL,
    program: Program,
    unilocs: HashMap<String, UniformLocation>,
    indices_count: i32,
}

#[wasm_bindgen]
impl Torus {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: JsString) -> Self {
        let gl = match Self::init_context(canvas_id) {
            Ok(context) => context,
            Err(e) => wasm_bindgen::throw_val(e)
        };

        let program = match Self::init_shader_program(&gl) {
            Ok(prog) => prog,
            Err(e) => wasm_bindgen::throw_val(e)
        };

        if let Err(e) = Self::init_shaders(&gl, &program) {
            wasm_bindgen::throw_val(e)
        }

        let indices_count = match Self::init_vertices(&gl, &program) {
            Ok(i) => i,
            Err(e) => wasm_bindgen::throw_val(e)
        };

        let unilocs = match Self::map_uniform_locations(&gl, &program) {
            Ok(locations) => locations,
            Err(e) => wasm_bindgen::throw_val(e)
        };

        gl.use_program(Some(&program));

        Self { gl, program, unilocs, indices_count }
    }

    #[wasm_bindgen]
    pub fn render(&self, canvas_width: Number, canvas_height: Number, dtheta: Number) {
        let width = canvas_width.as_f64().unwrap();
        let height = canvas_height.as_f64().unwrap();

        self.gl.enable(GL::DEPTH_TEST);
        self.gl.depth_func(GL::LEQUAL);
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let m_loc = self.unilocs.get("m").unwrap();
        let v_loc = self.unilocs.get("v").unwrap();
        let p_loc = self.unilocs.get("p").unwrap();

        self.gl.uniform_matrix4fv_with_f32_array(Some(m_loc), false, &transform::model_matrix(dtheta));
        self.gl.uniform_matrix4fv_with_f32_array(Some(v_loc), false, &transform::view_matrix());
        self.gl.uniform_matrix4fv_with_f32_array(Some(p_loc), false, &transform::projection_matrix(width, height));

        self.gl.draw_arrays(GL::POINTS, 0, self.indices_count);
        self.gl.flush();
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

    fn init_shaders(gl: &GL, program: &Program) -> Result<(), JsValue> {
        let vertex_shader = match gl.create_shader(GL::VERTEX_SHADER) {
            None => return Err(JsValue::from_str("Failed to initialize vertex shader.")),
            Some(shader) => match Self::compile_shader(gl, shader, shaders::VS_SRC_GLSL) {
                Ok(s) => s,
                Err(e) => return Err(e)
            }
        };

        let fragment_shader = match gl.create_shader(GL::FRAGMENT_SHADER) {
            None => return Err(JsValue::from_str("Failed to initialize fragment_shader.")),
            Some(shader) => match Self::compile_shader(gl, shader, shaders::FS_SRC_GLSL) {
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

    fn init_vertices(gl: &GL, program: &Program) -> Result<i32, JsValue> {
        let (positions, colors, indices_count) = geometry::compute_vertices();

        let positions_buffer = match gl.create_buffer() {
            None => return Err(JsValue::from_str("Failed to initialize positions buffer.")),
            Some(buf) => {
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buf));        

                let slice: &[f32] = &positions;
                let array = Float32Array::from(slice).buffer();

                gl.buffer_data_with_opt_array_buffer(
                    GL::ARRAY_BUFFER, Some(&array), GL::STATIC_DRAW
                );

                buf
            }
        };

        let colors_buffer = match gl.create_buffer() {
            None => return Err(JsValue::from_str("Failed to initialize colors buffer.")),
            Some(buf) => {
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buf));

                let slice: &[f32] = &colors;
                let array = Float32Array::from(slice).buffer();

                gl.buffer_data_with_opt_array_buffer(
                    GL::ARRAY_BUFFER, Some(&array), GL::STATIC_DRAW
                );

                buf
            }
        };

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&positions_buffer));
        let position = gl.get_attrib_location(program, "position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&colors_buffer));
        let color = gl.get_attrib_location(program, "color") as u32;
        gl.vertex_attrib_pointer_with_i32(color, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(color);

        Ok(indices_count)
    }

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
}
