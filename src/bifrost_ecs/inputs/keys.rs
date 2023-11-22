use std::collections::HashMap;

use super::InputHandler;

#[derive(Debug)]
pub struct Keys {
    pub keys: HashMap<glfw::Key, glfw::Action>,
}

impl Keys {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub(crate) fn set_key(&mut self, key: glfw::Key, action: glfw::Action) {
        self.keys.insert(key, action);
    }

    pub fn get_key(&self, key: glfw::Key) -> Option<&glfw::Action> {
        self.keys.get(&key)
    }
}

impl InputHandler for Keys {}