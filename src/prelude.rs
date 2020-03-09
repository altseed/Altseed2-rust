pub use crate::math::matrix::{Matrix33, Matrix44};
pub use crate::math::vector::{Vector2, Vector3, Vector4};

pub use crate::engine::Engine;

pub use crate::auto_generated_core_binding::{
    BuiltinShaderType, ButtonState, Configuration, CursorMode, File, Font, FramerateMode, Joystick,
    JoystickAxisType, JoystickButtonType, JoystickType, Keyboard, Keys, Log, LogCategory, LogLevel,
    Mouse, MouseButtons, Sound, SoundMixer, StaticFile, StreamFile, Texture2D, Tool,
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
