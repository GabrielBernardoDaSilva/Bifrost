use std::sync::Arc;

use crate::bifrost_ecs::resources::{shader::Shader, texture::Texture};

use nalgebra_glm as glm;


#[derive(Debug)]
pub struct Material {
    pub color: glm::Vec4,
    pub texture: Arc<Texture>,
    pub shader: Arc<Shader>,
}

impl Material {
    pub fn use_material(&self, gl: Arc<glow::Context>) {
        self.shader.use_program(gl.clone());
        self.shader.set_vec4("color", &self.color, gl.clone());
        self.shader.set_i32("texture", 0, gl.clone());
        self.texture.bind(0, gl.clone());
    }
}
