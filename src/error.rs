use crate::prelude::*;
use std::fmt;

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceType::StaticFile => write!(f, "StaticFile"),
            ResourceType::StreamFile => write!(f, "StreamFile"),
            ResourceType::Texture2D => write!(f, "Texture2D"),
            ResourceType::Font => write!(f, "Font"),
            ResourceType::Sound => write!(f, "Sound"),
            ResourceType::MAX => write!(f, "MAX"),
        }
    }
}

#[derive(Debug, Fail)]
pub enum AltseedError {
    #[fail(display = "Error in the Altseed Core: {}", 0)]
    CoreError(String),
    #[fail(display = "Failed to initialize the Engine")]
    InitializationFailed,
    #[fail(display = "The Engine has already been initialized")]
    AlreadyInitialized,
    #[fail(display = "Failed to create {} from '{}'", 0, 1)]
    FailedToCreateResource(ResourceType, String),

    #[fail(display = "This Node({}) has invalid Node State '{:?}'", 0, 1)]
    InvalidNodeState(String, NodeState),

    #[fail(
        display = "This Node({}) tried to remove node({}) which is added to another node({})'",
        0, 1, 2
    )]
    RemovedInvalidNode(String, String, String),

    #[fail(display = "Failed to play sound of '{}'", 0)]
    FailedToPlaySound(String),

    #[fail(display = "{}", 0)]
    Error(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for AltseedError {
    fn from(item: T) -> Self {
        AltseedError::Error(Box::new(item))
    }
}

pub type AltseedResult<T> = Result<T, AltseedError>;
