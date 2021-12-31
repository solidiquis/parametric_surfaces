pub const TRIFORCE_VS_GLSL: &'static str = r#"

attribute vec3 position;
attribute vec3 normal;
attribute vec2 texCoord;
varying highp vec2 vTextureCoord;
varying vec3 vNormal;

uniform mat4 p;
uniform mat4 m;
uniform mat4 v;

void main()
{
    gl_Position = p * v * m * vec4(position, 1.0);
    vTextureCoord = texCoord;
    vNormal = normal;
}

"#;

pub const TRIFORCE_FS_GLSL: &'static str = r#"

precision mediump float;
varying highp vec2 vTextureCoord;
varying vec3 vNormal;
uniform sampler2D uSampler;

void main()
{
  gl_FragColor = texture2D(uSampler, vTextureCoord);
}

"#;
