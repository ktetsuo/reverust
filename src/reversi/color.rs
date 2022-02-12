#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    /// 反対の面
    pub fn rev(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "@ "),
            Color::White => write!(f, "O "),
        }
    }
}
