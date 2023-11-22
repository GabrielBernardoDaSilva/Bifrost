use std::{collections::HashMap, sync::Arc};

use self::asset_loader_errors::AssetLoaderError;

use super::core::scene::Scene;

pub mod asset_loader_errors;
pub mod model;
pub mod shader;
pub mod texture;
pub mod time;
pub mod text_renderer;
pub mod sound;

pub trait AssetLoader {
    type Asset;
    fn load(scene: &Scene, path: &str) -> Result<Arc<Self::Asset>, AssetLoaderError>;
}

#[derive(Debug)]
pub struct Asset<T: AssetLoader> {
    loaded_data: HashMap<String, Arc<T::Asset>>,
}
impl<T: AssetLoader> Asset<T> {
    pub fn new() -> Self {
        Self {
            loaded_data: HashMap::new(),
        }
    }

    pub fn load(&mut self, scene: &Scene, path: &str) -> Result<Arc<T::Asset>, AssetLoaderError> {
        if let Some(_) = self.loaded_data.get(path) {
            return Err(AssetLoaderError::new_already_loaded(path));
        }

        let data = T::load(scene, path)?;
        let filename = path.to_string().split("/").last().unwrap().to_string();
        // remove extension
        let filename = filename.split(".").next().unwrap().to_string();
        self.loaded_data.insert(filename.clone(), data);
        Ok(self.loaded_data.get(&filename).unwrap().clone())
    }

    pub fn get(&self, path: &str) -> Result<Arc<T::Asset>, AssetLoaderError> {
        if let Some(data) = self.loaded_data.get(path) {
            Ok(data.clone())
        } else {
            Err(AssetLoaderError::new_not_loaded(path))
        }
    }

    pub fn remove(&mut self, path: &str) -> Result<(), AssetLoaderError> {
        if let Some(_) = self.loaded_data.get(path) {
            self.loaded_data.remove(path);
            Ok(())
        } else {
            Err(AssetLoaderError::new_not_founded(path))
        }
    }
}
