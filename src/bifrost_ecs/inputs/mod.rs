use std::ops::{DerefMut, Deref};

// pub mod keys;
// pub mod mouse;

pub trait InputHandler {   
}

#[derive(Debug)]
pub struct Input<T: InputHandler>{
    input: T,
}

impl<T: InputHandler> Input<T> {
    pub fn new(input: T) -> Self {
        Self {
            input,
        }
    }
}

impl<T: InputHandler> Deref for Input<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl<T: InputHandler> DerefMut for Input<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.input
    }
}