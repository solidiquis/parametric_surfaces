pub const TRIFORCE_VS_GLSL: &'static str = r#"

attribute vec3 position;
attribute vec3 normal;
attribute vec2 texCoord;

varying highp vec2 vTextureCoord;
varying highp vec3 vLighting;

uniform mat4 vm; // view-model matrix
uniform mat4 n; // normal matrix
uniform mat4 p; // projection matrix
uniform vec3 lightSource; // vector pointing to light source from NDC origin
uniform vec3 ambientLight;
uniform vec3 lightColor;

void main()
{
    gl_Position = p * vm * vec4(position, 1.0);
    vTextureCoord = texCoord;

    highp vec4 transformedNormal = n * vec4(normal, 0.0);
    highp vec3 normalizedLightSource = normalize(lightSource);
    highp float illuminationIntensity = dot(normalizedLightSource, transformedNormal.xyz);

    vLighting = ambientLight + (lightColor * max(illuminationIntensity, 0.0));
}
"#;

pub const TRIFORCE_FS_GLSL: &'static str = r#"

precision mediump float;
varying highp vec2 vTextureCoord;
varying highp vec3 vLighting;
uniform sampler2D uSampler;

void main()
{
    highp vec4 texelColor = texture2D(uSampler, vTextureCoord);
    gl_FragColor = vec4(texelColor.rgb * vLighting, texelColor.a);
}
"#;

