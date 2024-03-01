// pipeline executor is a module that is used to execute the pipeline
// for example in your project could you have MenuPipelineExecutor and GamePipelineExecutor
// to be easy shared the pipeline executor is a struct that hold many scenes

use std::collections::HashMap;

use super::scene::Scene;

pub trait Pipeline {}

pub struct PipelineExecutor {
    scenes: HashMap<std::any::TypeId, Scene>,
}

impl PipelineExecutor {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
        }
    }

    pub fn add_scene<T: Pipeline + 'static>(&mut self, scene: Scene) {
        self.scenes.insert(std::any::TypeId::of::<T>(), scene);
    }

    pub fn get_scene<T: Pipeline + 'static>(&self) -> Option<&Scene> {
        self.scenes.get(&std::any::TypeId::of::<T>())
    }


    #[cfg(not(target_arch = "wasm32"))]
    pub fn execute<T: Pipeline + 'static>(&mut self) {
        if let Some(scene) = self.scenes.get_mut(&std::any::TypeId::of::<T>()) {
            // native version
            scene.run_forever()
        }
    }

   
}
