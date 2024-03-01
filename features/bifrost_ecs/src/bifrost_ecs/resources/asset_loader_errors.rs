use std::{fmt::{Display, Debug}, error::Error};

pub enum AssetLoaderError {
    AlreadyLoaded(String),
    NotFounded(String),
    NotLoaded(String),
}


impl AssetLoaderError {
    pub fn new_already_loaded(path: &str) -> Self {
        Self::AlreadyLoaded(path.to_string())
    }

    pub fn new_not_founded(path: &str) -> Self {
        Self::NotFounded(path.to_string())
    }

    pub fn new_not_loaded(path: &str) -> Self {
        Self::NotLoaded(path.to_string())
    }
}


impl Display for AssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetLoaderError::AlreadyLoaded(path) => {
                write!(f, "AssetLoaderError: {} is already loaded", path)
            }
            AssetLoaderError::NotFounded(path) => {
                write!(f, "AssetLoaderError: {} is not founded", path)
            }
            AssetLoaderError::NotLoaded(path) => {
                write!(f, "AssetLoaderError: {} is not loaded", path)
            }
        }
    }
}

impl Debug for AssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetLoaderError::AlreadyLoaded(path) => {
                write!(f, "AssetLoaderError: {} is already loaded", path)
            }
            AssetLoaderError::NotFounded(path) => {
                write!(f, "AssetLoaderError: {} is not founded", path)
            }
            AssetLoaderError::NotLoaded(path) => {
                write!(f, "AssetLoaderError: {} is not loaded", path)
            }
        }
    }
}


impl Error for AssetLoaderError {}