

//pub fn get_shader(display: &Display, textured: bool) -> Program {
//  match textured {
//    true  => Program::from_source(display, VERT_TMODEL, FRAG_TMODEL, None).unwrap(),
//    false => Program::from_source(display, VERT_CMODEL, FRAG_CMODEL, None).unwrap(),
//  }
//}

pub mod model_vert {
  #[derive(VulkanoShader)]
  #[ty = "vertex"]
  #[src = "
#version 450
layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"
  ]
  struct Dummy;
}

pub mod model_frag {
  #[derive(VulkanoShader)]
  #[ty = "fragment"]
  #[src = "
#version 450
layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"
  ]
  struct Dummy;
}


//pub mod c_model_vert {
//  #[derive(VulkanoShader)]
//  #[ty = "vertex"]
//  #[src = "
//#version 450
//in vec3 position;
//in vec3 normal;
//
//out vec3 surface_normal;
//out vec3 v_position;
//out vec3 toLightVector;
//out vec3 toCameraVector;
//
//uniform mat4 transform;
//uniform mat4 view;
//uniform mat4 projection;
//uniform vec3 u_light;
//
//void main() {
//  v_position = gl_Position.xyz / gl_Position.w;
//  vec4 worldPos = transform * vec4(position, 1.0);
//  vec4 posRelToCam = view * worldPos;
//  gl_Position = projection * posRelToCam;
//
//  surface_normal = (transform * vec4(normal, 0.0)).xyz;
//
//  toLightVector = u_light - worldPos.xyz;
//  toCameraVector = (inverse(view) * vec4(0.0,0.0,0.0,1.0)).xyz - worldPos.xyz;
//}
//"
//  ]
//  struct Dummy;
//}
//
//pub mod c_model_frag {
//  #[derive(VulkanoShader)]
//  #[ty = "fragment"]
//  #[src = "
//#version 450
//in vec3 surface_normal;
//in vec3 v_position;
//in vec3 toLightVector;
//in vec3 toCameraVector;
//
//out vec4 color;
//
//const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
//const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
//const vec3 specular_color = vec3(1.0, 1.0, 1.0);
//
//void main() {
//  vec3 lightColour = vec3(1.0);
//  vec3 unitNormal = normalize(surface_normal);
//  vec3 unitCameraVector = normalize(toCameraVector);
//  vec3 unitLightVector = normalize(toLightVector);
//
//  float diffuse = max(dot(unitNormal, unitLightVector), 0.0);
//  float specular = max(dot(reflect(unitLightVector, unitNormal), unitCameraVector), 0.0);
//
//  color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
//}
//"
//  ]
//  struct Dummy;
//}
