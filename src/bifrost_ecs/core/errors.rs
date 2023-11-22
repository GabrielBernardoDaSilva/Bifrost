use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct ComponentNotFounded(String);
pub struct ComponentAlreadyExists(pub String);
pub struct ComponentAlreadyBorrowed(String);
pub struct ComponentUnableDowncast(String);

pub enum ComponentError {
    ComponentNotFoundedError(ComponentNotFounded),
    ComponentAlreadyExistsError(ComponentAlreadyExists),
    ComponentAlreadyBorrowedError(ComponentAlreadyBorrowed),
    ComponentUnableDowncastError(ComponentUnableDowncast),
}

impl ComponentAlreadyExists {
    pub fn new<T: 'static>() -> Self {
        Self(std::any::type_name::<T>().to_string())
    }
}

impl ComponentNotFounded {
    pub fn new<T: 'static>() -> Self {
        Self(std::any::type_name::<T>().to_string())
    }
}


impl ComponentAlreadyBorrowed {
    pub fn new<T: 'static>() -> Self {
        Self(std::any::type_name::<T>().to_string())
    }
}

impl ComponentUnableDowncast {
    pub fn new<T: 'static>() -> Self {
        Self(std::any::type_name::<T>().to_string())
    }
}

impl Display for ComponentNotFounded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component {} not founded", self.0)
    }
}

impl Display for ComponentAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component {} already exists", self.0)
    }
}

impl Display for ComponentAlreadyBorrowed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component {} already borrowed", self.0)
    }
}

impl Display for ComponentUnableDowncast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable downcast component {}", self.0)
    }
}

impl Debug for ComponentNotFounded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component {} not founded", self.0)
    }
}

impl Debug for ComponentAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component {} already exists", self.0)
    }
}


impl Debug for ComponentAlreadyBorrowed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component {} already borrowed", self.0)
    }
}

impl Debug for ComponentUnableDowncast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable downcast component {}", self.0)
    }
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentError::ComponentNotFoundedError(e) => write!(f, "{}", e),
            ComponentError::ComponentAlreadyExistsError(e) => write!(f, "{}", e),
            ComponentError::ComponentAlreadyBorrowedError(e) => write!(f, "{}", e),
            ComponentError::ComponentUnableDowncastError(e) => write!(f, "{}", e),
        }
    }
}

impl Debug for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentError::ComponentNotFoundedError(e) => write!(f, "{:?}", e),
            ComponentError::ComponentAlreadyExistsError(e) => write!(f, "{:?}", e),
            ComponentError::ComponentAlreadyBorrowedError(e) => write!(f, "{:?}", e),
            ComponentError::ComponentUnableDowncastError(e) => write!(f, "{:?}", e),
        }
    }
}

impl Error for ComponentAlreadyExists {}
impl Error for ComponentNotFounded {}
impl Error for ComponentAlreadyBorrowed {}
impl Error for ComponentUnableDowncast{}
