#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Entity {
    index: u32,
    version: u32,
}

pub trait Component {}

#[derive(Debug)]
pub struct Memoried<T> {
    value: T,
    prev: Option<T>,
}

impl<T: PartialEq + Clone + Copy> Memoried<T> {
    pub fn new(value: T) -> Self {
        Memoried { value, prev: None }
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn is_updated(&self) -> bool {
        match &self.prev {
            Some(x) if *x == self.value => false,
            _ => true,
        }
    }

    pub fn update(&mut self) -> bool {
        match &self.prev {
            None => {
                self.prev = Some(self.value.clone());
                true
            }
            Some(x) if *x != self.value => {
                self.prev = Some(self.value.clone());
                true
            }
            Some(_) => false,
        }
    }

    pub fn reset(&mut self) {
        self.prev = None;
    }
}

pub mod camera;
pub mod collider;
pub mod drawn;
pub mod drawn_kind;
pub mod sorted;
pub mod storage;
