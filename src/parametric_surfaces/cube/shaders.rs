pub const VS_GLSL: &'static str = r#"

attribute vec3 aPos;

uniform vec3 color;
uniform mat4 m;
uniform mat4 v;
uniform mat4 p;

varying highp vec3 vColor;

void main()
{
    gl_Position = p * v * m * vec4(aPos, 1.0);
    vColor = color;
}

"#;

pub const FS_GLSL: &'static str = r#"

varying highp vec3 vColor;

void main()
{
    gl_FragColor = vec4(vColor, 1.0);
}

"#;
