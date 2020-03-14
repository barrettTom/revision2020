in vec2 v_uv;
out vec4 frag;

uniform sampler2D tex;
uniform vec2 position;
uniform float intensity;
uniform float time;

void main() {
  vec2 p =-1.+2.*v_uv / vec2(1,1)-position;
  float cLength=length(p);
  vec2 uv=v_uv+(p/cLength)*cos(cLength*32.0-time*4.0)*intensity;
  vec3 col=smoothstep(0.1,.91,texture(tex,uv).xyz);

 frag = texture(tex,uv);
}