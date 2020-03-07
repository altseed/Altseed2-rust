#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8,
}

impl From<super::Color> for Color {
    fn from(item: super::Color) -> Self {
        Self {
            r : item.r,
            g : item.g,
            b : item.b,
            a : item.a,
        }
    }
}

impl Into<super::Color> for Color {
    fn into(self) -> super::Color {
        super::Color {
            r : self.r,
            g : self.g,
            b : self.b,
            a : self.a,
        }
    }
}