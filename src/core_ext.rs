use crate::auto_generated_core_binding::{ButtonState, Keyboard, Keys};

impl Keyboard {
    pub fn is_free(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Free
    }

    pub fn is_push(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Push
    }

    pub fn is_hold(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Hold
    }

    pub fn is_release(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Release
    }
}
