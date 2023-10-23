#[derive(Debug)]
pub enum Color {
    White,
    Red,
    Blue,
    Yellow,
    Green,
    Cyan,
    Magenta,
    Black,
}

impl Color {
    pub fn to_ansi_code(&self) -> &'static str {
        match self {
            Color::White => "\x1b[37m",
            Color::Red => "\x1b[31m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m",
            Color::Green => "\x1b[32m",
            Color::Cyan => "\x1b[36m",
            Color::Magenta => "\x1b[35m",
            Color::Black => "\x1b[30m",
        }
    }
}
