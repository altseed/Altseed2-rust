#[allow(unused_imports)]
use std::os::raw::*;
#[allow(unused_imports)]
use std::ffi::CString;


enum RawPtr { }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum DeviceType {
// }

pub(crate) mod structs {
    #[allow(unused_imports)]
    use super::*;
    #[repr(C)]
    pub(crate) struct Vector2DI {
    }
    
}
#[link(name = "Altseed_Core")]
extern {
    fn cbg_Graphics_GetInstance() -> ();
    
    fn cbg_Graphics_Release(self_ptr : *mut RawPtr) -> ();
    
    fn cbg_Texture2D_Reload(self_ptr : *mut RawPtr) -> ();
    
    fn cbg_Texture2D_GetSize(self_ptr : *mut RawPtr) -> ();
    
    fn cbg_Texture2D_Release(self_ptr : *mut RawPtr) -> ();
    
}


use std::sync::{Arc, Weak, RwLock, Mutex};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct RawPtrStorage(*mut RawPtr);

unsafe impl Send for RawPtrStorage { }
unsafe impl Sync for RawPtrStorage { }

lazy_static! {
    static ref GRAPHICS_CACHE: RwLock<HashMap<RawPtrStorage, Weak<Mutex<Graphics>>>> = RwLock::new(HashMap::new());
    static ref TEXTURE2D_CACHE: RwLock<HashMap<RawPtrStorage, Weak<Mutex<Texture2D>>>> = RwLock::new(HashMap::new());
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Graphics {
    self_ptr : *mut RawPtr,
}

unsafe impl Send for Graphics { }
unsafe impl Sync for Graphics { }

impl Clone for Graphics {
    fn clone(&self) -> Self {
        Graphics {
            self_ptr : self.self_ptr,
        }
    }
}

impl Graphics {
    #[allow(dead_code)]
    fn create(self_ptr : *mut RawPtr) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
        Graphics {
            self_ptr,
        }
        ))
    }
    
    
    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr : *mut RawPtr) -> Arc<Mutex<Self>> {
        let mut hash_map = GRAPHICS_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => { return o; },
                None => { hash_map.remove(&storage); },
            }
        }
        let o = Self::create(self_ptr);
        hash_map.insert(storage, Arc::downgrade(&o));
        o
    }
    
    pub fn get_instance() -> () {
        unsafe { cbg_Graphics_GetInstance() }
    }
    
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe { cbg_Graphics_Release(self.self_ptr) };
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Texture2D {
    self_ptr : *mut RawPtr,
}

unsafe impl Send for Texture2D { }
unsafe impl Sync for Texture2D { }

impl Clone for Texture2D {
    fn clone(&self) -> Self {
        Texture2D {
            self_ptr : self.self_ptr,
        }
    }
}

impl Texture2D {
    #[allow(dead_code)]
    fn create(self_ptr : *mut RawPtr) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
        Texture2D {
            self_ptr,
        }
        ))
    }
    
    
    #[allow(dead_code)]
    fn try_get_from_cache(self_ptr : *mut RawPtr) -> Arc<Mutex<Self>> {
        let mut hash_map = TEXTURE2D_CACHE.write().unwrap();
        let storage = RawPtrStorage(self_ptr);
        if let Some(x) = hash_map.get(&storage) {
            match x.upgrade() {
                Some(o) => { return o; },
                None => { hash_map.remove(&storage); },
            }
        }
        let o = Self::create(self_ptr);
        hash_map.insert(storage, Arc::downgrade(&o));
        o
    }
    
    pub fn reload(&mut self) -> () {
        unsafe { cbg_Texture2D_Reload(self.self_ptr) }
    }
    
    pub fn get_size(&mut self) -> () {
        unsafe { cbg_Texture2D_GetSize(self.self_ptr) }
    }
    
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { cbg_Texture2D_Release(self.self_ptr) };
    }
}

