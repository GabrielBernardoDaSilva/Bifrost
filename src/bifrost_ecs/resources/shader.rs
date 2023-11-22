use std::sync::Arc;

use glow::HasContext;

use crate::bifrost_ecs::core::scene::Scene;

use super::AssetLoader;
use nalgebra_glm as glm;

#[derive(Debug)]
pub struct Shader {
    pub id: glow::NativeProgram,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub geometry_shader: Option<String>,
    pub tessellation_control_shader: Option<String>,
    pub tessellation_evaluation_shader: Option<String>,
}

impl Shader {
    pub fn use_program(&self, gl: Arc<glow::Context>) {
        unsafe {
            gl.use_program(Some(self.id));
        };
    }

    pub fn get_location(
        &self,
        name: &str,
        gl: Arc<glow::Context>,
    ) -> Option<glow::NativeUniformLocation> {
        unsafe {
            let location: Option<glow::NativeUniformLocation> =
                gl.get_uniform_location(self.id, name);
            location
        }
    }

    pub fn set_vec2(&self, name: &str, vec2: &glm::Vec2, gl: Arc<glow::Context>) {
        unsafe {
            let location = self.get_location(name, gl.clone());
            if let Some(location) = location {
                gl.uniform_2_f32_slice(Some(&location), vec2.as_slice());
            }
        }
    }

    pub fn set_vec4(&self, name: &str, vec4: &glm::Vec4, gl: Arc<glow::Context>) {
        unsafe {
            let location = self.get_location(name, gl.clone());
            if let Some(location) = location {
                gl.uniform_4_f32_slice(Some(&location), vec4.as_slice());
            }
        }
    }

    pub fn set_mat4(&self,name: &str, mat4: &glm::Mat4, gl: Arc<glow::Context>) {
        unsafe {
            let location = self.get_location(name, gl.clone());
            if let Some(location) = location {
                gl.uniform_matrix_4_f32_slice(Some(&location), false, mat4.as_slice());
            }
        }
    }

    pub fn set_i32(&self, name: &str, value: i32, gl: Arc<glow::Context>) {
        unsafe {
            let location = self.get_location(name, gl.clone());
            if let Some(location) = location {
                gl.uniform_1_i32(Some(&location), value);
            }
        }
    }
}

impl AssetLoader for Shader {
    type Asset = Self;

    fn load(
        scene: &Scene,
        path: &str,
    ) -> Result<std::sync::Arc<Self::Asset>, super::asset_loader_errors::AssetLoaderError> {
        let gl = scene.window_container.gl.clone();
        let path = path.split(";").collect::<Vec<&str>>();
        let vertex_shader_str = std::fs::read_to_string(path[0]).expect("vertex shader not found");
        let fragment_shader_str =
            std::fs::read_to_string(path[1]).expect("fragment shader not found");
        let vertex_shader = unsafe {
            let shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(shader, &vertex_shader_str);
            gl.compile_shader(shader);
            // check for shader compile errors
            if !gl.get_shader_compile_status(shader) {
                let info = gl.get_shader_info_log(shader);
                panic!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", info);
            }
            shader
        };

        let fragment_shader = unsafe {
            let shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(shader, &fragment_shader_str);
            gl.compile_shader(shader);
            // check for shader compile errors
            if !gl.get_shader_compile_status(shader) {
                let info = gl.get_shader_info_log(shader);
                panic!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", info);
            }
            shader
        };

        let program = unsafe {
            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            // check for linking errors
            if !gl.get_program_link_status(program) {
                let info = gl.get_program_info_log(program);
                panic!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", info);
            }
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            program
        };

        let shader = Shader {
            id: program,
            vertex_shader: String::new(),
            fragment_shader: String::new(),
            geometry_shader: None,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
        };
        Ok(std::sync::Arc::new(shader))
    }
}
