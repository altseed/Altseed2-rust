//! # Altseed2
//! [Altseed](https://github.com/altseed/Altseed) の後継となるべく開発しているゲームエンジンのRust版エンジンです。  
//!
//! - [Documents](documents/index.html): 各種機能の解説ページ
//! - [Example](examples/index.html): サンプルコード集
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
extern crate retain_mut;

pub(crate) mod private {
    pub trait Private {}
}

mod array;
#[allow(unused_mut)]
mod auto_generated_core_binding;
mod core_ext;
mod log;
mod sound;
pub mod task;

mod collections;
pub mod component;
/// 各種機能との仲介を行う
pub mod engine;
/// Altseedのエラー
pub mod error;
/// 数学系の機能
pub mod math;
/// Coreとの受け渡しに利用する構造体
pub mod structs;

#[cfg(test)]
mod tests;

/// ドキュメント
pub mod documents;
/// サンプルコード集
pub mod examples;

pub mod prelude {
    pub use crate::engine::{Config, Engine, Loader};
    pub use crate::error::{AltseedError, AltseedResult};
    pub use crate::math::{Easing, HasTransform, Transform, Vector2, Vector3, Vector4};
    pub use crate::task::Cont;

    pub use crate::core::*;
    pub use crate::structs::{Color, Rect, Vertex};

    pub use crate::component::{
        camera::{CameraComponent, CameraID, CameraStorage},
        drawn::{DrawnComponent, DrawnID, DrawnStorage},
        drawn_kind::{DrawnKind, Polygon, Sprite, Text},
    };
}

/// AltseedのCoreとのバインディングです。
pub mod core {
    pub use crate::log::Log;
    pub use crate::sound::{SoundID, SoundMixer};

    pub use crate::auto_generated_core_binding::{
        AsCollider, AsTextureBase, BuiltinShaderType, ButtonState, CircleCollider, Collider,
        Cursor, CursorMode, File, Font, FramerateMode, Joystick, JoystickAxisType,
        JoystickButtonType, JoystickType, Keyboard, Keys, LogCategory, LogLevel, Material, Mouse,
        MouseButtons, PolygonCollider, RectangleCollider, RenderTexture, ResourceType, Shader,
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
