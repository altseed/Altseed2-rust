#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate num;

#[allow(unused_mut)]
mod auto_generated_core_binding;
pub mod engine;
pub mod error;
pub mod math;

pub mod structs;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::math::*;

    pub use crate::engine::Engine;
    pub use crate::error::{AltseedError, AltseedResult};

    pub use crate::core::*;
    pub use crate::structs::{rect::Rect, Color};
}

pub mod core {
    pub use crate::auto_generated_core_binding::{
        BuiltinShaderType, ButtonState, CursorMode, File, Font, FramerateMode, Joystick,
        JoystickAxisType, JoystickButtonType, JoystickType, Keyboard, Keys, Log, LogCategory,
        LogLevel, Mouse, MouseButtons, Sound, SoundMixer, StaticFile, StreamFile, Texture2D, Tool,
        WritingDirection,
    };

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
