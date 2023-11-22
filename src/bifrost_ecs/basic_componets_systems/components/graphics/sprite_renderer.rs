use super::{material::Material, quad::Quad};

use nalgebra_glm as glm;

#[derive(Debug)]
pub struct SpriteRenderer {
    pub quad: Quad,
    pub material: Material,
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    pub rotation: f32,
}

impl SpriteRenderer {
    pub fn new(
        quad: Quad,
        material: Material,
        position: glm::Vec2,
        size: glm::Vec2,
        rotation: f32,
    ) -> Self {
        Self {
            quad,
            material,
            position,
            rotation,
            size,
        }
    }
}
