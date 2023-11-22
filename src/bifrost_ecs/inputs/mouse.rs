use std::collections::HashMap;

use super::InputHandler;

#[derive(Debug)]
pub struct Mouse {
    mouse_button: HashMap<glfw::MouseButton, glfw::Action>,
    mouse_pos: (f64, f64),
    scroll_offset: (f64, f64),
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            mouse_button: HashMap::new(),
            mouse_pos: (0.0, 0.0),
            scroll_offset: (0.0, 0.0),
        }
    }

    pub(crate) fn set_mouse_button(&mut self, button: glfw::MouseButton, action: glfw::Action) {
        self.mouse_button.insert(button, action);
    }

    pub(crate) fn set_mouse_pos(&mut self, x: f64, y: f64) {
        self.mouse_pos = (x, y);
    }

    pub(crate) fn set_scroll_offset(&mut self, x: f64, y: f64) {
        self.scroll_offset = (x, y);
    }

    pub fn get_mouse_button(&self, button: glfw::MouseButton) -> Option<&glfw::Action> {
        self.mouse_button.get(&button)
    }

    pub fn get_mouse_pos(&self) -> (f64, f64) {
        self.mouse_pos
    }

    pub fn get_scroll_offset(&self) -> (f64, f64) {
        self.scroll_offset
    }
}


impl InputHandler for Mouse {}