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





pub enum ArchetypeError{
    EntityNotFoundedError(EntityNotFounded),
    EntityAlreadyHaveComponentError(EntityAlreadyHaveComponent),
}


pub struct EntityNotFounded(u32);
pub struct EntityAlreadyHaveComponent(u32, String);

impl EntityNotFounded{
    pub fn new(entity_id: u32) -> Self{
        Self(entity_id)
    }
}

impl EntityAlreadyHaveComponent{
    pub fn new(entity_id: u32, component_name: String) -> Self{
        Self(entity_id, component_name)
    }
}

impl ArchetypeError{
    pub fn entity_not_founded(entity_id: u32) -> Self{
        Self::EntityNotFoundedError(EntityNotFounded::new(entity_id))
    }
}

impl Display for EntityNotFounded{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity {} not founded", self.0)
    }
}

impl Debug for EntityNotFounded{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity {} not founded", self.0)
    }
}

impl Display for EntityAlreadyHaveComponent{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity {} already have component {}", self.0, self.1)
    }
}

impl Debug for EntityAlreadyHaveComponent{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity {} already have component {}", self.0, self.1)
    }
}


impl Display for ArchetypeError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchetypeError::EntityNotFoundedError(e) => write!(f, "{}", e),
            ArchetypeError::EntityAlreadyHaveComponentError(e) => write!(f, "{}", e),
        }
    }
}

impl Debug for ArchetypeError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchetypeError::EntityNotFoundedError(e) => write!(f, "{:?}", e),
            ArchetypeError::EntityAlreadyHaveComponentError(e) => write!(f, "{:?}", e),
        }
    }
}


impl Error for ArchetypeError{}