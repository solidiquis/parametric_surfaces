pub const VS_SRC_GLSL: &'static str = r#"

attribute vec3 position;
attribute vec3 color;
varying vec4 vColor;
uniform mat4 p;
uniform mat4 m;
uniform mat4 v;

void main()
{
    vColor = vec4(color, 1.0);
    gl_Position = p * v * m * vec4(position, 1.0);
    gl_PointSize = 2.0;
}
"#;

pub const FS_SRC_GLSL: &'static str = r#"

precision mediump float;
varying   vec4 vColor;

void main()
{
    gl_FragColor  = vColor;
}

"#;
