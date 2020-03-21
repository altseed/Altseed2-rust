//! # Altseed2
//! [Altseed](https://github.com/altseed/Altseed) の後継となるべく開発しているゲームエンジンのRust版エンジンです。  
//!
//! - [Documents](documents/index.html)
//! - [Example](examples/index.html)
//!
//! ## Related Repogitories
//! - [Core](https://github.com/altseed/Altseed2): 各種機能
//! - [CppBindingGenerator](https://github.com/altseed/CppBindingGenerator): FFIコード生成
//! - [Engine(C#)](https://github.com/altseed/Altseed2-csharp): C#版Engine
//!

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

mod array;
#[allow(unused_mut)]
mod auto_generated_core_binding;
mod log;
mod runner;
mod sound;

/// 各種機能との仲介を行う
pub mod engine;
/// Altseedのエラー
pub mod error;
/// 数学系の機能
pub mod math;
pub mod node;
/// Coreとの受け渡しに利用する構造体
pub mod structs;

#[cfg(test)]
mod tests;

/// ドキュメント
pub mod documents;
/// サンプルコード集
pub mod examples;

pub mod prelude {
    //! re-export
    //! ```no_run
    //! use altseed2::*;
    //! use altseed2::prelude::*;
    //! ```

    pub use crate::math::{Easing, Vector2, Vector3, Vector4};

    pub use crate::engine::{Config, CoreContainer, Engine, Loader};
    pub use crate::error::{AltseedError, AltseedResult};
    pub use crate::node::{
        camera::CameraNode, drawn::Drawn, drawn::DrawnKind, drawn::DrawnNode, polygon::Polygon,
        sprite::Sprite, text::Text, BaseNode, HasBaseNode, Node, NodeState,
    };

    pub use crate::core::*;
    pub use crate::structs::{Color, Rect, Vertex};
}

/// AltseedのCoreとのバインディングです。
pub mod core {
    pub use crate::log::Log;
    pub use crate::sound::{SoundID, SoundMixer};

    pub use crate::auto_generated_core_binding::{
        AsTexture2D, BuiltinShaderType, ButtonState, CursorMode, File, Font, FramerateMode,
        Joystick, JoystickAxisType, JoystickButtonType, JoystickType, Keyboard, Keys, LogCategory,
        LogLevel, Material, Mouse, MouseButtons, RenderTexture, ResourceType, Shader, Sound,
        StaticFile, StreamFile, Texture2D, Tool, WritingDirection,
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
