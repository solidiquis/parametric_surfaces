mod geometry;
mod shaders;
use crate::{gl_context, shader, fmt_mat_f32, buf_f32};
use js_sys::{JsString, Number};
use nalgebra_glm as glm;
use std::f32::consts::PI;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlProgram as Program;

#[wasm_bindgen]
pub struct Triforce {
    gl: GL,
    triforce_shader: shader::Shader,
}

type TriforceResult<T> = Result<T, JsValue>;

#[wasm_bindgen]
impl Triforce {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: JsString) -> Self {
        match Self::try_new(canvas_id) {
            Ok(triforce) => triforce,
            Err(e) => wasm_bindgen::throw_val(e),
        }
    }

    fn try_new(canvas_id: JsString) -> TriforceResult<Self> {
        let gl = gl_context::init_gl_context(canvas_id)?;
        let triforce_shader = shader::Shader::new(
            &gl, shaders::TRIFORCE_VS_GLSL, shaders::TRIFORCE_FS_GLSL
        )?;
        Self::init_vertices(&gl, &triforce_shader.program)?;

        Ok( Self { gl, triforce_shader } )
    }

    #[wasm_bindgen]
    pub fn render(&self, canvas_width: Number, canvas_height: Number, dtheta: Number) {
        match self.try_render(canvas_width, canvas_height, dtheta) {
            Ok(()) => (),
            Err(e) => wasm_bindgen::throw_val(e)
        }
    }

    fn try_render(&self, canvas_width: Number, canvas_height: Number, _dtheta: Number) -> TriforceResult<()> {
        let width = canvas_width.as_f64().unwrap();
        let height = canvas_height.as_f64().unwrap();

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.triforce_shader.use_shader(&self.gl);

        self.triforce_shader.set_mat4_f32(&self.gl, "v", &self.view_matrix())?;
        self.triforce_shader.set_mat4_f32(&self.gl, "p", &self.projection_matrix(width, height))?;

        self.triforce_shader.set_mat4_f32(&self.gl, "m", &self.top_model_matrix())?;
        self.gl.draw_arrays(GL::TRIANGLES, 0, 3);

        self.triforce_shader.set_mat4_f32(&self.gl, "m", &self.bottom_left_model_matrix())?;
        self.gl.draw_arrays(GL::TRIANGLES, 0, 3);

        self.triforce_shader.set_mat4_f32(&self.gl, "m", &self.bottom_right_model_matrix())?;
        self.gl.draw_arrays(GL::TRIANGLES, 0, 3);

        self.gl.flush();

        Ok(())
    }

    fn init_vertices(gl: &GL, program: &Program) -> TriforceResult<()> {
        // *======== Positions data ========*
        let positions_buffer = gl.create_buffer()
            .ok_or_else(|| JsValue::from_str("Failed to initialize positions vbo."))?;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&positions_buffer));

        let positions_data = buf_f32!(&geometry::VERTICES);

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

        let colors_data = buf_f32!(&geometry::COLORS);

        gl.buffer_data_with_opt_array_buffer(
            GL::ARRAY_BUFFER, Some(&colors_data), GL::STATIC_DRAW
        );

        let color_attr = gl
            .get_attrib_location(program, "color") as u32;

        gl.vertex_attrib_pointer_with_i32(color_attr, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(color_attr);

        Ok(())
    }

    fn top_model_matrix(&self) -> Vec<f32> {
        let identity = glm::TMat4::identity();
        let transl = glm::translate(&identity, &glm::vec3(0.0, 0.5, -3.0));
        let mat = transl;

        fmt_mat_f32!(mat)
    }

    fn bottom_left_model_matrix(&self) -> Vec<f32> {
        let identity = glm::TMat4::identity();
        let transl = glm::translate(&identity, &glm::vec3(-0.5, -0.5, -3.0));
        let mat = transl;

        fmt_mat_f32!(mat)
    }

    fn bottom_right_model_matrix(&self) -> Vec<f32> {
        let identity = glm::TMat4::identity();
        let transl = glm::translate(&identity, &glm::vec3(0.5, -0.5, -3.0));
        let mat = transl;

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

