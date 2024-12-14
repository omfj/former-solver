#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Orange,
    Pink,
    Blue,
    Green,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {
    pub color: Option<Color>,
}

impl Shape {
    /// Create a new shape with the given color.
    pub fn new(color: Color) -> Shape {
        Shape { color: Some(color) }
    }
}
