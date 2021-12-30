pub const TRIFORCE_VS_GLSL: &'static str = r#"

attribute vec3 position;
attribute vec3 color;
varying vec3 vColor;

uniform mat4 p;
uniform mat4 m;
uniform mat4 v;

void main()
{
    gl_Position = p * v * m * vec4(position, 1.0);
    vColor = color;
}

"#;

pub const TRIFORCE_FS_GLSL: &'static str = r#"

precision mediump float;
varying vec3 vColor;

void main()
{
    gl_FragColor  = vec4(vColor, 1.0);
}

"#;
