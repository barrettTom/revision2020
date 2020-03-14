in vec3 pos;
in vec2 uv;
out vec2 v_uv;

uniform mat4 projection;
uniform mat4 view;

void main() {
  gl_Position =  projection * view * vec4(pos, 1.);
  v_uv = uv; 
}
