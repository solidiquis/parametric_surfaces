mod geometry;
mod shaders;
mod transform;
use crate::parametric_surfaces::ParametricSurface;
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

impl ParametricSurface for Torus {
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
}

#[wasm_bindgen]
impl Torus {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: JsString) -> Self {
        match Self::try_new(canvas_id) {
            Ok(torus) => torus,
            Err(e) => wasm_bindgen::throw_val(e),
        }
    }

    fn try_new(canvas_id: JsString) -> Result<Torus, JsValue> {
        let gl = Self::init_context(canvas_id)?;
        let program = Self::init_shader_program(&gl)?;
        Self::init_shaders(&gl, &program, shaders::VS_SRC_GLSL, shaders::FS_SRC_GLSL)?;
        let indices_count = Self::init_vertices(&gl, &program)?;
        let unilocs = Self::map_uniform_locations(&gl, &program)?;

        gl.use_program(Some(&program));

        Ok(Self { gl, program, unilocs, indices_count })
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
}
