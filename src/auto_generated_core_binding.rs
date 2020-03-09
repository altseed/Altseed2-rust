// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//
//   このファイルは自動生成されました。
//   このファイルへの変更は消失することがあります。
//
//   THIS FILE IS AUTO GENERATED.
//   YOUR COMMITMENT ON THIS FILE WILL BE WIPED.
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use std::ffi::c_void;
#[allow(unused_imports)]
use std::os::raw::*;

const NULLPTR: *mut RawPtr = 0 as *mut RawPtr;

#[allow(dead_code)]
fn decode_string(source: *const u16) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *source.offset(i) != 0).count();
        let slice = std::slice::from_raw_parts(source, len);
        String::from_utf16_lossy(slice)
    }
}

#[allow(dead_code)]
fn encode_string(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

enum RawPtr {}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FramerateMode {
    Variable,
    Constant,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    StaticFile,
    StreamFile,
    Texture2D,
    Font,
    Sound,
    MAX,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keys {
    Unknown,
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadDecimal,
    KeypadDivide,
    KeypadMultiply,
    KeypadSubstract,
    KeypadAdd,
    KeypadEnter,
    KeypadEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftWin,
    RightShift,
    RightControl,
    RightAlt,
    RightWin,
    Menu,
    Last,
    MAX,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Free = 0,
    Push = 1,
    Hold = 3,
    Release = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButtons {
    ButtonLeft = 0,
    ButtonRight = 1,
    ButtonMiddle = 2,
    SubButton1 = 3,
    SubButton2 = 4,
    SubButton3 = 5,
    SubButton4 = 6,
    SubButton5 = 7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorMode {
    Normal = 212993,
    Hidden = 212994,
    Disable = 212995,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoystickType {
    Other = 0,
    PS4 = 8200,
    XBOX360 = 8199,
    JoyconL = 8198,
    JoyconR = 8197,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoystickButtonType {
    Start,
    Select,
    Home,
    Release,
    Capture,
    LeftUp,
    LeftDown,
    LeftLeft,
    LeftRight,
    LeftPush,
    RightUp,
    RightRight,
    RightLeft,
    RightDown,
    RightPush,
    L1,
    R1,
    L2,
    R2,
    L3,
    R3,
    LeftStart,
    RightStart,
    Max,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoystickAxisType {
    Start,
    LeftH,
    LeftV,
    RightH,
    RightV,
    L2,
    R2,
    Max,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinShaderType {
    SpriteUnlitVS,
    SpriteUnlitPS,
    FontUnlitPS,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WritingDirection {
    Vertical,
    Horizontal,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolDir {
    None = -1,
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolCond {
    None = 0,
    Always = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolTreeNode {
    None = 0,
    Selected = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolInputText {
    None = 0,
    CharsDecimal = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolColorEdit {
    None = 0,
    NoAlpha = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolSelectable {
    None = 0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolWindow {
    None = 0,
    NoTitleBar = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolTabBar {
    None = 0,
    Reorderable = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolGlyphRanges {
    Default,
    Cyrillic,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FFTWindow {
    Rectangular,
    Triangle,
    Hamming,
    Hanning,
    Blackman,
    BlackmanHarris,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Critical = 5,
    Off = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogCategory {
    Core = 0,
    Engine = 1,
    User = 2,
}

#[allow(dead_code)]
#[link(name = "Altseed_Core")]
extern "C" {
    fn cbg_Configuration_Constructor_0() -> *mut RawPtr;

    fn cbg_Configuration_GetIsFullscreenMode(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetIsFullscreenMode(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetIsResizable(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetIsResizable(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetEnabledConsoleLogging(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetEnabledConsoleLogging(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetEnabledFileLogging(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Configuration_SetEnabledFileLogging(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Configuration_GetLogFilename(self_ptr: *mut RawPtr) -> *const u16;
    fn cbg_Configuration_SetLogFilename(self_ptr: *mut RawPtr, value: *const u16) -> ();

    fn cbg_Configuration_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Core_Initialize(
        title: *const u16,
        width: c_int,
        height: c_int,
        config: *mut RawPtr,
    ) -> bool;

    fn cbg_Core_DoEvent(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Core_Terminate() -> ();

    fn cbg_Core_GetInstance() -> *mut RawPtr;

    fn cbg_Core_GetDeltaSecond(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Core_GetCurrentFPS(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Core_GetTargetFPS(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_Core_SetTargetFPS(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_Core_GetFramerateMode(self_ptr: *mut RawPtr) -> c_int;
    fn cbg_Core_SetFramerateMode(self_ptr: *mut RawPtr, value: c_int) -> ();

    fn cbg_Core_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int8Array_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int8Array_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Int8Array_GetData(self_ptr: *mut RawPtr) -> c_void;

    fn cbg_Int8Array_Assign(self_ptr: *mut RawPtr, ptr: c_void, size: c_int) -> ();

    fn cbg_Int8Array_CopyTo(self_ptr: *mut RawPtr, ptr: c_void) -> ();

    fn cbg_Int8Array_GetAt(self_ptr: *mut RawPtr, index: c_int) -> c_uchar;

    fn cbg_Int8Array_SetAt(self_ptr: *mut RawPtr, index: c_int, value: c_uchar) -> ();

    fn cbg_Int8Array_Create(size: c_int) -> *mut RawPtr;

    fn cbg_Int8Array_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Int8Array_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int32Array_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Int32Array_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Int32Array_GetData(self_ptr: *mut RawPtr) -> c_void;

    fn cbg_Int32Array_Assign(self_ptr: *mut RawPtr, ptr: c_void, size: c_int) -> ();

    fn cbg_Int32Array_CopyTo(self_ptr: *mut RawPtr, ptr: c_void) -> ();

    fn cbg_Int32Array_GetAt(self_ptr: *mut RawPtr, index: c_int) -> c_int;

    fn cbg_Int32Array_SetAt(self_ptr: *mut RawPtr, index: c_int, value: c_int) -> ();

    fn cbg_Int32Array_Create(size: c_int) -> *mut RawPtr;

    fn cbg_Int32Array_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Int32Array_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_VertexArray_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_VertexArray_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_VertexArray_GetData(self_ptr: *mut RawPtr) -> c_void;

    fn cbg_VertexArray_Assign(self_ptr: *mut RawPtr, ptr: c_void, size: c_int) -> ();

    fn cbg_VertexArray_CopyTo(self_ptr: *mut RawPtr, ptr: c_void) -> ();

    fn cbg_VertexArray_GetAt(self_ptr: *mut RawPtr, index: c_int) -> crate::structs::Vertex;

    fn cbg_VertexArray_SetAt(
        self_ptr: *mut RawPtr,
        index: c_int,
        value: crate::structs::Vertex,
    ) -> ();

    fn cbg_VertexArray_Create(size: c_int) -> *mut RawPtr;

    fn cbg_VertexArray_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_VertexArray_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_FloatArray_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_FloatArray_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_FloatArray_GetData(self_ptr: *mut RawPtr) -> c_void;

    fn cbg_FloatArray_Assign(self_ptr: *mut RawPtr, ptr: c_void, size: c_int) -> ();

    fn cbg_FloatArray_CopyTo(self_ptr: *mut RawPtr, ptr: c_void) -> ();

    fn cbg_FloatArray_GetAt(self_ptr: *mut RawPtr, index: c_int) -> c_float;

    fn cbg_FloatArray_SetAt(self_ptr: *mut RawPtr, index: c_int, value: c_float) -> ();

    fn cbg_FloatArray_Create(size: c_int) -> *mut RawPtr;

    fn cbg_FloatArray_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_FloatArray_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Vector2FArray_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Vector2FArray_Resize(self_ptr: *mut RawPtr, size: c_int) -> ();

    fn cbg_Vector2FArray_GetData(self_ptr: *mut RawPtr) -> c_void;

    fn cbg_Vector2FArray_Assign(self_ptr: *mut RawPtr, ptr: c_void, size: c_int) -> ();

    fn cbg_Vector2FArray_CopyTo(self_ptr: *mut RawPtr, ptr: c_void) -> ();

    fn cbg_Vector2FArray_GetAt(self_ptr: *mut RawPtr, index: c_int) -> crate::structs::Vector2F;

    fn cbg_Vector2FArray_SetAt(
        self_ptr: *mut RawPtr,
        index: c_int,
        value: crate::structs::Vector2F,
    ) -> ();

    fn cbg_Vector2FArray_Create(size: c_int) -> *mut RawPtr;

    fn cbg_Vector2FArray_GetCount(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Vector2FArray_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Resources_GetInstance() -> *mut RawPtr;

    fn cbg_Resources_GetResourcesCount(self_ptr: *mut RawPtr, type_: c_int) -> c_int;

    fn cbg_Resources_Clear(self_ptr: *mut RawPtr) -> ();

    fn cbg_Resources_Reload(self_ptr: *mut RawPtr) -> ();

    fn cbg_Resources_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Keyboard_GetKeyState(self_ptr: *mut RawPtr, key: c_int) -> c_int;

    fn cbg_Keyboard_GetInstance() -> *mut RawPtr;

    fn cbg_Keyboard_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Mouse_GetInstance() -> *mut RawPtr;

    fn cbg_Mouse_GetWheel(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Mouse_GetMouseButtonState(self_ptr: *mut RawPtr, button: c_int) -> c_int;

    fn cbg_Mouse_GetPosition(self_ptr: *mut RawPtr) -> crate::structs::Vector2F;
    fn cbg_Mouse_SetPosition(self_ptr: *mut RawPtr, value: crate::structs::Vector2F) -> ();

    fn cbg_Mouse_GetCursorMode(self_ptr: *mut RawPtr) -> c_int;
    fn cbg_Mouse_SetCursorMode(self_ptr: *mut RawPtr, value: c_int) -> ();

    fn cbg_Mouse_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Joystick_GetInstance() -> *mut RawPtr;

    fn cbg_Joystick_IsPresent(self_ptr: *mut RawPtr, joystickIndex: c_int) -> bool;

    fn cbg_Joystick_RefreshConnectedState(self_ptr: *mut RawPtr) -> ();

    fn cbg_Joystick_GetButtonStateByIndex(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        buttonIndex: c_int,
    ) -> c_int;

    fn cbg_Joystick_GetButtonStateByType(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        type_: c_int,
    ) -> c_int;

    fn cbg_Joystick_GetJoystickType(self_ptr: *mut RawPtr, index: c_int) -> c_int;

    fn cbg_Joystick_GetAxisStateByIndex(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        axisIndex: c_int,
    ) -> c_float;

    fn cbg_Joystick_GetAxisStateByType(
        self_ptr: *mut RawPtr,
        joystickIndex: c_int,
        type_: c_int,
    ) -> c_float;

    fn cbg_Joystick_GetJoystickName(self_ptr: *mut RawPtr, index: c_int) -> *const u16;

    fn cbg_Joystick_Vibrate(
        self_ptr: *mut RawPtr,
        index: c_int,
        frequency: c_float,
        amplitude: c_float,
    ) -> ();

    fn cbg_Joystick_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Graphics_GetInstance() -> *mut RawPtr;

    fn cbg_Graphics_BeginFrame(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Graphics_EndFrame(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Graphics_DoEvents(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Graphics_GetCommandList(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Graphics_GetBuiltinShader(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Graphics_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Texture2D_Load(path: *const u16) -> *mut RawPtr;

    fn cbg_Texture2D_Reload(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Texture2D_GetSize(self_ptr: *mut RawPtr) -> crate::structs::Vector2I;

    fn cbg_Texture2D_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Material_Constructor_0() -> *mut RawPtr;

    fn cbg_Material_GetVector4F(self_ptr: *mut RawPtr, key: *const u16)
        -> crate::structs::Vector4F;

    fn cbg_Material_SetVector4F(
        self_ptr: *mut RawPtr,
        key: *const u16,
        value: crate::structs::Vector4F,
    ) -> ();

    fn cbg_Material_GetMatrix44F(
        self_ptr: *mut RawPtr,
        key: *const u16,
    ) -> crate::structs::Matrix44F;

    fn cbg_Material_SetMatrix44F(
        self_ptr: *mut RawPtr,
        key: *const u16,
        value: crate::structs::Matrix44F,
    ) -> ();

    fn cbg_Material_GetTexture(self_ptr: *mut RawPtr, key: *const u16) -> *mut RawPtr;

    fn cbg_Material_SetTexture(self_ptr: *mut RawPtr, key: *const u16, value: *mut RawPtr) -> ();

    fn cbg_Material_GetShader(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_Material_SetShader(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_Material_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Renderer_GetInstance() -> *mut RawPtr;

    fn cbg_Renderer_DrawSprite(self_ptr: *mut RawPtr, sprite: *mut RawPtr) -> ();

    fn cbg_Renderer_DrawText(self_ptr: *mut RawPtr, text: *mut RawPtr) -> ();

    fn cbg_Renderer_Render(self_ptr: *mut RawPtr, commandList: *mut RawPtr) -> ();

    fn cbg_Renderer_DrawPolygon(
        self_ptr: *mut RawPtr,
        vertexBuffer: *mut RawPtr,
        indexBuffer: *mut RawPtr,
        texture: *mut RawPtr,
        material: *mut RawPtr,
    ) -> ();

    fn cbg_Renderer_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_CommandList_SetRenderTargetWithScreen(self_ptr: *mut RawPtr) -> ();

    fn cbg_CommandList_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Rendered_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedSprite_Create() -> *mut RawPtr;

    fn cbg_RenderedSprite_GetTexture(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedSprite_SetTexture(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedSprite_GetSrc(self_ptr: *mut RawPtr) -> crate::structs::RectF;
    fn cbg_RenderedSprite_SetSrc(self_ptr: *mut RawPtr, value: crate::structs::RectF) -> ();

    fn cbg_RenderedSprite_GetTransform(self_ptr: *mut RawPtr) -> crate::structs::Matrix44F;
    fn cbg_RenderedSprite_SetTransform(
        self_ptr: *mut RawPtr,
        value: crate::structs::Matrix44F,
    ) -> ();

    fn cbg_RenderedSprite_GetMaterial(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedSprite_SetMaterial(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedSprite_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedText_Create() -> *mut RawPtr;

    fn cbg_RenderedText_GetTransform(self_ptr: *mut RawPtr) -> crate::structs::Matrix44F;
    fn cbg_RenderedText_SetTransform(self_ptr: *mut RawPtr, value: crate::structs::Matrix44F)
        -> ();

    fn cbg_RenderedText_GetMaterial(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedText_SetMaterial(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedText_GetText(self_ptr: *mut RawPtr) -> *const u16;
    fn cbg_RenderedText_SetText(self_ptr: *mut RawPtr, value: *const u16) -> ();

    fn cbg_RenderedText_GetFont(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedText_SetFont(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedText_GetWeight(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_RenderedText_SetWeight(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_RenderedText_GetColor(self_ptr: *mut RawPtr) -> crate::structs::Color;
    fn cbg_RenderedText_SetColor(self_ptr: *mut RawPtr, value: crate::structs::Color) -> ();

    fn cbg_RenderedText_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_Create() -> *mut RawPtr;

    fn cbg_RenderedPolygon_GetVertexes(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_RenderedPolygon_SetVertexes(self_ptr: *mut RawPtr, vertexes: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_SetVertexesByVector2F(
        self_ptr: *mut RawPtr,
        vertexes: *mut RawPtr,
    ) -> ();

    fn cbg_RenderedPolygon_GetTexture(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedPolygon_SetTexture(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_GetSrc(self_ptr: *mut RawPtr) -> crate::structs::RectF;
    fn cbg_RenderedPolygon_SetSrc(self_ptr: *mut RawPtr, value: crate::structs::RectF) -> ();

    fn cbg_RenderedPolygon_GetTransform(self_ptr: *mut RawPtr) -> crate::structs::Matrix44F;
    fn cbg_RenderedPolygon_SetTransform(
        self_ptr: *mut RawPtr,
        value: crate::structs::Matrix44F,
    ) -> ();

    fn cbg_RenderedPolygon_GetMaterial(self_ptr: *mut RawPtr) -> *mut RawPtr;
    fn cbg_RenderedPolygon_SetMaterial(self_ptr: *mut RawPtr, value: *mut RawPtr) -> ();

    fn cbg_RenderedPolygon_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_RenderedCamera_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_BuiltinShader_Create(self_ptr: *mut RawPtr, type_: c_int) -> *mut RawPtr;

    fn cbg_BuiltinShader_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Shader_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Glyph_GetTextureSize(self_ptr: *mut RawPtr) -> crate::structs::Vector2I;

    fn cbg_Glyph_GetTextureIndex(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Glyph_GetPosition(self_ptr: *mut RawPtr) -> crate::structs::Vector2I;

    fn cbg_Glyph_GetSize(self_ptr: *mut RawPtr) -> crate::structs::Vector2I;

    fn cbg_Glyph_GetOffset(self_ptr: *mut RawPtr) -> crate::structs::Vector2I;

    fn cbg_Glyph_GetGlyphWidth(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Glyph_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Font_LoadDynamicFont(path: *const u16, size: c_int) -> *mut RawPtr;

    fn cbg_Font_LoadStaticFont(path: *const u16) -> *mut RawPtr;

    fn cbg_Font_GetGlyph(self_ptr: *mut RawPtr, character: c_int) -> *mut RawPtr;

    fn cbg_Font_GetFontTexture(self_ptr: *mut RawPtr, index: c_int) -> *mut RawPtr;

    fn cbg_Font_GetKerning(self_ptr: *mut RawPtr, c1: c_int, c2: c_int) -> c_int;

    fn cbg_Font_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_Font_CalcTextureSize(
        self_ptr: *mut RawPtr,
        text: *const u16,
        direction: c_int,
        isEnableKerning: bool,
    ) -> crate::structs::Vector2I;

    fn cbg_Font_CreateImageFont(baseFont: *mut RawPtr) -> *mut RawPtr;

    fn cbg_Font_AddImageGlyph(self_ptr: *mut RawPtr, character: c_int, texture: *mut RawPtr) -> ();

    fn cbg_Font_GetImageGlyph(self_ptr: *mut RawPtr, character: c_int) -> *mut RawPtr;

    fn cbg_Font_GetSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_GetAscent(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_GetDescent(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_GetLineGap(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_Font_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_GetInstance() -> *mut RawPtr;

    fn cbg_Tool_Begin(self_ptr: *mut RawPtr, name: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_End(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Dummy(self_ptr: *mut RawPtr, size: crate::structs::Vector2F) -> ();

    fn cbg_Tool_Text(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_TextUnformatted(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_TextWrapped(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_TextColored(
        self_ptr: *mut RawPtr,
        color: crate::structs::Vector4F,
        text: *const u16,
    ) -> ();

    fn cbg_Tool_TextDisabled(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_BulletText(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_LabelText(self_ptr: *mut RawPtr, label: *const u16, text: *const u16) -> ();

    fn cbg_Tool_CollapsingHeader(self_ptr: *mut RawPtr, label: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_TreeNode(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_TreeNodeEx(self_ptr: *mut RawPtr, label: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_TreePop(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetNextItemOpen(self_ptr: *mut RawPtr, isOpen: bool, cond: c_int) -> ();

    fn cbg_Tool_Button(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::structs::Vector2F,
    ) -> bool;

    fn cbg_Tool_CheckBox(self_ptr: *mut RawPtr, label: *const u16, isOpen: *mut bool) -> bool;

    fn cbg_Tool_RadioButton(self_ptr: *mut RawPtr, label: *const u16, active: bool) -> bool;

    fn cbg_Tool_ArrowButton(self_ptr: *mut RawPtr, label: *const u16, dir: c_int) -> bool;

    fn cbg_Tool_InvisibleButton(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::structs::Vector2F,
    ) -> bool;

    fn cbg_Tool_Selectable(
        self_ptr: *mut RawPtr,
        label: *const u16,
        selected: *const bool,
        flags: c_int,
    ) -> bool;

    fn cbg_Tool_InputInt(self_ptr: *mut RawPtr, label: *const u16, value: *const c_int) -> bool;

    fn cbg_Tool_InputFloat(self_ptr: *mut RawPtr, label: *const u16, value: *const c_float)
        -> bool;

    fn cbg_Tool_SliderInt(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_int,
        speed: c_float,
        valueMin: c_int,
        valueMax: c_int,
    ) -> bool;

    fn cbg_Tool_SliderFloat(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_float,
        speed: c_float,
        valueMin: c_float,
        valueMax: c_float,
    ) -> bool;

    fn cbg_Tool_SliderAngle(
        self_ptr: *mut RawPtr,
        label: *const u16,
        angle: *const c_float,
    ) -> bool;

    fn cbg_Tool_VSliderInt(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::structs::Vector2F,
        value: *const c_int,
        valueMin: c_int,
        valueMax: c_int,
    ) -> bool;

    fn cbg_Tool_VSliderFloat(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::structs::Vector2F,
        value: *const c_float,
        valueMin: c_float,
        valueMax: c_float,
    ) -> bool;

    fn cbg_Tool_DragInt(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_int,
        speed: c_float,
        valueMin: c_int,
        valueMax: c_int,
    ) -> bool;

    fn cbg_Tool_DragFloat(
        self_ptr: *mut RawPtr,
        label: *const u16,
        value: *const c_float,
        speed: c_float,
        valueMin: c_float,
        valueMax: c_float,
    ) -> bool;

    fn cbg_Tool_OpenPopup(self_ptr: *mut RawPtr, label: *const u16) -> ();

    fn cbg_Tool_BeginPopup(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_BeginPopupModal(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_EndPopup(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginChild(
        self_ptr: *mut RawPtr,
        label: *const u16,
        size: crate::structs::Vector2F,
        border: bool,
        flags: c_int,
    ) -> bool;

    fn cbg_Tool_EndChild(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginMenuBar(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_EndMenuBar(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginMenu(self_ptr: *mut RawPtr, label: *const u16, enabled: bool) -> bool;

    fn cbg_Tool_EndMenu(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_MenuItem(
        self_ptr: *mut RawPtr,
        label: *const u16,
        shortcut: *const u16,
        selected: bool,
        enabled: bool,
    ) -> bool;

    fn cbg_Tool_BeginTabBar(self_ptr: *mut RawPtr, label: *const u16, flags: c_int) -> bool;

    fn cbg_Tool_EndTabBar(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_BeginTabItem(self_ptr: *mut RawPtr, label: *const u16) -> bool;

    fn cbg_Tool_EndTabItem(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Indent(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Unindent(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Separator(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetTooltip(self_ptr: *mut RawPtr, text: *const u16) -> ();

    fn cbg_Tool_BeginTooltip(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_EndTooltip(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_NewLine(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SameLine(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_PushTextWrapPos(self_ptr: *mut RawPtr, wrapLocalPosX: c_float) -> ();

    fn cbg_Tool_PopTextWrapPos(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetNextItemWidth(self_ptr: *mut RawPtr, width: c_float) -> ();

    fn cbg_Tool_PushItemWidth(self_ptr: *mut RawPtr, width: c_float) -> ();

    fn cbg_Tool_PopItemWidth(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_PushButtonRepeat(self_ptr: *mut RawPtr, repeat: bool) -> ();

    fn cbg_Tool_PopButtonRepeat(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_Columns(self_ptr: *mut RawPtr, count: c_int, border: bool) -> ();

    fn cbg_Tool_NextColumn(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_PushID(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_Tool_PopID(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_IsItemActive(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_IsItemHovered(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_SetScrollHere(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetScrollHereX(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_SetScrollHereY(self_ptr: *mut RawPtr) -> ();

    fn cbg_Tool_GetTextLineHeight(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Tool_GetFontSize(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Tool_GetWindowSize(self_ptr: *mut RawPtr) -> crate::structs::Vector2F;

    fn cbg_Tool_SetWindowSize(self_ptr: *mut RawPtr, size: crate::structs::Vector2F) -> ();

    fn cbg_Tool_IsMousePosValid(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_IsMouseDragging(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Tool_IsMouseDoubleClicked(self_ptr: *mut RawPtr, button: c_int) -> bool;

    fn cbg_Tool_GetMouseDragDelta(self_ptr: *mut RawPtr, button: c_int)
        -> crate::structs::Vector2F;

    fn cbg_Tool_ResetMouseDragDelta(self_ptr: *mut RawPtr, button: c_int) -> ();

    fn cbg_Tool_SetNextWindowContentSize(
        self_ptr: *mut RawPtr,
        size: crate::structs::Vector2F,
    ) -> ();

    fn cbg_Tool_SetNextWindowSize(self_ptr: *mut RawPtr, size: crate::structs::Vector2F) -> ();

    fn cbg_Tool_SetNextWindowPos(self_ptr: *mut RawPtr, pos: crate::structs::Vector2F) -> ();

    fn cbg_Tool_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_StreamFile_Create(path: *const u16) -> *mut RawPtr;

    fn cbg_StreamFile_Read(self_ptr: *mut RawPtr, size: c_int) -> c_int;

    fn cbg_StreamFile_GetTempBuffer(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_StreamFile_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_StreamFile_Reload(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StreamFile_GetSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StreamFile_GetCurrentPosition(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StreamFile_GetTempBufferSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StreamFile_GetIsInPackage(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StreamFile_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_StaticFile_Create(path: *const u16) -> *mut RawPtr;

    fn cbg_StaticFile_GetBuffer(self_ptr: *mut RawPtr) -> *mut RawPtr;

    fn cbg_StaticFile_Reload(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StaticFile_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_StaticFile_GetSize(self_ptr: *mut RawPtr) -> c_int;

    fn cbg_StaticFile_GetIsInPackage(self_ptr: *mut RawPtr) -> bool;

    fn cbg_StaticFile_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_File_GetInstance() -> *mut RawPtr;

    fn cbg_File_AddRootDirectory(self_ptr: *mut RawPtr, path: *const u16) -> bool;

    fn cbg_File_AddRootPackageWithPassword(
        self_ptr: *mut RawPtr,
        path: *const u16,
        password: *const u16,
    ) -> bool;

    fn cbg_File_AddRootPackage(self_ptr: *mut RawPtr, path: *const u16) -> bool;

    fn cbg_File_ClearRootDirectories(self_ptr: *mut RawPtr) -> ();

    fn cbg_File_Exists(self_ptr: *mut RawPtr, path: *const u16) -> bool;

    fn cbg_File_Pack(self_ptr: *mut RawPtr, srcPath: *const u16, dstPath: *const u16) -> bool;

    fn cbg_File_PackWithPassword(
        self_ptr: *mut RawPtr,
        srcPath: *const u16,
        dstPath: *const u16,
        password: *const u16,
    ) -> bool;

    fn cbg_File_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Sound_Load(path: *const u16, isDecompressed: bool) -> *mut RawPtr;

    fn cbg_Sound_GetPath(self_ptr: *mut RawPtr) -> *const u16;

    fn cbg_Sound_GetIsDecompressed(self_ptr: *mut RawPtr) -> bool;

    fn cbg_Sound_GetLoopStartingPoint(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_Sound_SetLoopStartingPoint(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_Sound_GetLoopEndPoint(self_ptr: *mut RawPtr) -> c_float;
    fn cbg_Sound_SetLoopEndPoint(self_ptr: *mut RawPtr, value: c_float) -> ();

    fn cbg_Sound_GetIsLoopingMode(self_ptr: *mut RawPtr) -> bool;
    fn cbg_Sound_SetIsLoopingMode(self_ptr: *mut RawPtr, value: bool) -> ();

    fn cbg_Sound_GetLength(self_ptr: *mut RawPtr) -> c_float;

    fn cbg_Sound_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_SoundMixer_GetInstance() -> *mut RawPtr;

    fn cbg_SoundMixer_Play(self_ptr: *mut RawPtr, sound: *mut RawPtr) -> c_int;

    fn cbg_SoundMixer_GetIsPlaying(self_ptr: *mut RawPtr, id: c_int) -> bool;

    fn cbg_SoundMixer_StopAll(self_ptr: *mut RawPtr) -> ();

    fn cbg_SoundMixer_Stop(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_SoundMixer_Pause(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_SoundMixer_Resume(self_ptr: *mut RawPtr, id: c_int) -> ();

    fn cbg_SoundMixer_SetVolume(self_ptr: *mut RawPtr, id: c_int, volume: c_float) -> ();

    fn cbg_SoundMixer_FadeIn(self_ptr: *mut RawPtr, id: c_int, second: c_float) -> ();

    fn cbg_SoundMixer_FadeOut(self_ptr: *mut RawPtr, id: c_int, second: c_float) -> ();

    fn cbg_SoundMixer_Fade(
        self_ptr: *mut RawPtr,
        id: c_int,
        second: c_float,
        targetedVolume: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetIsPlaybackSpeedEnabled(self_ptr: *mut RawPtr, id: c_int) -> bool;

    fn cbg_SoundMixer_SetIsPlaybackSpeedEnabled(
        self_ptr: *mut RawPtr,
        id: c_int,
        isPlaybackSpeedEnabled: bool,
    ) -> ();

    fn cbg_SoundMixer_GetPlaybackSpeed(self_ptr: *mut RawPtr, id: c_int) -> c_float;

    fn cbg_SoundMixer_SetPlaybackSpeed(
        self_ptr: *mut RawPtr,
        id: c_int,
        playbackSpeed: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetPanningPosition(self_ptr: *mut RawPtr, id: c_int) -> c_float;

    fn cbg_SoundMixer_SetPanningPosition(
        self_ptr: *mut RawPtr,
        id: c_int,
        panningPosition: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetPlaybackPosition(self_ptr: *mut RawPtr, id: c_int) -> c_float;

    fn cbg_SoundMixer_SetPlaybackPosition(
        self_ptr: *mut RawPtr,
        id: c_int,
        position: c_float,
    ) -> ();

    fn cbg_SoundMixer_GetSpectrumData(
        self_ptr: *mut RawPtr,
        id: c_int,
        spectrums: *mut RawPtr,
        window: c_int,
    ) -> ();

    fn cbg_SoundMixer_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Log_GetInstance() -> *mut RawPtr;

    fn cbg_Log_Write(
        self_ptr: *mut RawPtr,
        category: c_int,
        level: c_int,
        message: *const u16,
    ) -> ();

    fn cbg_Log_Trace(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Debug(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Info(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Warn(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Error(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_Critical(self_ptr: *mut RawPtr, category: c_int, message: *const u16) -> ();

    fn cbg_Log_SetLevel(self_ptr: *mut RawPtr, category: c_int, level: c_int) -> ();

    fn cbg_Log_Release(self_ptr: *mut RawPtr) -> ();

    fn cbg_Window_GetInstance() -> *mut RawPtr;

    fn cbg_Window_GetTitle(self_ptr: *mut RawPtr) -> *const u16;
    fn cbg_Window_SetTitle(self_ptr: *mut RawPtr, value: *const u16) -> ();

    fn cbg_Window_Release(self_ptr: *mut RawPtr) -> ();

}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{self, Rc};
use std::sync::{self, Arc, Mutex, RwLock};

#[derive(Debug, PartialEq, Eq, Hash)]
struct RawPtrStorage(*mut RawPtr);

unsafe impl Send for RawPtrStorage {}
unsafe impl Sync for RawPtrStorage {}

#[derive(Debug)]
pub struct Configuration {
    self_ptr: *mut RawPtr,
    is_fullscreen_mode: Option<bool>,
    is_resizable: Option<bool>,
    enabled_console_logging: Option<bool>,
    enabled_file_logging: Option<bool>,
    log_filename: Option<String>,
}

impl Configuration {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Configuration>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Configuration {
            self_ptr,
            is_fullscreen_mode: None,
            is_resizable: None,
            enabled_console_logging: None,
            enabled_file_logging: None,
            log_filename: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static CONFIGURATION_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Configuration>>>> = RefCell::new(HashMap::new());
        }
        CONFIGURATION_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    pub fn new() -> Option<Rc<RefCell<Configuration>>> {
        Self::cbg_create_raw(unsafe { cbg_Configuration_Constructor_0() })
    }

    /// 全画面モードかどうかを取得または設定します。
    pub fn get_is_fullscreen_mode(&mut self) -> bool {
        if let Some(value) = self.is_fullscreen_mode.clone() {
            return value;
        }
        let ret = unsafe { cbg_Configuration_GetIsFullscreenMode(self.self_ptr) };
        ret
    }
    pub fn set_is_fullscreen_mode(&mut self, value: bool) {
        self.is_fullscreen_mode = Some(value.clone());
        unsafe { cbg_Configuration_SetIsFullscreenMode(self.self_ptr, value) }
    }

    /// 画面サイズ可変かどうかを取得または設定します。
    pub fn get_is_resizable(&mut self) -> bool {
        if let Some(value) = self.is_resizable.clone() {
            return value;
        }
        let ret = unsafe { cbg_Configuration_GetIsResizable(self.self_ptr) };
        ret
    }
    pub fn set_is_resizable(&mut self, value: bool) {
        self.is_resizable = Some(value.clone());
        unsafe { cbg_Configuration_SetIsResizable(self.self_ptr, value) }
    }

    /// ログをコンソールに出力するかどうかを取得または設定します。
    pub fn get_enabled_console_logging(&mut self) -> bool {
        if let Some(value) = self.enabled_console_logging.clone() {
            return value;
        }
        let ret = unsafe { cbg_Configuration_GetEnabledConsoleLogging(self.self_ptr) };
        ret
    }
    pub fn set_enabled_console_logging(&mut self, value: bool) {
        self.enabled_console_logging = Some(value.clone());
        unsafe { cbg_Configuration_SetEnabledConsoleLogging(self.self_ptr, value) }
    }

    /// ログをファイルに出力するかどうかを取得または設定します。
    pub fn get_enabled_file_logging(&mut self) -> bool {
        if let Some(value) = self.enabled_file_logging.clone() {
            return value;
        }
        let ret = unsafe { cbg_Configuration_GetEnabledFileLogging(self.self_ptr) };
        ret
    }
    pub fn set_enabled_file_logging(&mut self, value: bool) {
        self.enabled_file_logging = Some(value.clone());
        unsafe { cbg_Configuration_SetEnabledFileLogging(self.self_ptr, value) }
    }

    /// ログファイル名を取得または設定します。
    pub fn get_log_filename(&mut self) -> String {
        if let Some(value) = self.log_filename.clone() {
            return value;
        }
        let ret = unsafe { cbg_Configuration_GetLogFilename(self.self_ptr) };
        decode_string(ret)
    }
    pub fn set_log_filename(&mut self, value: String) {
        self.log_filename = Some(value.clone());
        unsafe { cbg_Configuration_SetLogFilename(self.self_ptr, encode_string(&value).as_ptr()) }
    }
}

impl Drop for Configuration {
    fn drop(&mut self) {
        unsafe { cbg_Configuration_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct Core {
    self_ptr: *mut RawPtr,
    target_fps: Option<f32>,
    framerate_mode: Option<FramerateMode>,
}

impl Core {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Core>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Core {
            self_ptr,
            target_fps: None,
            framerate_mode: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static CORE_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Core>>>> = RefCell::new(HashMap::new());
        }
        CORE_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// 初期化処理を行います。
    /// # Arguments
    /// * `title` - ウィンドウの左上に表示されるウィンドウん名
    /// * `width` - ウィンドウの横幅
    /// * `height` - ウィンドウの縦幅
    /// * `config` - 初期化時の設定
    pub fn initialize(title: &str, width: i32, height: i32, config: &mut Configuration) -> bool {
        let ret = unsafe {
            cbg_Core_Initialize(
                encode_string(&title).as_ptr(),
                width,
                height,
                config.self_ptr,
            )
        };
        ret
    }

    /// イベントを実行します。
    pub fn do_event(&mut self) -> bool {
        let ret = unsafe { cbg_Core_DoEvent(self.self_ptr) };
        ret
    }

    /// 終了処理を行います。
    pub fn terminate() -> () {
        unsafe { cbg_Core_Terminate() }
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Core>>> {
        let ret = unsafe { cbg_Core_GetInstance() };
        Core::try_get_from_cache(ret)
    }

    /// 前のフレームからの経過時間(秒)を取得します。
    pub fn get_delta_second(&mut self) -> f32 {
        let ret = unsafe { cbg_Core_GetDeltaSecond(self.self_ptr) };
        ret
    }

    /// 現在のFPSを取得します。
    pub fn get_current_fps(&mut self) -> f32 {
        let ret = unsafe { cbg_Core_GetCurrentFPS(self.self_ptr) };
        ret
    }

    /// 目標のFPSを取得または設定します。
    pub fn get_target_fps(&mut self) -> f32 {
        if let Some(value) = self.target_fps.clone() {
            return value;
        }
        let ret = unsafe { cbg_Core_GetTargetFPS(self.self_ptr) };
        ret
    }
    pub fn set_target_fps(&mut self, value: f32) {
        self.target_fps = Some(value.clone());
        unsafe { cbg_Core_SetTargetFPS(self.self_ptr, value) }
    }

    /// フレームレートモードを取得または設定します。デフォルトでは可変フレームレートです。
    pub fn get_framerate_mode(&mut self) -> FramerateMode {
        if let Some(value) = self.framerate_mode.clone() {
            return value;
        }
        let ret = unsafe { cbg_Core_GetFramerateMode(self.self_ptr) };
        unsafe { std::mem::transmute(ret) }
    }
    pub fn set_framerate_mode(&mut self, value: FramerateMode) {
        self.framerate_mode = Some(value.clone());
        unsafe { cbg_Core_SetFramerateMode(self.self_ptr, value as i32) }
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe { cbg_Core_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Int8Array {
    self_ptr: *mut RawPtr,
}

impl Int8Array {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Int8Array>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Int8Array { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static INT8ARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Int8Array>>>> = RefCell::new(HashMap::new());
        }
        INT8ARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。
    pub fn clear(&mut self) -> () {
        unsafe { cbg_Int8Array_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_Int8Array_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> c_void {
        let ret = unsafe { cbg_Int8Array_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: c_void, size: i32) -> () {
        unsafe { cbg_Int8Array_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ
    pub fn copy_to(&mut self, ptr: c_void) -> () {
        unsafe { cbg_Int8Array_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    pub fn get_at(&mut self, index: i32) -> u8 {
        let ret = unsafe { cbg_Int8Array_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値
    pub fn set_at(&mut self, index: i32, value: u8) -> () {
        unsafe { cbg_Int8Array_SetAt(self.self_ptr, index, value) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn create(size: i32) -> Option<Rc<RefCell<Int8Array>>> {
        let ret = unsafe { cbg_Int8Array_Create(size) };
        Int8Array::try_get_from_cache(ret)
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_Int8Array_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for Int8Array {
    fn drop(&mut self) {
        unsafe { cbg_Int8Array_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Int32Array {
    self_ptr: *mut RawPtr,
}

impl Int32Array {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Int32Array>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Int32Array { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static INT32ARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Int32Array>>>> = RefCell::new(HashMap::new());
        }
        INT32ARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。
    pub fn clear(&mut self) -> () {
        unsafe { cbg_Int32Array_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_Int32Array_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> c_void {
        let ret = unsafe { cbg_Int32Array_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: c_void, size: i32) -> () {
        unsafe { cbg_Int32Array_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ
    pub fn copy_to(&mut self, ptr: c_void) -> () {
        unsafe { cbg_Int32Array_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    pub fn get_at(&mut self, index: i32) -> i32 {
        let ret = unsafe { cbg_Int32Array_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値
    pub fn set_at(&mut self, index: i32, value: i32) -> () {
        unsafe { cbg_Int32Array_SetAt(self.self_ptr, index, value) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn create(size: i32) -> Option<Rc<RefCell<Int32Array>>> {
        let ret = unsafe { cbg_Int32Array_Create(size) };
        Int32Array::try_get_from_cache(ret)
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_Int32Array_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for Int32Array {
    fn drop(&mut self) {
        unsafe { cbg_Int32Array_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct VertexArray {
    self_ptr: *mut RawPtr,
}

impl VertexArray {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<VertexArray>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(VertexArray { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static VERTEXARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<VertexArray>>>> = RefCell::new(HashMap::new());
        }
        VERTEXARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。
    pub fn clear(&mut self) -> () {
        unsafe { cbg_VertexArray_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_VertexArray_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> c_void {
        let ret = unsafe { cbg_VertexArray_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: c_void, size: i32) -> () {
        unsafe { cbg_VertexArray_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ
    pub fn copy_to(&mut self, ptr: c_void) -> () {
        unsafe { cbg_VertexArray_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    pub fn get_at(&mut self, index: i32) -> crate::structs::vertex::Vertex {
        let ret = unsafe { cbg_VertexArray_GetAt(self.self_ptr, index) };
        ret.into()
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値
    pub fn set_at(&mut self, index: i32, value: crate::structs::vertex::Vertex) -> () {
        unsafe { cbg_VertexArray_SetAt(self.self_ptr, index, value.into()) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn create(size: i32) -> Option<Rc<RefCell<VertexArray>>> {
        let ret = unsafe { cbg_VertexArray_Create(size) };
        VertexArray::try_get_from_cache(ret)
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_VertexArray_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { cbg_VertexArray_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct FloatArray {
    self_ptr: *mut RawPtr,
}

impl FloatArray {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<FloatArray>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(FloatArray { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static FLOATARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<FloatArray>>>> = RefCell::new(HashMap::new());
        }
        FLOATARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。
    pub fn clear(&mut self) -> () {
        unsafe { cbg_FloatArray_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_FloatArray_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> c_void {
        let ret = unsafe { cbg_FloatArray_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: c_void, size: i32) -> () {
        unsafe { cbg_FloatArray_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ
    pub fn copy_to(&mut self, ptr: c_void) -> () {
        unsafe { cbg_FloatArray_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    pub fn get_at(&mut self, index: i32) -> f32 {
        let ret = unsafe { cbg_FloatArray_GetAt(self.self_ptr, index) };
        ret
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値
    pub fn set_at(&mut self, index: i32, value: f32) -> () {
        unsafe { cbg_FloatArray_SetAt(self.self_ptr, index, value) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn create(size: i32) -> Option<Rc<RefCell<FloatArray>>> {
        let ret = unsafe { cbg_FloatArray_Create(size) };
        FloatArray::try_get_from_cache(ret)
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_FloatArray_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for FloatArray {
    fn drop(&mut self) {
        unsafe { cbg_FloatArray_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Vector2FArray {
    self_ptr: *mut RawPtr,
}

impl Vector2FArray {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Vector2FArray>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Vector2FArray { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static VECTOR2FARRAY_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Vector2FArray>>>> = RefCell::new(HashMap::new());
        }
        VECTOR2FARRAY_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// データをクリアします。
    pub fn clear(&mut self) -> () {
        unsafe { cbg_Vector2FArray_Clear(self.self_ptr) }
    }

    /// サイズを変更します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn resize(&mut self, size: i32) -> () {
        unsafe { cbg_Vector2FArray_Resize(self.self_ptr, size) }
    }

    pub fn get_data(&mut self) -> c_void {
        let ret = unsafe { cbg_Vector2FArray_GetData(self.self_ptr) };
        ret
    }

    pub fn assign(&mut self, ptr: c_void, size: i32) -> () {
        unsafe { cbg_Vector2FArray_Assign(self.self_ptr, ptr, size) }
    }

    /// データを指定したポインタにコピーします。
    /// # Arguments
    /// * `ptr` - ポインタ
    pub fn copy_to(&mut self, ptr: c_void) -> () {
        unsafe { cbg_Vector2FArray_CopyTo(self.self_ptr, ptr) }
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    pub fn get_at(&mut self, index: i32) -> crate::prelude::Vector2<f32> {
        let ret = unsafe { cbg_Vector2FArray_GetAt(self.self_ptr, index) };
        ret.into()
    }

    /// インデックスアクセス
    /// # Arguments
    /// * `index` - インデックス
    /// * `value` - 値
    pub fn set_at(&mut self, index: i32, value: crate::prelude::Vector2<f32>) -> () {
        unsafe { cbg_Vector2FArray_SetAt(self.self_ptr, index, value.into()) }
    }

    /// インスタンスを作成します。
    /// # Arguments
    /// * `size` - 要素数
    pub fn create(size: i32) -> Option<Rc<RefCell<Vector2FArray>>> {
        let ret = unsafe { cbg_Vector2FArray_Create(size) };
        Vector2FArray::try_get_from_cache(ret)
    }

    /// 格納されている要素の数を取得します。
    pub fn get_count(&mut self) -> i32 {
        let ret = unsafe { cbg_Vector2FArray_GetCount(self.self_ptr) };
        ret
    }
}

impl Drop for Vector2FArray {
    fn drop(&mut self) {
        unsafe { cbg_Vector2FArray_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct Resources {
    self_ptr: *mut RawPtr,
}

impl Resources {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Resources>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Resources { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RESOURCES_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Resources>>>> = RefCell::new(HashMap::new());
        }
        RESOURCES_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Resources>>> {
        let ret = unsafe { cbg_Resources_GetInstance() };
        Resources::try_get_from_cache(ret)
    }

    /// 指定した種類のリソースの個数を返します。
    /// # Arguments
    /// * `type_` - 個数を検索するリソースの種類
    pub fn get_resources_count(&mut self, type_: ResourceType) -> i32 {
        let ret = unsafe { cbg_Resources_GetResourcesCount(self.self_ptr, type_ as i32) };
        ret
    }

    /// 登録されたリソースをすべて削除します。
    pub fn clear(&mut self) -> () {
        unsafe { cbg_Resources_Clear(self.self_ptr) }
    }

    /// リソースの再読み込みを行います。
    pub fn reload(&mut self) -> () {
        unsafe { cbg_Resources_Reload(self.self_ptr) }
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        unsafe { cbg_Resources_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Keyboard {
    self_ptr: *mut RawPtr,
}

impl Keyboard {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Keyboard>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Keyboard { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static KEYBOARD_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Keyboard>>>> = RefCell::new(HashMap::new());
        }
        KEYBOARD_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// キーの状態を取得します。
    /// # Arguments
    /// * `key` - キー
    pub fn get_key_state(&mut self, key: Keys) -> ButtonState {
        let ret = unsafe { cbg_Keyboard_GetKeyState(self.self_ptr, key as i32) };
        unsafe { std::mem::transmute(ret) }
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Keyboard>>> {
        let ret = unsafe { cbg_Keyboard_GetInstance() };
        Keyboard::try_get_from_cache(ret)
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe { cbg_Keyboard_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Mouse {
    self_ptr: *mut RawPtr,
    position: Option<crate::prelude::Vector2<f32>>,
    cursor_mode: Option<CursorMode>,
}

impl Mouse {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Mouse>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Mouse {
            self_ptr,
            position: None,
            cursor_mode: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static MOUSE_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Mouse>>>> = RefCell::new(HashMap::new());
        }
        MOUSE_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Mouse>>> {
        let ret = unsafe { cbg_Mouse_GetInstance() };
        Mouse::try_get_from_cache(ret)
    }

    /// マウスホイールの回転量を取得します。
    pub fn get_wheel(&mut self) -> f32 {
        let ret = unsafe { cbg_Mouse_GetWheel(self.self_ptr) };
        ret
    }

    /// マウスボタンの状態を取得します。
    /// # Arguments
    /// * `button` - 状態を取得するマウスのボタン
    pub fn get_mouse_button_state(&mut self, button: MouseButtons) -> ButtonState {
        let ret = unsafe { cbg_Mouse_GetMouseButtonState(self.self_ptr, button as i32) };
        unsafe { std::mem::transmute(ret) }
    }

    /// マウスカーソルの座標を取得または設定します。
    pub fn get_position(&mut self) -> crate::prelude::Vector2<f32> {
        if let Some(value) = self.position.clone() {
            return value;
        }
        let ret = unsafe { cbg_Mouse_GetPosition(self.self_ptr) };
        ret.into()
    }
    pub fn set_position(&mut self, value: crate::prelude::Vector2<f32>) {
        self.position = Some(value.clone());
        unsafe { cbg_Mouse_SetPosition(self.self_ptr, value.into()) }
    }

    /// カーソルのモードを取得または設定します。
    pub fn get_cursor_mode(&mut self) -> CursorMode {
        if let Some(value) = self.cursor_mode.clone() {
            return value;
        }
        let ret = unsafe { cbg_Mouse_GetCursorMode(self.self_ptr) };
        unsafe { std::mem::transmute(ret) }
    }
    pub fn set_cursor_mode(&mut self, value: CursorMode) {
        self.cursor_mode = Some(value.clone());
        unsafe { cbg_Mouse_SetCursorMode(self.self_ptr, value as i32) }
    }
}

impl Drop for Mouse {
    fn drop(&mut self) {
        unsafe { cbg_Mouse_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Joystick {
    self_ptr: *mut RawPtr,
}

impl Joystick {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Joystick>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Joystick { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static JOYSTICK_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Joystick>>>> = RefCell::new(HashMap::new());
        }
        JOYSTICK_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Joystick>>> {
        let ret = unsafe { cbg_Joystick_GetInstance() };
        Joystick::try_get_from_cache(ret)
    }

    /// 指定したジョイスティックが親であるかどうかを取得します。
    /// # Arguments
    /// * `joystick_index` - ジョイスティックのインデックス
    pub fn is_present(&mut self, joystick_index: i32) -> bool {
        let ret = unsafe { cbg_Joystick_IsPresent(self.self_ptr, joystick_index) };
        ret
    }

    /// 接続の状態をリセットします。
    pub fn refresh_connected_state(&mut self) -> () {
        unsafe { cbg_Joystick_RefreshConnectedState(self.self_ptr) }
    }

    /// ボタンの状態をインデックスで取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `button_index` - 状態を検索するボタンのインデックス
    pub fn get_button_state_by_index(
        &mut self,
        joystick_index: i32,
        button_index: i32,
    ) -> ButtonState {
        let ret = unsafe {
            cbg_Joystick_GetButtonStateByIndex(self.self_ptr, joystick_index, button_index)
        };
        unsafe { std::mem::transmute(ret) }
    }

    /// ボタンの状態を種類から取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `type_` - 状態を検索するボタンの種類
    pub fn get_button_state_by_type(
        &mut self,
        joystick_index: i32,
        type_: JoystickButtonType,
    ) -> ButtonState {
        let ret = unsafe {
            cbg_Joystick_GetButtonStateByType(self.self_ptr, joystick_index, type_ as i32)
        };
        unsafe { std::mem::transmute(ret) }
    }

    /// 指定インデックスのジョイスティックの種類を取得します。
    /// # Arguments
    /// * `index` - 種類を取得するジョイスティックのインデックス
    pub fn get_joystick_type(&mut self, index: i32) -> JoystickType {
        let ret = unsafe { cbg_Joystick_GetJoystickType(self.self_ptr, index) };
        unsafe { std::mem::transmute(ret) }
    }

    /// 軸の状態をインデックスで取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `axis_index` - 状態を検索する軸のインデックス
    pub fn get_axis_state_by_index(&mut self, joystick_index: i32, axis_index: i32) -> f32 {
        let ret =
            unsafe { cbg_Joystick_GetAxisStateByIndex(self.self_ptr, joystick_index, axis_index) };
        ret
    }

    /// 軸の状態を軸の種類で取得します。
    /// # Arguments
    /// * `joystick_index` - 検索するジョイスティックのインデックス
    /// * `type_` - 状態を検索する軸の種類
    pub fn get_axis_state_by_type(&mut self, joystick_index: i32, type_: JoystickAxisType) -> f32 {
        let ret =
            unsafe { cbg_Joystick_GetAxisStateByType(self.self_ptr, joystick_index, type_ as i32) };
        ret
    }

    /// ジョイスティックの名前を取得します。
    /// # Arguments
    /// * `index` - 名前を検索するジョイスティックのインデックス
    pub fn get_joystick_name(&mut self, index: i32) -> String {
        let ret = unsafe { cbg_Joystick_GetJoystickName(self.self_ptr, index) };
        decode_string(ret)
    }

    /// 指定したジョイスティックコントローラーを振動させます
    /// # Arguments
    /// * `index` - ジョイスティックのインデックス
    /// * `frequency` - 周波数
    /// * `amplitude` - 振幅
    pub fn vibrate(&mut self, index: i32, frequency: f32, amplitude: f32) -> () {
        unsafe { cbg_Joystick_Vibrate(self.self_ptr, index, frequency, amplitude) }
    }
}

impl Drop for Joystick {
    fn drop(&mut self) {
        unsafe { cbg_Joystick_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct Graphics {
    self_ptr: *mut RawPtr,
}

impl Graphics {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Graphics>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Graphics { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static GRAPHICS_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Graphics>>>> = RefCell::new(HashMap::new());
        }
        GRAPHICS_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Graphics>>> {
        let ret = unsafe { cbg_Graphics_GetInstance() };
        Graphics::try_get_from_cache(ret)
    }

    /// 描画を開始します。
    pub fn begin_frame(&mut self) -> bool {
        let ret = unsafe { cbg_Graphics_BeginFrame(self.self_ptr) };
        ret
    }

    /// 描画を終了します。
    pub fn end_frame(&mut self) -> bool {
        let ret = unsafe { cbg_Graphics_EndFrame(self.self_ptr) };
        ret
    }

    /// イベントを処理します。
    pub fn do_events(&mut self) -> bool {
        let ret = unsafe { cbg_Graphics_DoEvents(self.self_ptr) };
        ret
    }

    /// コマンドリストを取得します。
    pub fn get_command_list(&mut self) -> Option<Rc<RefCell<CommandList>>> {
        let ret = unsafe { cbg_Graphics_GetCommandList(self.self_ptr) };
        CommandList::try_get_from_cache(ret)
    }

    /// 組み込みのシェーダを取得します。
    pub fn get_builtin_shader(&mut self) -> Option<Rc<RefCell<BuiltinShader>>> {
        let ret = unsafe { cbg_Graphics_GetBuiltinShader(self.self_ptr) };
        BuiltinShader::try_get_from_cache(ret)
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe { cbg_Graphics_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Texture2D {
    self_ptr: *mut RawPtr,
}

impl Texture2D {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Texture2D>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Texture2D { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static TEXTURE2D_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Texture2D>>>> = RefCell::new(HashMap::new());
        }
        TEXTURE2D_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// 指定したファイルからテクスチャを読み込みます。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス
    pub fn load(path: &str) -> Option<Rc<RefCell<Texture2D>>> {
        let ret = unsafe { cbg_Texture2D_Load(encode_string(&path).as_ptr()) };
        Texture2D::try_get_from_cache(ret)
    }

    /// 再読み込みを行います。
    pub fn reload(&mut self) -> bool {
        let ret = unsafe { cbg_Texture2D_Reload(self.self_ptr) };
        ret
    }

    /// テクスチャの大きさ(ピクセル)を取得します。
    pub fn get_size(&mut self) -> crate::prelude::Vector2<i32> {
        let ret = unsafe { cbg_Texture2D_GetSize(self.self_ptr) };
        ret.into()
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { cbg_Texture2D_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Material {
    self_ptr: *mut RawPtr,
    shader: Option<Rc<RefCell<Shader>>>,
}

impl Material {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Material>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Material {
            self_ptr,
            shader: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static MATERIAL_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Material>>>> = RefCell::new(HashMap::new());
        }
        MATERIAL_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    ///
    pub fn new() -> Option<Rc<RefCell<Material>>> {
        Self::cbg_create_raw(unsafe { cbg_Material_Constructor_0() })
    }

    ///
    /// # Arguments
    /// * `key` -
    pub fn get_vector4f(&mut self, key: &str) -> crate::prelude::Vector4<f32> {
        let ret = unsafe { cbg_Material_GetVector4F(self.self_ptr, encode_string(&key).as_ptr()) };
        ret.into()
    }

    ///
    /// # Arguments
    /// * `key` -
    /// * `value` -
    pub fn set_vector4f(&mut self, key: &str, value: crate::prelude::Vector4<f32>) -> () {
        unsafe {
            cbg_Material_SetVector4F(self.self_ptr, encode_string(&key).as_ptr(), value.into())
        }
    }

    ///
    /// # Arguments
    /// * `key` -
    pub fn get_matrix44f(&mut self, key: &str) -> crate::prelude::Matrix44<f32> {
        let ret = unsafe { cbg_Material_GetMatrix44F(self.self_ptr, encode_string(&key).as_ptr()) };
        ret.into()
    }

    ///
    /// # Arguments
    /// * `key` -
    /// * `value` -
    pub fn set_matrix44f(&mut self, key: &str, value: crate::prelude::Matrix44<f32>) -> () {
        unsafe {
            cbg_Material_SetMatrix44F(self.self_ptr, encode_string(&key).as_ptr(), value.into())
        }
    }

    ///
    /// # Arguments
    /// * `key` -
    pub fn get_texture(&mut self, key: &str) -> Option<Rc<RefCell<Texture2D>>> {
        let ret = unsafe { cbg_Material_GetTexture(self.self_ptr, encode_string(&key).as_ptr()) };
        Texture2D::try_get_from_cache(ret)
    }

    ///
    /// # Arguments
    /// * `key` -
    /// * `value` -
    pub fn set_texture(&mut self, key: &str, value: &mut Texture2D) -> () {
        unsafe {
            cbg_Material_SetTexture(self.self_ptr, encode_string(&key).as_ptr(), value.self_ptr)
        }
    }

    ///
    pub fn get_shader(&mut self) -> Option<Rc<RefCell<Shader>>> {
        if let Some(value) = self.shader.clone() {
            return Some(value);
        }
        let ret = unsafe { cbg_Material_GetShader(self.self_ptr) };
        Shader::try_get_from_cache(ret)
    }
    pub fn set_shader(&mut self, value: Rc<RefCell<Shader>>) {
        self.shader = Some(value.clone());
        unsafe { cbg_Material_SetShader(self.self_ptr, value.borrow_mut().self_ptr) }
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        unsafe { cbg_Material_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct Renderer {
    self_ptr: *mut RawPtr,
}

impl Renderer {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Renderer>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Renderer { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RENDERER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Renderer>>>> = RefCell::new(HashMap::new());
        }
        RENDERER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Renderer>>> {
        let ret = unsafe { cbg_Renderer_GetInstance() };
        Renderer::try_get_from_cache(ret)
    }

    /// スプライトを描画します。
    pub fn draw_sprite(&mut self, sprite: &mut RenderedSprite) -> () {
        unsafe { cbg_Renderer_DrawSprite(self.self_ptr, sprite.self_ptr) }
    }

    /// テキストを描画します。
    pub fn draw_text(&mut self, text: &mut RenderedText) -> () {
        unsafe { cbg_Renderer_DrawText(self.self_ptr, text.self_ptr) }
    }

    /// コマンドリストを描画します。
    /// # Arguments
    /// * `command_list` - コマンドリスト
    pub fn render(&mut self, command_list: &mut CommandList) -> () {
        unsafe { cbg_Renderer_Render(self.self_ptr, command_list.self_ptr) }
    }

    /// ポリゴンを描画します
    /// # Arguments
    /// * `vertex_buffer` - 頂点バッファ
    /// * `index_buffer` -
    /// * `texture` - テクスチャ
    /// * `material` - マテリアル
    pub fn draw_polygon(
        &mut self,
        vertex_buffer: &mut VertexArray,
        index_buffer: &mut Int32Array,
        texture: &mut Texture2D,
        material: &mut Material,
    ) -> () {
        unsafe {
            cbg_Renderer_DrawPolygon(
                self.self_ptr,
                vertex_buffer.self_ptr,
                index_buffer.self_ptr,
                texture.self_ptr,
                material.self_ptr,
            )
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { cbg_Renderer_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct CommandList {
    self_ptr: *mut RawPtr,
}

impl CommandList {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<CommandList>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(CommandList { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static COMMANDLIST_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<CommandList>>>> = RefCell::new(HashMap::new());
        }
        COMMANDLIST_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// ？
    pub fn set_render_target_with_screen(&mut self) -> () {
        unsafe { cbg_CommandList_SetRenderTargetWithScreen(self.self_ptr) }
    }
}

impl Drop for CommandList {
    fn drop(&mut self) {
        unsafe { cbg_CommandList_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Rendered {
    self_ptr: *mut RawPtr,
}

impl Rendered {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Rendered>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Rendered { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RENDERED_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Rendered>>>> = RefCell::new(HashMap::new());
        }
        RENDERED_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }
}

impl Drop for Rendered {
    fn drop(&mut self) {
        unsafe { cbg_Rendered_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct RenderedSprite {
    self_ptr: *mut RawPtr,
    texture: Option<Rc<RefCell<Texture2D>>>,
    src: Option<crate::structs::rect::Rect<f32>>,
    transform: Option<crate::prelude::Matrix44<f32>>,
    material: Option<Rc<RefCell<Material>>>,
}

impl RenderedSprite {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<RenderedSprite>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(RenderedSprite {
            self_ptr,
            texture: None,
            src: None,
            transform: None,
            material: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RENDEREDSPRITE_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<RenderedSprite>>>> = RefCell::new(HashMap::new());
        }
        RENDEREDSPRITE_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// スプライトを作成します。
    pub fn create() -> Option<Rc<RefCell<RenderedSprite>>> {
        let ret = unsafe { cbg_RenderedSprite_Create() };
        RenderedSprite::try_get_from_cache(ret)
    }

    /// テクスチャを取得または設定します。
    pub fn get_texture(&mut self) -> Option<Rc<RefCell<Texture2D>>> {
        if let Some(value) = self.texture.clone() {
            return Some(value);
        }
        let ret = unsafe { cbg_RenderedSprite_GetTexture(self.self_ptr) };
        Texture2D::try_get_from_cache(ret)
    }
    pub fn set_texture(&mut self, value: Rc<RefCell<Texture2D>>) {
        self.texture = Some(value.clone());
        unsafe { cbg_RenderedSprite_SetTexture(self.self_ptr, value.borrow_mut().self_ptr) }
    }

    /// 描画範囲を取得または設定します。
    pub fn get_src(&mut self) -> crate::structs::rect::Rect<f32> {
        if let Some(value) = self.src.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedSprite_GetSrc(self.self_ptr) };
        ret.into()
    }
    pub fn set_src(&mut self, value: crate::structs::rect::Rect<f32>) {
        self.src = Some(value.clone());
        unsafe { cbg_RenderedSprite_SetSrc(self.self_ptr, value.into()) }
    }

    /// 変換行列を取得または設定します。
    pub fn get_transform(&mut self) -> crate::prelude::Matrix44<f32> {
        if let Some(value) = self.transform.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedSprite_GetTransform(self.self_ptr) };
        ret.into()
    }
    pub fn set_transform(&mut self, value: crate::prelude::Matrix44<f32>) {
        self.transform = Some(value.clone());
        unsafe { cbg_RenderedSprite_SetTransform(self.self_ptr, value.into()) }
    }

    /// マテリアルを取得または設定します。
    pub fn get_material(&mut self) -> Option<Rc<RefCell<Material>>> {
        if let Some(value) = self.material.clone() {
            return Some(value);
        }
        let ret = unsafe { cbg_RenderedSprite_GetMaterial(self.self_ptr) };
        Material::try_get_from_cache(ret)
    }
    pub fn set_material(&mut self, value: Rc<RefCell<Material>>) {
        self.material = Some(value.clone());
        unsafe { cbg_RenderedSprite_SetMaterial(self.self_ptr, value.borrow_mut().self_ptr) }
    }
}

impl Drop for RenderedSprite {
    fn drop(&mut self) {
        unsafe { cbg_RenderedSprite_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct RenderedText {
    self_ptr: *mut RawPtr,
    transform: Option<crate::prelude::Matrix44<f32>>,
    material: Option<Rc<RefCell<Material>>>,
    text: Option<String>,
    font: Option<Arc<Mutex<Font>>>,
    weight: Option<f32>,
    color: Option<crate::structs::color::Color>,
}

impl RenderedText {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<RenderedText>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(RenderedText {
            self_ptr,
            transform: None,
            material: None,
            text: None,
            font: None,
            weight: None,
            color: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RENDEREDTEXT_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<RenderedText>>>> = RefCell::new(HashMap::new());
        }
        RENDEREDTEXT_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// テキストを作成します。
    pub fn create() -> Option<Rc<RefCell<RenderedText>>> {
        let ret = unsafe { cbg_RenderedText_Create() };
        RenderedText::try_get_from_cache(ret)
    }

    /// 変換行列を取得または設定します。
    pub fn get_transform(&mut self) -> crate::prelude::Matrix44<f32> {
        if let Some(value) = self.transform.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedText_GetTransform(self.self_ptr) };
        ret.into()
    }
    pub fn set_transform(&mut self, value: crate::prelude::Matrix44<f32>) {
        self.transform = Some(value.clone());
        unsafe { cbg_RenderedText_SetTransform(self.self_ptr, value.into()) }
    }

    /// マテリアルを取得または設定します。
    pub fn get_material(&mut self) -> Option<Rc<RefCell<Material>>> {
        if let Some(value) = self.material.clone() {
            return Some(value);
        }
        let ret = unsafe { cbg_RenderedText_GetMaterial(self.self_ptr) };
        Material::try_get_from_cache(ret)
    }
    pub fn set_material(&mut self, value: Rc<RefCell<Material>>) {
        self.material = Some(value.clone());
        unsafe { cbg_RenderedText_SetMaterial(self.self_ptr, value.borrow_mut().self_ptr) }
    }

    /// テキストを取得または設定します。
    pub fn get_text(&mut self) -> String {
        if let Some(value) = self.text.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedText_GetText(self.self_ptr) };
        decode_string(ret)
    }
    pub fn set_text(&mut self, value: String) {
        self.text = Some(value.clone());
        unsafe { cbg_RenderedText_SetText(self.self_ptr, encode_string(&value).as_ptr()) }
    }

    /// フォントを取得または設定します。
    pub fn get_font(&mut self) -> Option<Arc<Mutex<Font>>> {
        if let Some(value) = self.font.clone() {
            return Some(value);
        }
        let ret = unsafe { cbg_RenderedText_GetFont(self.self_ptr) };
        Font::try_get_from_cache(ret)
    }
    pub fn set_font(&mut self, value: Arc<Mutex<Font>>) {
        self.font = Some(value.clone());
        unsafe {
            cbg_RenderedText_SetFont(
                self.self_ptr,
                value.lock().expect("Failed to get lock of Font").self_ptr,
            )
        }
    }

    /// 文字の太さを取得または設定します。(0 ~ 255)
    pub fn get_weight(&mut self) -> f32 {
        if let Some(value) = self.weight.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedText_GetWeight(self.self_ptr) };
        ret
    }
    pub fn set_weight(&mut self, value: f32) {
        self.weight = Some(value.clone());
        unsafe { cbg_RenderedText_SetWeight(self.self_ptr, value) }
    }

    /// 色を取得または設定します。
    pub fn get_color(&mut self) -> crate::structs::color::Color {
        if let Some(value) = self.color.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedText_GetColor(self.self_ptr) };
        ret.into()
    }
    pub fn set_color(&mut self, value: crate::structs::color::Color) {
        self.color = Some(value.clone());
        unsafe { cbg_RenderedText_SetColor(self.self_ptr, value.into()) }
    }
}

impl Drop for RenderedText {
    fn drop(&mut self) {
        unsafe { cbg_RenderedText_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct RenderedPolygon {
    self_ptr: *mut RawPtr,
    texture: Option<Rc<RefCell<Texture2D>>>,
    src: Option<crate::structs::rect::Rect<f32>>,
    transform: Option<crate::prelude::Matrix44<f32>>,
    material: Option<Rc<RefCell<Material>>>,
}

impl RenderedPolygon {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<RenderedPolygon>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(RenderedPolygon {
            self_ptr,
            texture: None,
            src: None,
            transform: None,
            material: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RENDEREDPOLYGON_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<RenderedPolygon>>>> = RefCell::new(HashMap::new());
        }
        RENDEREDPOLYGON_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// ポリゴンを作成します。
    pub fn create() -> Option<Rc<RefCell<RenderedPolygon>>> {
        let ret = unsafe { cbg_RenderedPolygon_Create() };
        RenderedPolygon::try_get_from_cache(ret)
    }

    /// 頂点情報を取得します。
    pub fn get_vertexes(&mut self) -> Option<Rc<RefCell<VertexArray>>> {
        let ret = unsafe { cbg_RenderedPolygon_GetVertexes(self.self_ptr) };
        VertexArray::try_get_from_cache(ret)
    }

    /// 頂点情報
    pub fn set_vertexes(&mut self, vertexes: &mut VertexArray) -> () {
        unsafe { cbg_RenderedPolygon_SetVertexes(self.self_ptr, vertexes.self_ptr) }
    }

    /// 頂点情報
    pub fn set_vertexes_by_vector2f(&mut self, vertexes: &mut Vector2FArray) -> () {
        unsafe { cbg_RenderedPolygon_SetVertexesByVector2F(self.self_ptr, vertexes.self_ptr) }
    }

    /// テクスチャを取得または設定します。
    pub fn get_texture(&mut self) -> Option<Rc<RefCell<Texture2D>>> {
        if let Some(value) = self.texture.clone() {
            return Some(value);
        }
        let ret = unsafe { cbg_RenderedPolygon_GetTexture(self.self_ptr) };
        Texture2D::try_get_from_cache(ret)
    }
    pub fn set_texture(&mut self, value: Rc<RefCell<Texture2D>>) {
        self.texture = Some(value.clone());
        unsafe { cbg_RenderedPolygon_SetTexture(self.self_ptr, value.borrow_mut().self_ptr) }
    }

    /// 描画範囲を取得または設定します。
    pub fn get_src(&mut self) -> crate::structs::rect::Rect<f32> {
        if let Some(value) = self.src.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedPolygon_GetSrc(self.self_ptr) };
        ret.into()
    }
    pub fn set_src(&mut self, value: crate::structs::rect::Rect<f32>) {
        self.src = Some(value.clone());
        unsafe { cbg_RenderedPolygon_SetSrc(self.self_ptr, value.into()) }
    }

    /// 変換行列を取得または設定します。
    pub fn get_transform(&mut self) -> crate::prelude::Matrix44<f32> {
        if let Some(value) = self.transform.clone() {
            return value;
        }
        let ret = unsafe { cbg_RenderedPolygon_GetTransform(self.self_ptr) };
        ret.into()
    }
    pub fn set_transform(&mut self, value: crate::prelude::Matrix44<f32>) {
        self.transform = Some(value.clone());
        unsafe { cbg_RenderedPolygon_SetTransform(self.self_ptr, value.into()) }
    }

    /// マテリアルを取得または設定します。
    pub fn get_material(&mut self) -> Option<Rc<RefCell<Material>>> {
        if let Some(value) = self.material.clone() {
            return Some(value);
        }
        let ret = unsafe { cbg_RenderedPolygon_GetMaterial(self.self_ptr) };
        Material::try_get_from_cache(ret)
    }
    pub fn set_material(&mut self, value: Rc<RefCell<Material>>) {
        self.material = Some(value.clone());
        unsafe { cbg_RenderedPolygon_SetMaterial(self.self_ptr, value.borrow_mut().self_ptr) }
    }
}

impl Drop for RenderedPolygon {
    fn drop(&mut self) {
        unsafe { cbg_RenderedPolygon_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct RenderedCamera {
    self_ptr: *mut RawPtr,
}

impl RenderedCamera {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<RenderedCamera>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(RenderedCamera { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static RENDEREDCAMERA_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<RenderedCamera>>>> = RefCell::new(HashMap::new());
        }
        RENDEREDCAMERA_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }
}

impl Drop for RenderedCamera {
    fn drop(&mut self) {
        unsafe { cbg_RenderedCamera_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct BuiltinShader {
    self_ptr: *mut RawPtr,
}

impl BuiltinShader {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<BuiltinShader>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(BuiltinShader { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static BUILTINSHADER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<BuiltinShader>>>> = RefCell::new(HashMap::new());
        }
        BUILTINSHADER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// シェーダを取得します。
    /// # Arguments
    /// * `type_` - シェーダの種類
    pub fn create(&mut self, type_: BuiltinShaderType) -> Option<Rc<RefCell<Shader>>> {
        let ret = unsafe { cbg_BuiltinShader_Create(self.self_ptr, type_ as i32) };
        Shader::try_get_from_cache(ret)
    }
}

impl Drop for BuiltinShader {
    fn drop(&mut self) {
        unsafe { cbg_BuiltinShader_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Shader {
    self_ptr: *mut RawPtr,
}

impl Shader {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Shader>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Shader { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static SHADER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Shader>>>> = RefCell::new(HashMap::new());
        }
        SHADER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { cbg_Shader_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Glyph {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for Glyph {}
unsafe impl Sync for Glyph {}

impl Glyph {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Glyph>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(Glyph { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref GLYPH_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<Glyph>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = GLYPH_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 文字テクスチャのサイズを取得する
    pub fn get_texture_size(&mut self) -> crate::prelude::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetTextureSize(self.self_ptr) };
        ret.into()
    }

    /// 文字テクスチャのインデックスを取得する
    pub fn get_texture_index(&mut self) -> i32 {
        let ret = unsafe { cbg_Glyph_GetTextureIndex(self.self_ptr) };
        ret
    }

    /// 文字の座標を取得する
    pub fn get_position(&mut self) -> crate::prelude::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetPosition(self.self_ptr) };
        ret.into()
    }

    /// 文字のサイズを取得する
    pub fn get_size(&mut self) -> crate::prelude::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetSize(self.self_ptr) };
        ret.into()
    }

    /// 文字のオフセットを取得する
    pub fn get_offset(&mut self) -> crate::prelude::Vector2<i32> {
        let ret = unsafe { cbg_Glyph_GetOffset(self.self_ptr) };
        ret.into()
    }

    /// 文字の幅を取得する
    pub fn get_glyph_width(&mut self) -> i32 {
        let ret = unsafe { cbg_Glyph_GetGlyphWidth(self.self_ptr) };
        ret
    }
}

impl Drop for Glyph {
    fn drop(&mut self) {
        unsafe { cbg_Glyph_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Font {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}

impl Font {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Font>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(Font { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref FONT_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<Font>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = FONT_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 動的にフォントを生成します
    /// # Arguments
    /// * `path` - 読み込むフォントのパス
    /// * `size` - フォントのサイズ
    pub fn load_dynamic_font(path: &str, size: i32) -> Option<Arc<Mutex<Font>>> {
        let ret = unsafe { cbg_Font_LoadDynamicFont(encode_string(&path).as_ptr(), size) };
        Font::try_get_from_cache(ret)
    }

    /// 静的にフォントを生成します
    /// # Arguments
    /// * `path` - 読み込むフォントのパス
    pub fn load_static_font(path: &str) -> Option<Arc<Mutex<Font>>> {
        let ret = unsafe { cbg_Font_LoadStaticFont(encode_string(&path).as_ptr()) };
        Font::try_get_from_cache(ret)
    }

    /// 文字情報を取得する
    /// # Arguments
    /// * `character` - 文字
    pub fn get_glyph(&mut self, character: i32) -> Option<Arc<Mutex<Glyph>>> {
        let ret = unsafe { cbg_Font_GetGlyph(self.self_ptr, character) };
        Glyph::try_get_from_cache(ret)
    }

    /// 文字列テクスチャを得る
    /// # Arguments
    /// * `index` - インデックス
    pub fn get_font_texture(&mut self, index: i32) -> Option<Rc<RefCell<Texture2D>>> {
        let ret = unsafe { cbg_Font_GetFontTexture(self.self_ptr, index) };
        Texture2D::try_get_from_cache(ret)
    }

    /// カーニングを得る
    /// # Arguments
    /// * `c1` - 文字1
    /// * `c2` - 文字2
    pub fn get_kerning(&mut self, c1: i32, c2: i32) -> i32 {
        let ret = unsafe { cbg_Font_GetKerning(self.self_ptr, c1, c2) };
        ret
    }

    /// 読み込んだファイルのパスを取得します。
    pub(crate) fn get_path(&mut self) -> String {
        let ret = unsafe { cbg_Font_GetPath(self.self_ptr) };
        decode_string(ret)
    }

    /// テキストを描画したときのサイズを取得します
    /// # Arguments
    /// * `text` - テキスト
    /// * `direction` - 文字列の方向
    /// * `is_enable_kerning` - カーニングの有無
    pub fn calc_texture_size(
        &mut self,
        text: &str,
        direction: WritingDirection,
        is_enable_kerning: bool,
    ) -> crate::prelude::Vector2<i32> {
        let ret = unsafe {
            cbg_Font_CalcTextureSize(
                self.self_ptr,
                encode_string(&text).as_ptr(),
                direction as i32,
                is_enable_kerning,
            )
        };
        ret.into()
    }

    /// テクスチャ追加対応フォントを生成します
    /// # Arguments
    /// * `base_font` - ベースとなるフォント
    pub fn create_image_font(base_font: &mut Font) -> Option<Arc<Mutex<Font>>> {
        let ret = unsafe { cbg_Font_CreateImageFont(base_font.self_ptr) };
        Font::try_get_from_cache(ret)
    }

    /// テクスチャ文字を追加する
    /// # Arguments
    /// * `character` - 文字
    /// * `texture` - テクスチャ
    pub(crate) fn add_image_glyph(&mut self, character: i32, texture: &mut Texture2D) -> () {
        unsafe { cbg_Font_AddImageGlyph(self.self_ptr, character, texture.self_ptr) }
    }

    /// テクスチャ文字を取得する
    /// # Arguments
    /// * `character` - 文字
    pub fn get_image_glyph(&mut self, character: i32) -> Option<Rc<RefCell<Texture2D>>> {
        let ret = unsafe { cbg_Font_GetImageGlyph(self.self_ptr, character) };
        Texture2D::try_get_from_cache(ret)
    }

    /// フォントのサイズを取得する
    pub fn get_size(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetSize(self.self_ptr) };
        ret
    }

    /// フォントのベースラインからトップラインまでの距離を取得する
    pub fn get_ascent(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetAscent(self.self_ptr) };
        ret
    }

    /// フォントのベースラインからボトムラインまでの距離を取得する
    pub fn get_descent(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetDescent(self.self_ptr) };
        ret
    }

    /// フォントの行間の距離を取得する
    pub fn get_line_gap(&mut self) -> i32 {
        let ret = unsafe { cbg_Font_GetLineGap(self.self_ptr) };
        ret
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { cbg_Font_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Tool {
    self_ptr: *mut RawPtr,
}

impl Tool {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Tool>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Tool { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static TOOL_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Tool>>>> = RefCell::new(HashMap::new());
        }
        TOOL_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    pub(crate) fn get_instance() -> Option<Rc<RefCell<Tool>>> {
        let ret = unsafe { cbg_Tool_GetInstance() };
        Tool::try_get_from_cache(ret)
    }

    /// `End()` を呼び出してください。
    pub fn begin(&mut self, name: &str, flags: ToolWindow) -> bool {
        let ret =
            unsafe { cbg_Tool_Begin(self.self_ptr, encode_string(&name).as_ptr(), flags as i32) };
        ret
    }

    ///
    pub fn end(&mut self) -> () {
        unsafe { cbg_Tool_End(self.self_ptr) }
    }

    ///
    pub fn dummy(&mut self, size: crate::prelude::Vector2<f32>) -> () {
        unsafe { cbg_Tool_Dummy(self.self_ptr, size.into()) }
    }

    ///
    pub fn text(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_Text(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///
    pub fn text_unformatted(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_TextUnformatted(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///
    pub fn text_wrapped(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_TextWrapped(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///
    pub fn text_colored(&mut self, color: crate::prelude::Vector4<f32>, text: &str) -> () {
        unsafe { cbg_Tool_TextColored(self.self_ptr, color.into(), encode_string(&text).as_ptr()) }
    }

    ///
    pub fn text_disabled(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_TextDisabled(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///
    pub fn bullet_text(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_BulletText(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    ///
    pub fn label_text(&mut self, label: &str, text: &str) -> () {
        unsafe {
            cbg_Tool_LabelText(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                encode_string(&text).as_ptr(),
            )
        }
    }

    ///
    pub fn collapsing_header(&mut self, label: &str, flags: ToolTreeNode) -> bool {
        let ret = unsafe {
            cbg_Tool_CollapsingHeader(self.self_ptr, encode_string(&label).as_ptr(), flags as i32)
        };
        ret
    }

    ///
    pub fn tree_node(&mut self, label: &str) -> bool {
        let ret = unsafe { cbg_Tool_TreeNode(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    ///
    pub fn tree_node_ex(&mut self, label: &str, flags: ToolTreeNode) -> bool {
        let ret = unsafe {
            cbg_Tool_TreeNodeEx(self.self_ptr, encode_string(&label).as_ptr(), flags as i32)
        };
        ret
    }

    ///
    pub fn tree_pop(&mut self) -> () {
        unsafe { cbg_Tool_TreePop(self.self_ptr) }
    }

    ///
    pub fn set_next_item_open(&mut self, is_open: bool, cond: ToolCond) -> () {
        unsafe { cbg_Tool_SetNextItemOpen(self.self_ptr, is_open, cond as i32) }
    }

    ///
    pub fn button(&mut self, label: &str, size: crate::prelude::Vector2<f32>) -> bool {
        let ret =
            unsafe { cbg_Tool_Button(self.self_ptr, encode_string(&label).as_ptr(), size.into()) };
        ret
    }

    ///
    pub fn check_box(&mut self, label: &str, is_open: &mut bool) -> bool {
        let ret = unsafe {
            cbg_Tool_CheckBox(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                is_open as *mut bool,
            )
        };
        ret
    }

    ///
    pub fn radio_button(&mut self, label: &str, active: bool) -> bool {
        let ret =
            unsafe { cbg_Tool_RadioButton(self.self_ptr, encode_string(&label).as_ptr(), active) };
        ret
    }

    ///
    pub fn arrow_button(&mut self, label: &str, dir: ToolDir) -> bool {
        let ret = unsafe {
            cbg_Tool_ArrowButton(self.self_ptr, encode_string(&label).as_ptr(), dir as i32)
        };
        ret
    }

    ///
    pub fn invisible_button(&mut self, label: &str, size: crate::prelude::Vector2<f32>) -> bool {
        let ret = unsafe {
            cbg_Tool_InvisibleButton(self.self_ptr, encode_string(&label).as_ptr(), size.into())
        };
        ret
    }

    ///
    pub fn selectable(&mut self, label: &str, selected: &bool, flags: ToolSelectable) -> bool {
        let ret = unsafe {
            cbg_Tool_Selectable(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                selected as *const bool,
                flags as i32,
            )
        };
        ret
    }

    ///
    pub fn input_int(&mut self, label: &str, value: &i32) -> bool {
        let ret = unsafe {
            cbg_Tool_InputInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_int,
            )
        };
        ret
    }

    ///
    pub fn input_float(&mut self, label: &str, value: &f32) -> bool {
        let ret = unsafe {
            cbg_Tool_InputFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_float,
            )
        };
        ret
    }

    ///
    pub fn slider_int(
        &mut self,
        label: &str,
        value: &i32,
        speed: f32,
        value_min: i32,
        value_max: i32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_SliderInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_int,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///
    pub fn slider_float(
        &mut self,
        label: &str,
        value: &f32,
        speed: f32,
        value_min: f32,
        value_max: f32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_SliderFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_float,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///
    pub fn slider_angle(&mut self, label: &str, angle: &f32) -> bool {
        let ret = unsafe {
            cbg_Tool_SliderAngle(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                angle as *const c_float,
            )
        };
        ret
    }

    ///
    pub fn vslider_int(
        &mut self,
        label: &str,
        size: crate::prelude::Vector2<f32>,
        value: &i32,
        value_min: i32,
        value_max: i32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_VSliderInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                size.into(),
                value as *const c_int,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///
    pub fn vslider_float(
        &mut self,
        label: &str,
        size: crate::prelude::Vector2<f32>,
        value: &f32,
        value_min: f32,
        value_max: f32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_VSliderFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                size.into(),
                value as *const c_float,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///
    pub fn drag_int(
        &mut self,
        label: &str,
        value: &i32,
        speed: f32,
        value_min: i32,
        value_max: i32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_DragInt(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_int,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///
    pub fn drag_float(
        &mut self,
        label: &str,
        value: &f32,
        speed: f32,
        value_min: f32,
        value_max: f32,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_DragFloat(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                value as *const c_float,
                speed,
                value_min,
                value_max,
            )
        };
        ret
    }

    ///
    pub fn open_popup(&mut self, label: &str) -> () {
        unsafe { cbg_Tool_OpenPopup(self.self_ptr, encode_string(&label).as_ptr()) }
    }

    /// `EndPopup()` を呼び出してください
    pub fn begin_popup(&mut self, label: &str) -> bool {
        let ret = unsafe { cbg_Tool_BeginPopup(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    /// `EndPopup()` を呼び出してください
    pub fn begin_popup_modal(&mut self, label: &str) -> bool {
        let ret =
            unsafe { cbg_Tool_BeginPopupModal(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    ///
    pub fn end_popup(&mut self) -> () {
        unsafe { cbg_Tool_EndPopup(self.self_ptr) }
    }

    /// `EndChild()` を呼び出してください
    pub fn begin_child(
        &mut self,
        label: &str,
        size: crate::prelude::Vector2<f32>,
        border: bool,
        flags: ToolWindow,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_BeginChild(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                size.into(),
                border,
                flags as i32,
            )
        };
        ret
    }

    ///
    pub fn end_child(&mut self) -> () {
        unsafe { cbg_Tool_EndChild(self.self_ptr) }
    }

    /// `EndMenuBar()` を呼び出してください
    pub fn begin_menu_bar(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_BeginMenuBar(self.self_ptr) };
        ret
    }

    ///
    pub fn end_menu_bar(&mut self) -> () {
        unsafe { cbg_Tool_EndMenuBar(self.self_ptr) }
    }

    /// `EndMenu()` を呼び出してください
    pub fn begin_menu(&mut self, label: &str, enabled: bool) -> bool {
        let ret =
            unsafe { cbg_Tool_BeginMenu(self.self_ptr, encode_string(&label).as_ptr(), enabled) };
        ret
    }

    ///
    pub fn end_menu(&mut self) -> () {
        unsafe { cbg_Tool_EndMenu(self.self_ptr) }
    }

    ///
    pub fn menu_item(
        &mut self,
        label: &str,
        shortcut: &str,
        selected: bool,
        enabled: bool,
    ) -> bool {
        let ret = unsafe {
            cbg_Tool_MenuItem(
                self.self_ptr,
                encode_string(&label).as_ptr(),
                encode_string(&shortcut).as_ptr(),
                selected,
                enabled,
            )
        };
        ret
    }

    /// `EndTabBar()` を呼び出してください
    pub fn begin_tab_bar(&mut self, label: &str, flags: ToolTabBar) -> bool {
        let ret = unsafe {
            cbg_Tool_BeginTabBar(self.self_ptr, encode_string(&label).as_ptr(), flags as i32)
        };
        ret
    }

    ///
    pub fn end_tab_bar(&mut self) -> () {
        unsafe { cbg_Tool_EndTabBar(self.self_ptr) }
    }

    /// `EndTabItem()` を呼び出してください
    pub fn begin_tab_item(&mut self, label: &str) -> bool {
        let ret = unsafe { cbg_Tool_BeginTabItem(self.self_ptr, encode_string(&label).as_ptr()) };
        ret
    }

    ///
    pub fn end_tab_item(&mut self) -> () {
        unsafe { cbg_Tool_EndTabItem(self.self_ptr) }
    }

    ///
    pub fn indent(&mut self) -> () {
        unsafe { cbg_Tool_Indent(self.self_ptr) }
    }

    ///
    pub fn unindent(&mut self) -> () {
        unsafe { cbg_Tool_Unindent(self.self_ptr) }
    }

    ///
    pub fn separator(&mut self) -> () {
        unsafe { cbg_Tool_Separator(self.self_ptr) }
    }

    ///
    pub fn set_tooltip(&mut self, text: &str) -> () {
        unsafe { cbg_Tool_SetTooltip(self.self_ptr, encode_string(&text).as_ptr()) }
    }

    /// `EndTooltip()` を呼び出してください
    pub fn begin_tooltip(&mut self) -> () {
        unsafe { cbg_Tool_BeginTooltip(self.self_ptr) }
    }

    ///
    pub fn end_tooltip(&mut self) -> () {
        unsafe { cbg_Tool_EndTooltip(self.self_ptr) }
    }

    ///
    pub fn new_line(&mut self) -> () {
        unsafe { cbg_Tool_NewLine(self.self_ptr) }
    }

    ///
    pub fn same_line(&mut self) -> () {
        unsafe { cbg_Tool_SameLine(self.self_ptr) }
    }

    ///
    pub fn push_text_wrap_pos(&mut self, wrap_local_pos_x: f32) -> () {
        unsafe { cbg_Tool_PushTextWrapPos(self.self_ptr, wrap_local_pos_x) }
    }

    ///
    pub fn pop_text_wrap_pos(&mut self) -> () {
        unsafe { cbg_Tool_PopTextWrapPos(self.self_ptr) }
    }

    ///
    pub fn set_next_item_width(&mut self, width: f32) -> () {
        unsafe { cbg_Tool_SetNextItemWidth(self.self_ptr, width) }
    }

    ///
    pub fn push_item_width(&mut self, width: f32) -> () {
        unsafe { cbg_Tool_PushItemWidth(self.self_ptr, width) }
    }

    ///
    pub fn pop_item_width(&mut self) -> () {
        unsafe { cbg_Tool_PopItemWidth(self.self_ptr) }
    }

    ///
    pub fn push_button_repeat(&mut self, repeat: bool) -> () {
        unsafe { cbg_Tool_PushButtonRepeat(self.self_ptr, repeat) }
    }

    ///
    pub fn pop_button_repeat(&mut self) -> () {
        unsafe { cbg_Tool_PopButtonRepeat(self.self_ptr) }
    }

    ///
    pub fn columns(&mut self, count: i32, border: bool) -> () {
        unsafe { cbg_Tool_Columns(self.self_ptr, count, border) }
    }

    ///
    pub fn next_column(&mut self) -> () {
        unsafe { cbg_Tool_NextColumn(self.self_ptr) }
    }

    ///
    pub fn push_id(&mut self, id: i32) -> () {
        unsafe { cbg_Tool_PushID(self.self_ptr, id) }
    }

    ///
    pub fn pop_id(&mut self) -> () {
        unsafe { cbg_Tool_PopID(self.self_ptr) }
    }

    ///
    pub fn is_item_active(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsItemActive(self.self_ptr) };
        ret
    }

    ///
    pub fn is_item_hovered(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsItemHovered(self.self_ptr) };
        ret
    }

    ///
    pub fn set_scroll_here(&mut self) -> () {
        unsafe { cbg_Tool_SetScrollHere(self.self_ptr) }
    }

    ///
    pub fn set_scroll_here_x(&mut self) -> () {
        unsafe { cbg_Tool_SetScrollHereX(self.self_ptr) }
    }

    ///
    pub fn set_scroll_here_y(&mut self) -> () {
        unsafe { cbg_Tool_SetScrollHereY(self.self_ptr) }
    }

    ///
    pub fn get_text_line_height(&mut self) -> f32 {
        let ret = unsafe { cbg_Tool_GetTextLineHeight(self.self_ptr) };
        ret
    }

    ///
    pub fn get_font_size(&mut self) -> f32 {
        let ret = unsafe { cbg_Tool_GetFontSize(self.self_ptr) };
        ret
    }

    ///
    pub fn get_window_size(&mut self) -> crate::prelude::Vector2<f32> {
        let ret = unsafe { cbg_Tool_GetWindowSize(self.self_ptr) };
        ret.into()
    }

    ///
    pub fn set_window_size(&mut self, size: crate::prelude::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetWindowSize(self.self_ptr, size.into()) }
    }

    ///
    pub fn is_mouse_pos_valid(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsMousePosValid(self.self_ptr) };
        ret
    }

    ///
    pub fn is_mouse_dragging(&mut self) -> bool {
        let ret = unsafe { cbg_Tool_IsMouseDragging(self.self_ptr) };
        ret
    }

    ///
    pub fn is_mouse_double_clicked(&mut self, button: i32) -> bool {
        let ret = unsafe { cbg_Tool_IsMouseDoubleClicked(self.self_ptr, button) };
        ret
    }

    ///
    pub fn get_mouse_drag_delta(&mut self, button: i32) -> crate::prelude::Vector2<f32> {
        let ret = unsafe { cbg_Tool_GetMouseDragDelta(self.self_ptr, button) };
        ret.into()
    }

    ///
    pub fn reset_mouse_drag_delta(&mut self, button: i32) -> () {
        unsafe { cbg_Tool_ResetMouseDragDelta(self.self_ptr, button) }
    }

    ///
    pub fn set_next_window_content_size(&mut self, size: crate::prelude::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetNextWindowContentSize(self.self_ptr, size.into()) }
    }

    ///
    pub fn set_next_window_size(&mut self, size: crate::prelude::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetNextWindowSize(self.self_ptr, size.into()) }
    }

    ///
    pub fn set_next_window_pos(&mut self, pos: crate::prelude::Vector2<f32>) -> () {
        unsafe { cbg_Tool_SetNextWindowPos(self.self_ptr, pos.into()) }
    }
}

impl Drop for Tool {
    fn drop(&mut self) {
        unsafe { cbg_Tool_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct StreamFile {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for StreamFile {}
unsafe impl Sync for StreamFile {}

impl StreamFile {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<StreamFile>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(StreamFile { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref STREAMFILE_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<StreamFile>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = STREAMFILE_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 指定ファイルを読み込むStreamFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス
    pub fn create(path: &str) -> Option<Arc<Mutex<StreamFile>>> {
        let ret = unsafe { cbg_StreamFile_Create(encode_string(&path).as_ptr()) };
        StreamFile::try_get_from_cache(ret)
    }

    /// 指定した分ファイルを読み込む
    /// # Arguments
    /// * `size` - この処理で読み込むデータサイズ
    pub fn read(&mut self, size: i32) -> i32 {
        let ret = unsafe { cbg_StreamFile_Read(self.self_ptr, size) };
        ret
    }

    /// 現在読み込んでいるファイルのデータを取得します。
    pub(crate) fn get_temp_buffer(&mut self) -> Option<Rc<RefCell<Int8Array>>> {
        let ret = unsafe { cbg_StreamFile_GetTempBuffer(self.self_ptr) };
        Int8Array::try_get_from_cache(ret)
    }

    /// 読み込んだファイルのパスを取得します。
    pub(crate) fn get_path(&mut self) -> String {
        let ret = unsafe { cbg_StreamFile_GetPath(self.self_ptr) };
        decode_string(ret)
    }

    /// 再読み込みを行います。
    pub fn reload(&mut self) -> bool {
        let ret = unsafe { cbg_StreamFile_Reload(self.self_ptr) };
        ret
    }

    /// 読み込むファイルのデータサイズを取得します。
    pub fn get_size(&mut self) -> i32 {
        let ret = unsafe { cbg_StreamFile_GetSize(self.self_ptr) };
        ret
    }

    /// 現在読み込んでいるファイル上の位置を取得します。
    pub fn get_current_position(&mut self) -> i32 {
        let ret = unsafe { cbg_StreamFile_GetCurrentPosition(self.self_ptr) };
        ret
    }

    /// 現在読み込んでいるファイルのデータサイズを取得します。
    pub fn get_temp_buffer_size(&mut self) -> i32 {
        let ret = unsafe { cbg_StreamFile_GetTempBufferSize(self.self_ptr) };
        ret
    }

    /// 読み込むファイルがファイルパッケージ内に格納されているかどうかを取得します。
    pub fn get_is_in_package(&mut self) -> bool {
        let ret = unsafe { cbg_StreamFile_GetIsInPackage(self.self_ptr) };
        ret
    }
}

impl Drop for StreamFile {
    fn drop(&mut self) {
        unsafe { cbg_StreamFile_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct StaticFile {
    self_ptr: *mut RawPtr,
}

unsafe impl Send for StaticFile {}
unsafe impl Sync for StaticFile {}

impl StaticFile {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<StaticFile>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Arc::new(Mutex::new(StaticFile { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Arc<Mutex<Self>>> {
        lazy_static! {
            static ref STATICFILE_CACHE: RwLock<HashMap<RawPtrStorage, sync::Weak<Mutex<StaticFile>>>> =
                RwLock::new(HashMap::new());
        }
        let mut hash_map = STATICFILE_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => {
                    return Some(o);
                }
                None => {
                    hash_map.remove(&storage);
                }
            }
        }
        let o = Self::cbg_create_raw(self_ptr)?;
        hash_map.insert(storage, Arc::downgrade(&o));
        Some(o)
    }

    /// 指定ファイルを読み込んだStaticFileの新しいインスタンスを生成します。
    /// # Arguments
    /// * `path` - 読み込むファイルのパス
    pub fn create(path: &str) -> Option<Arc<Mutex<StaticFile>>> {
        let ret = unsafe { cbg_StaticFile_Create(encode_string(&path).as_ptr()) };
        StaticFile::try_get_from_cache(ret)
    }

    /// 読み込んだファイルのデータを取得します。
    pub(crate) fn get_buffer(&mut self) -> Option<Rc<RefCell<Int8Array>>> {
        let ret = unsafe { cbg_StaticFile_GetBuffer(self.self_ptr) };
        Int8Array::try_get_from_cache(ret)
    }

    /// 再読み込みを行います。
    pub fn reload(&mut self) -> bool {
        let ret = unsafe { cbg_StaticFile_Reload(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルのパスを取得します。
    pub fn get_path(&mut self) -> String {
        let ret = unsafe { cbg_StaticFile_GetPath(self.self_ptr) };
        decode_string(ret)
    }

    /// 読み込んだファイルのデータサイズを取得します。
    pub fn get_size(&mut self) -> i32 {
        let ret = unsafe { cbg_StaticFile_GetSize(self.self_ptr) };
        ret
    }

    /// 読み込んだファイルがファイルパッケージ内に格納されているかどうかを取得します。
    pub fn get_is_in_package(&mut self) -> bool {
        let ret = unsafe { cbg_StaticFile_GetIsInPackage(self.self_ptr) };
        ret
    }
}

impl Drop for StaticFile {
    fn drop(&mut self) {
        unsafe { cbg_StaticFile_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct File {
    self_ptr: *mut RawPtr,
}

impl File {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<File>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(File { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static FILE_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<File>>>> = RefCell::new(HashMap::new());
        }
        FILE_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<File>>> {
        let ret = unsafe { cbg_File_GetInstance() };
        File::try_get_from_cache(ret)
    }

    /// ファイル読み込み時に自動的に保管されるディレクトリを追加します。
    /// # Arguments
    /// * `path` - 追加するディレクトリ
    pub fn add_root_directory(&mut self, path: &str) -> bool {
        let ret =
            unsafe { cbg_File_AddRootDirectory(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// ファイルパッケージをパスワード有りで読み込む
    /// # Arguments
    /// * `path` - 読み込むファイルパッケージのパス
    /// * `password` - 読み込むファイルパッケージのパスワード
    pub fn add_root_package_with_password(&mut self, path: &str, password: &str) -> bool {
        let ret = unsafe {
            cbg_File_AddRootPackageWithPassword(
                self.self_ptr,
                encode_string(&path).as_ptr(),
                encode_string(&password).as_ptr(),
            )
        };
        ret
    }

    /// ファイルパッケージをパスワード無しで読み込む
    /// # Arguments
    /// * `path` - 読み込むファイルパッケージのパス
    pub fn add_root_package(&mut self, path: &str) -> bool {
        let ret = unsafe { cbg_File_AddRootPackage(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    /// 追加されたディレクトリやファイルパッケージをすべて削除します。
    pub fn clear_root_directories(&mut self) -> () {
        unsafe { cbg_File_ClearRootDirectories(self.self_ptr) }
    }

    /// 指定したファイルが存在するかどうかを検索します。
    /// # Arguments
    /// * `path` - 存在を確認するファイルのパス
    pub fn exists(&mut self, path: &str) -> bool {
        let ret = unsafe { cbg_File_Exists(self.self_ptr, encode_string(&path).as_ptr()) };
        ret
    }

    ///
    /// # Arguments
    /// * `src_path` -
    /// * `dst_path` -
    pub fn pack(&mut self, src_path: &str, dst_path: &str) -> bool {
        let ret = unsafe {
            cbg_File_Pack(
                self.self_ptr,
                encode_string(&src_path).as_ptr(),
                encode_string(&dst_path).as_ptr(),
            )
        };
        ret
    }

    ///
    /// # Arguments
    /// * `src_path` -
    /// * `dst_path` -
    /// * `password` -
    pub fn pack_with_password(&mut self, src_path: &str, dst_path: &str, password: &str) -> bool {
        let ret = unsafe {
            cbg_File_PackWithPassword(
                self.self_ptr,
                encode_string(&src_path).as_ptr(),
                encode_string(&dst_path).as_ptr(),
                encode_string(&password).as_ptr(),
            )
        };
        ret
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { cbg_File_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Sound {
    self_ptr: *mut RawPtr,
    loop_starting_point: Option<f32>,
    loop_end_point: Option<f32>,
    is_looping_mode: Option<bool>,
}

impl Sound {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Sound>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Sound {
            self_ptr,
            loop_starting_point: None,
            loop_end_point: None,
            is_looping_mode: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static SOUND_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Sound>>>> = RefCell::new(HashMap::new());
        }
        SOUND_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    ///
    /// # Arguments
    /// * `path` -
    /// * `is_decompressed` -
    pub fn load(path: &str, is_decompressed: bool) -> Option<Rc<RefCell<Sound>>> {
        let ret = unsafe { cbg_Sound_Load(encode_string(&path).as_ptr(), is_decompressed) };
        Sound::try_get_from_cache(ret)
    }

    /// 読み込んだファイルのパスを取得します。
    pub(crate) fn get_path(&mut self) -> String {
        let ret = unsafe { cbg_Sound_GetPath(self.self_ptr) };
        decode_string(ret)
    }

    /// 音源を解凍するかどうかを取得する
    pub(crate) fn get_is_decompressed(&mut self) -> bool {
        let ret = unsafe { cbg_Sound_GetIsDecompressed(self.self_ptr) };
        ret
    }

    /// ループ開始地点(秒)を取得または設定します。
    pub fn get_loop_starting_point(&mut self) -> f32 {
        if let Some(value) = self.loop_starting_point.clone() {
            return value;
        }
        let ret = unsafe { cbg_Sound_GetLoopStartingPoint(self.self_ptr) };
        ret
    }
    pub fn set_loop_starting_point(&mut self, value: f32) {
        self.loop_starting_point = Some(value.clone());
        unsafe { cbg_Sound_SetLoopStartingPoint(self.self_ptr, value) }
    }

    /// ループ終了地点(秒)を取得または設定します。
    pub fn get_loop_end_point(&mut self) -> f32 {
        if let Some(value) = self.loop_end_point.clone() {
            return value;
        }
        let ret = unsafe { cbg_Sound_GetLoopEndPoint(self.self_ptr) };
        ret
    }
    pub fn set_loop_end_point(&mut self, value: f32) {
        self.loop_end_point = Some(value.clone());
        unsafe { cbg_Sound_SetLoopEndPoint(self.self_ptr, value) }
    }

    /// ループするかどうかを取得または設定します。
    pub fn get_is_looping_mode(&mut self) -> bool {
        if let Some(value) = self.is_looping_mode.clone() {
            return value;
        }
        let ret = unsafe { cbg_Sound_GetIsLoopingMode(self.self_ptr) };
        ret
    }
    pub fn set_is_looping_mode(&mut self, value: bool) {
        self.is_looping_mode = Some(value.clone());
        unsafe { cbg_Sound_SetIsLoopingMode(self.self_ptr, value) }
    }

    /// 音源の長さ(秒)を取得します。
    pub fn get_length(&mut self) -> f32 {
        let ret = unsafe { cbg_Sound_GetLength(self.self_ptr) };
        ret
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe { cbg_Sound_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct SoundMixer {
    self_ptr: *mut RawPtr,
}

impl SoundMixer {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<SoundMixer>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(SoundMixer { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static SOUNDMIXER_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<SoundMixer>>>> = RefCell::new(HashMap::new());
        }
        SOUNDMIXER_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    pub(crate) fn get_instance() -> Option<Rc<RefCell<SoundMixer>>> {
        let ret = unsafe { cbg_SoundMixer_GetInstance() };
        SoundMixer::try_get_from_cache(ret)
    }

    /// 音を再生します。
    /// # Arguments
    /// * `sound` - 音源データ
    pub fn play(&mut self, sound: &mut Sound) -> i32 {
        let ret = unsafe { cbg_SoundMixer_Play(self.self_ptr, sound.self_ptr) };
        ret
    }

    /// 指定した音が再生中であるかを取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_is_playing(&mut self, id: i32) -> bool {
        let ret = unsafe { cbg_SoundMixer_GetIsPlaying(self.self_ptr, id) };
        ret
    }

    /// 再生中の音を全て停止します。
    pub fn stop_all(&mut self) -> () {
        unsafe { cbg_SoundMixer_StopAll(self.self_ptr) }
    }

    /// 指定した音の再生を停止します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn stop(&mut self, id: i32) -> () {
        unsafe { cbg_SoundMixer_Stop(self.self_ptr, id) }
    }

    /// 指定した音の再生を一時停止します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn pause(&mut self, id: i32) -> () {
        unsafe { cbg_SoundMixer_Pause(self.self_ptr, id) }
    }

    /// 指定した音の再生を再開します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn resume(&mut self, id: i32) -> () {
        unsafe { cbg_SoundMixer_Resume(self.self_ptr, id) }
    }

    /// 指定した音の音量を変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `volume` - 音量(0.0〜1.0
    pub fn set_volume(&mut self, id: i32, volume: f32) -> () {
        unsafe { cbg_SoundMixer_SetVolume(self.self_ptr, id, volume) }
    }

    /// 指定した音をフェードインさせる
    /// # Arguments
    /// * `id` -
    /// * `second` - フェードインに使用する時間(秒)
    pub fn fade_in(&mut self, id: i32, second: f32) -> () {
        unsafe { cbg_SoundMixer_FadeIn(self.self_ptr, id, second) }
    }

    /// 指定した音をフェードアウトさせる
    /// # Arguments
    /// * `id` - 音のID
    /// * `second` - フェードアウトに使用する時間(秒)
    pub fn fade_out(&mut self, id: i32, second: f32) -> () {
        unsafe { cbg_SoundMixer_FadeOut(self.self_ptr, id, second) }
    }

    /// 指定した音の音量を一定時間かけて変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `second` - フェードに使用する時間(秒)
    /// * `targeted_volume` - 変更後の音量(0.0〜1.0)
    pub fn fade(&mut self, id: i32, second: f32, targeted_volume: f32) -> () {
        unsafe { cbg_SoundMixer_Fade(self.self_ptr, id, second, targeted_volume) }
    }

    /// 再生速度を変更するかを取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_is_playback_speed_enabled(&mut self, id: i32) -> bool {
        let ret = unsafe { cbg_SoundMixer_GetIsPlaybackSpeedEnabled(self.self_ptr, id) };
        ret
    }

    /// 再生速度を変更するかを設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `is_playback_speed_enabled` - 再生速度を変更するか?
    pub fn set_is_playback_speed_enabled(
        &mut self,
        id: i32,
        is_playback_speed_enabled: bool,
    ) -> () {
        unsafe {
            cbg_SoundMixer_SetIsPlaybackSpeedEnabled(self.self_ptr, id, is_playback_speed_enabled)
        }
    }

    /// 再生速度を取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_playback_speed(&mut self, id: i32) -> f32 {
        let ret = unsafe { cbg_SoundMixer_GetPlaybackSpeed(self.self_ptr, id) };
        ret
    }

    /// 再生速度を設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `playback_speed` - 変更後の再生速度
    pub fn set_playback_speed(&mut self, id: i32, playback_speed: f32) -> () {
        unsafe { cbg_SoundMixer_SetPlaybackSpeed(self.self_ptr, id, playback_speed) }
    }

    /// パン位置を取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_panning_position(&mut self, id: i32) -> f32 {
        let ret = unsafe { cbg_SoundMixer_GetPanningPosition(self.self_ptr, id) };
        ret
    }

    /// パン位置を設定します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `panning_position` - パン位置 : 0.0で中央, -1.0で左, 1.0で右
    pub fn set_panning_position(&mut self, id: i32, panning_position: f32) -> () {
        unsafe { cbg_SoundMixer_SetPanningPosition(self.self_ptr, id, panning_position) }
    }

    /// 指定した音の再生位置を取得します。
    /// # Arguments
    /// * `id` - 音のID
    pub fn get_playback_position(&mut self, id: i32) -> f32 {
        let ret = unsafe { cbg_SoundMixer_GetPlaybackPosition(self.self_ptr, id) };
        ret
    }

    /// 指定した音の再生位置を変更します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `position` - 再生位置(秒)
    pub fn set_playback_position(&mut self, id: i32, position: f32) -> () {
        unsafe { cbg_SoundMixer_SetPlaybackPosition(self.self_ptr, id, position) }
    }

    /// 再生中の音のスペクトル情報を取得します。
    /// # Arguments
    /// * `id` - 音のID
    /// * `spectrums` - 音のスペクトル情報を格納するための配列
    /// * `window` - フーリエ変換に用いる窓関数
    pub(crate) fn get_spectrum_data(
        &mut self,
        id: i32,
        spectrums: &mut FloatArray,
        window: FFTWindow,
    ) -> () {
        unsafe {
            cbg_SoundMixer_GetSpectrumData(self.self_ptr, id, spectrums.self_ptr, window as i32)
        }
    }
}

impl Drop for SoundMixer {
    fn drop(&mut self) {
        unsafe { cbg_SoundMixer_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub struct Log {
    self_ptr: *mut RawPtr,
}

impl Log {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Log>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Log { self_ptr })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static LOG_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Log>>>> = RefCell::new(HashMap::new());
        }
        LOG_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Log>>> {
        let ret = unsafe { cbg_Log_GetInstance() };
        Log::try_get_from_cache(ret)
    }

    /// ログを出力します。
    pub fn write(&mut self, category: LogCategory, level: LogLevel, message: &str) -> () {
        unsafe {
            cbg_Log_Write(
                self.self_ptr,
                category as i32,
                level as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルTraceでログを出力します。
    pub fn trace(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Trace(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルDebugでログを出力します。
    pub fn debug(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Debug(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルInfoでログを出力します。
    pub fn info(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Info(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルWarningでログを出力します。
    pub fn warn(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Warn(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルErrorでログを出力します。
    pub fn error(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Error(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルCriticalでログを出力します。
    pub fn critical(&mut self, category: LogCategory, message: &str) -> () {
        unsafe {
            cbg_Log_Critical(
                self.self_ptr,
                category as i32,
                encode_string(&message).as_ptr(),
            )
        }
    }

    /// ログレベルを設定します。
    pub fn set_level(&mut self, category: LogCategory, level: LogLevel) -> () {
        unsafe { cbg_Log_SetLevel(self.self_ptr, category as i32, level as i32) }
    }
}

impl Drop for Log {
    fn drop(&mut self) {
        unsafe { cbg_Log_Release(self.self_ptr) };
    }
}

#[derive(Debug)]
pub(crate) struct Window {
    self_ptr: *mut RawPtr,
    title: Option<String>,
}

impl Window {
    #[allow(dead_code)]
    fn cbg_create_raw(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Window>>> {
        if self_ptr == NULLPTR {
            return None;
        }
        Some(Rc::new(RefCell::new(Window {
            self_ptr,
            title: None,
        })))
    }

    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr: *mut RawPtr) -> Option<Rc<RefCell<Self>>> {
        thread_local! {
            static WINDOW_CACHE: RefCell<HashMap<RawPtrStorage, rc::Weak<RefCell<Window>>>> = RefCell::new(HashMap::new());
        }
        WINDOW_CACHE.with(|hash_map| {
            let mut hash_map = hash_map.borrow_mut();
            let storage = RawPtrStorage(self_ptr);
            if let Some(x) = hash_map.get(&storage) {
                match x.upgrade() {
                    Some(o) => {
                        return Some(o);
                    }
                    None => {
                        hash_map.remove(&storage);
                    }
                }
            }
            let o = Self::cbg_create_raw(self_ptr)?;
            hash_map.insert(storage, Rc::downgrade(&o));
            Some(o)
        })
    }

    /// インスタンスを取得します。
    pub(crate) fn get_instance() -> Option<Rc<RefCell<Window>>> {
        let ret = unsafe { cbg_Window_GetInstance() };
        Window::try_get_from_cache(ret)
    }

    pub fn get_title(&mut self) -> String {
        if let Some(value) = self.title.clone() {
            return value;
        }
        let ret = unsafe { cbg_Window_GetTitle(self.self_ptr) };
        decode_string(ret)
    }
    pub fn set_title(&mut self, value: String) {
        self.title = Some(value.clone());
        unsafe { cbg_Window_SetTitle(self.self_ptr, encode_string(&value).as_ptr()) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { cbg_Window_Release(self.self_ptr) };
    }
}
