use crate::bifrost_ecs::core::scene::Scene;

use super::AssetLoader;


#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub tangents: Vec<f32>,
    pub bitangents: Vec<f32>,
}

impl AssetLoader for Model {
    type Asset = Self;
    fn load(
        _scene: &Scene,
        _path: &str,
    ) -> Result<std::sync::Arc<Self::Asset>, super::asset_loader_errors::AssetLoaderError> {
        let model = Model {
            vertices: vec![],
            indices: vec![],
            normals: vec![],
            uvs: vec![],
            tangents: vec![],
            bitangents: vec![],
        };
        Ok(std::sync::Arc::new(model))
    }
}
