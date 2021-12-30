mod geometry;
mod shaders;
use crate::{gl_context, shader, fmt_mat_f32, buf_f32};
use js_sys::{JsString, Number};
use nalgebra_glm as glm;
use std::f32::consts::PI;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGlProgram as Program;
use web_sys::WebGlRenderingContext as GL;

#[wasm_bindgen]
pub struct Torus {
    gl: GL,
    torus_shader: shader::Shader,
    indices_count: i32,
}

type TorusResult<T> = Result<T, JsValue>;

#[wasm_bindgen]
impl Torus {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: JsString) -> Self {
        match Self::try_new(canvas_id) {
            Ok(torus) => torus,
            Err(e) => wasm_bindgen::throw_val(e),
        }
    }

    fn try_new(canvas_id: JsString) -> TorusResult<Self> {
        let gl = gl_context::init_gl_context(canvas_id)?;
        let torus_shader = shader::Shader::new(
            &gl, shaders::VS_GLSL, shaders::FS_GLSL
        )?;
        let indices_count = Self::init_vertices(&gl, &torus_shader.program)?;

        Ok( Self { gl, torus_shader, indices_count } )
    }

    #[wasm_bindgen]
    pub fn render(&self, canvas_width: Number, canvas_height: Number, dtheta: Number) {
        match self.try_render(canvas_width, canvas_height, dtheta) {
            Ok(()) => (),
            Err(e) => wasm_bindgen::throw_val(e)
        }
    }

    pub fn try_render(&self, canvas_width: Number, canvas_height: Number, dtheta: Number) -> TorusResult<()> {
        let width = canvas_width.as_f64().unwrap();
        let height = canvas_height.as_f64().unwrap();

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.torus_shader.use_shader(&self.gl);
        self.torus_shader.set_mat4_f32(&self.gl, "m", &self.model_matrix(dtheta))?;
        self.torus_shader.set_mat4_f32(&self.gl, "v", &self.view_matrix())?;
        self.torus_shader.set_mat4_f32(&self.gl, "p", &self.projection_matrix(width, height))?;

        self.gl.draw_arrays(GL::POINTS, 0, self.indices_count);
        self.gl.flush();

        Ok(())
    }

    fn init_vertices(gl: &GL, program: &Program) -> TorusResult<i32> {
        let (positions, colors, indices_count) = geometry::compute_vertices();

        // *======== Positions data ========*
        let positions_buffer = gl.create_buffer()
            .ok_or_else(|| JsValue::from_str("Failed to initialize positions vbo."))?;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&positions_buffer));

        let positions_data = buf_f32!(&positions);

        gl.buffer_data_with_opt_array_buffer(
            GL::ARRAY_BUFFER, Some(&positions_data), GL::STATIC_DRAW
        );

        let position_attr = gl
            .get_attrib_location(program, "position") as u32;

        gl.vertex_attrib_pointer_with_i32(position_attr, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position_attr);

        // *======== Color data ========*
        let colors_buffer = gl.create_buffer()
            .ok_or_else(|| JsValue::from_str("Failed to initialize colors vbo."))?;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&colors_buffer));

        let colors_data = buf_f32!(&colors);

        gl.buffer_data_with_opt_array_buffer(
            GL::ARRAY_BUFFER, Some(&colors_data), GL::STATIC_DRAW
        );

        let color_attr = gl
            .get_attrib_location(program, "color") as u32;

        gl.vertex_attrib_pointer_with_i32(color_attr, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(color_attr);

        Ok(indices_count)
    }

    fn model_matrix(&self, dtheta: Number) -> Vec<f32> {
        let theta = ((PI / 4.0) + (dtheta.as_f64().unwrap() as f32)) % (2.0 * PI);
        let identity = glm::TMat4::identity();
        let rotate = glm::rotate(&identity, theta, &glm::vec3(0.0, 1.0, 1.0));
        let transl = glm::translate(&identity, &glm::vec3(0.0, 0.0, -2.0));
        let mat = transl * rotate;

        fmt_mat_f32!(mat)
    }

    fn view_matrix(&self) -> Vec<f32> {
        let cam_position = glm::vec3(0.0, 0.0, 0.0);
        let cam_target = glm::vec3(0.0, 0.0, -1.0);
        let cam_up = glm::vec3(0.0, 1.0, 0.0);
        let mat = glm::look_at(&cam_position, &cam_target, &cam_up);

        fmt_mat_f32!(mat)
    }

    fn projection_matrix(&self, canvas_width: f64, canvas_height: f64) -> Vec<f32> {
        let aspect_ratio = canvas_width as f32 / canvas_height as f32;
        let fov = PI / 4.0;
        let near = 0.1;
        let far = 100.0;
        let mat = glm::perspective(aspect_ratio, fov, near, far);

        fmt_mat_f32!(mat)
    }
}
