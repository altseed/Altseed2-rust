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

/// Altseedが発生させるErrorを表します。
#[derive(Debug, Fail)]
pub enum AltseedError {
    /// AltseedのCore内部でエラーが発生しました。
    #[fail(display = "Error in the Altseed Core: {}", 0)]
    CoreError(String),
    /// Altseedの初期化に失敗しました。
    #[fail(display = "Failed to initialize the Engine")]
    InitializationFailed,
    /// エンジンが初期化済みです。
    #[fail(display = "The Engine has already been initialized")]
    AlreadyInitialized,
    /// リソースの作成に失敗しました。
    #[fail(display = "Failed to create {} from '{}'", 0, 1)]
    FailedToCreateResource(ResourceType, String),

    /// ノードの状態が無効です。
    #[fail(display = "{}, target nodehas invalid NodeState '{:?}' ({})", 0, 1, 2)]
    InvalidNodeState(String, NodeState, String),

    /// 音源の再生に失敗しました。
    #[fail(display = "Failed to play sound of '{}'", 0)]
    FailedToPlaySound(String),

    #[fail(display = "{}", 0)]
    Error(Box<dyn std::error::Error + Send + Sync + 'static>),

    #[fail(display = "{}", 0)]
    Msg(String),
}

impl AltseedError {
    pub fn msg(message: &str) -> Self {
        Self::Msg(message.to_owned())
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for AltseedError {
    fn from(item: T) -> Self {
        AltseedError::Error(Box::new(item))
    }
}

pub type AltseedResult<T> = Result<T, AltseedError>;
