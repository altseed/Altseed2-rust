#![feature(link_args)]
#![feature(type_name_of_val)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate downcast_rs;
extern crate num;

#[allow(unused_mut)]
mod auto_generated_core_binding;

mod runner;

/// エンジン
pub mod engine;

/// エラー
pub mod error;

mod array;
/// 数学系の機能
pub mod math;
pub mod node;
mod sound;
pub mod structs;

#[cfg(test)]
mod tests;

/// ドキュメント用のサンプル集です。
pub mod examples;

pub mod prelude {
    pub use crate::math::{Vector2, Vector3, Vector4};

    pub use crate::engine::{Engine, Loader};
    pub use crate::error::{AltseedError, AltseedResult};
    pub use crate::node::{
        BaseNode, CameraNode, Drawn, DrawnKind, DrawnNode, HasBaseNode, Node, NodeState, Polygon,
        Sprite, Text,
    };

    pub use crate::core::*;
    pub use crate::structs::{Color, Rect, Vertex};
}

/// AltseedのCoreとのバインディングです。
pub mod core {
    pub use crate::sound::{SoundID, SoundMixer};

    pub use crate::auto_generated_core_binding::{
        AsTexture2D, BuiltinShaderType, ButtonState, CursorMode, File, Font, FramerateMode,
        Joystick, JoystickAxisType, JoystickButtonType, JoystickType, Keyboard, Keys, Log,
        LogCategory, LogLevel, Material, Mouse, MouseButtons, RenderTexture, ResourceType, Shader,
        Sound, StaticFile, StreamFile, Texture2D, Tool, WritingDirection,
    };

    /// ツール機能で使用するフラグです。
    pub mod tool {
        pub use crate::auto_generated_core_binding::ToolColorEdit as ColorEdit;
        pub use crate::auto_generated_core_binding::ToolCond as Cond;
        pub use crate::auto_generated_core_binding::ToolDir as Dir;
        pub use crate::auto_generated_core_binding::ToolGlyphRanges as GlyphRanges;
        pub use crate::auto_generated_core_binding::ToolInputText as InputText;
        pub use crate::auto_generated_core_binding::ToolSelectable as Selectable;
        pub use crate::auto_generated_core_binding::ToolTabBar as TabBar;
        pub use crate::auto_generated_core_binding::ToolTreeNode as TreeNode;
        pub use crate::auto_generated_core_binding::ToolWindow as Window;
    }
}
