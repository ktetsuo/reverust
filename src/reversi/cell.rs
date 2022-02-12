use super::color::Color;

pub struct Cell {
    color: Option<Color>,
}

impl Cell {
    /// 石を取り除く
    pub fn clear(&mut self) {
        self.color = None;
    }
    /// 石を置く
    pub fn put(&mut self, color: Color) -> bool {
        if self.color.is_some() {
            // すでに置かれている
            return false;
        }
        self.color = Some(color);
        true
    }
    /// 石をセットする（置いてあっても上書き）
    pub fn set(&mut self, color: Color) {
        self.color = Some(color);
    }
    /// 色を取得する
    pub fn get_color(&self) -> Option<Color> {
        self.color
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.color {
            None => write!(f, ". "),
            Some(color) => write!(f, "{}", color),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell { color: None }
    }
}
