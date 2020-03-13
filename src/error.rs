use crate::prelude::*;

#[derive(Debug, Fail)]
pub enum AltseedError {
    #[fail(display = "Error in the Altseed Core: {}", 0)]
    CoreError(String),
    #[fail(display = "Failed to initialize the Engine")]
    InitializationFailed,
    #[fail(display = "File '{}' is not found", 0)]
    FileNotFound(String),
    #[fail(display = "Failed to create Texture2D from '{}'", 0)]
    FailedToCreateTexture2D(String),
    #[fail(display = "Failed to create DynamicFont from '{}'", 0)]
    FailedToCreateDynamicFont(String),
    #[fail(display = "Failed to create StaticFont from '{}'", 0)]
    FailedToCreateStaticFont(String),
    #[fail(display = "Failed to create Sound from '{}'", 0)]
    FailedToCreateSound(String),

    #[fail(display = "This Node({}) has invalid Node State '{:?}'", 0, 1)]
    InvalidNodeState(String, NodeState),

    #[fail(
        display = "This Node({}) tried to remove node({}) which is added to another node({})'",
        0, 1, 2
    )]
    RemovedInvalidNode(String, String, String),

    #[fail(display = "{}", 0)]
    Error(Box<dyn std::error::Error + Send + Sync + 'static>),
}

pub type AltseedResult<T> = Result<T, AltseedError>;
