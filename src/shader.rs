use wasm_bindgen::JsValue;
use web_sys::WebGlProgram as Program;
use web_sys::WebGlRenderingContext as GL;

struct Shader<'a> {
    pub program: Program,
    pub gl: &'a GL
}

type ShaderResult<T> = Result<T, JsValue>;

impl<'a> Shader<'a> {
    pub fn new(gl: &'a GL, vsrc: &'static str, fsrc: &'static str) -> ShaderResult<Self> {
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

        Ok(Self { program, gl })
    }
}
