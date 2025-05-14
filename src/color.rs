#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Green,
    Pink,
    Blue,
    Orange,
}

impl TryFrom<char> for Color {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'G' => Ok(Color::Green),
            'P' => Ok(Color::Pink),
            'B' => Ok(Color::Blue),
            'O' => Ok(Color::Orange),
            _ => Err(format!("Invalid character: {}", c)),
        }
    }
}
