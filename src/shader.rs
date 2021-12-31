use wasm_bindgen::JsValue;
use web_sys::WebGlProgram as Program;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

pub struct Shader {
    pub program: Program
}

type ShaderResult<T> = Result<T, JsValue>;

impl Shader {
    pub fn new(gl: &GL, vsrc: &'static str, fsrc: &'static str) -> ShaderResult<Self> {
        let program = gl.create_program().ok_or_else(|| {
           JsValue::from("Failed to initialize shader program.")
        })?;

        // *======== Vertex Shader ========*
        let vertex_shader = gl.create_shader(GL::VERTEX_SHADER).ok_or_else(|| {
           JsValue::from_str("Failed to initialize vertex shader.")
        })?;

        gl.shader_source(&vertex_shader, vsrc);
        gl.compile_shader(&vertex_shader);

        if let false = gl.get_shader_parameter(&vertex_shader, GL::COMPILE_STATUS).as_bool().unwrap() {
            let log = gl.get_shader_info_log(&vertex_shader).unwrap();
            let err = JsValue::from(format!("An error occurred compiling shader: {}", log));
            gl.delete_shader(Some(&vertex_shader));
            return Err(err)
        }

        // *======== Fragment Shader ========*
        let fragment_shader = gl.create_shader(GL::FRAGMENT_SHADER).ok_or_else(|| {
           JsValue::from_str("Failed to initialize fragment shader.")
        })?;

        gl.shader_source(&fragment_shader, fsrc);
        gl.compile_shader(&fragment_shader);

        if let false = gl.get_shader_parameter(&fragment_shader, GL::COMPILE_STATUS).as_bool().unwrap() {
            let log = gl.get_shader_info_log(&fragment_shader).unwrap();
            let err = JsValue::from(format!("An error occurred compiling shader: {}", log));
            gl.delete_shader(Some(&fragment_shader));
            return Err(err)
        }

        // *======== Linking ========*
        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        if let false = gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap() {
            let log = gl.get_program_info_log(&program).unwrap();
            let err = JsValue::from(format!("An error occurred compiling shader program: {}", log));
            return Err(err)
        }

        // Shaders successfully linked to our GPU program so shaders on CPU no longer necessary.
        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        Ok(Self { program })
    }

    pub fn use_shader(&self, gl: &GL) {
        gl.use_program(Some(&self.program));
    }

    pub fn set_mat4_f32(&self, gl: &GL, uniform: &str, data: &[f32]) -> ShaderResult<()> {
        let location = self.get_uniform_location(gl, uniform)?;
        gl.uniform_matrix4fv_with_f32_array(Some(&location), false, data);
        Ok(())
    }

    pub fn set_vec3_f32(&self, gl: &GL, uniform: &str, data: &[f32]) -> ShaderResult<()> {
        let location = self.get_uniform_location(gl, uniform)?;
        gl.uniform3fv_with_f32_array(Some(&location), data);
        Ok(())
    }

    pub fn set_i32(&self, gl: &GL, uniform: &str, data: i32) -> ShaderResult<()> {
        let location = self.get_uniform_location(gl, uniform)?;
        gl.uniform1i(Some(&location), data);
        Ok(())
    }

    fn get_uniform_location(&self, gl: &GL, uniform: &str) -> ShaderResult<WebGlUniformLocation> {
        let location = gl.get_uniform_location(&self.program, uniform)
            .ok_or_else(|| JsValue::from(format!("Failed to get location for uniform, '{}'", uniform)))?;
        Ok(location)
    }

}
