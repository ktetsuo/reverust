use super::board::Board;
use super::color::Color;

pub struct Context {
    board: Board,
    turn: Color,
}

impl Context {
    /// 盤面を初期化する
    pub fn init(&mut self) {
        self.board.clear();
        self.board.set(3, 3, Color::Black);
        self.board.set(4, 4, Color::Black);
        self.board.set(3, 4, Color::White);
        self.board.set(4, 3, Color::White);
    }
    /// どちらのターンか取得する
    pub fn get_turn(&self) -> Color {
        self.turn
    }
    /// 現在のターンの色がどこかに置けるか？
    pub fn can_put(&self) -> bool {
        self.board.can_put(self.turn)
    }
    /// 現在のターンの色が指定した場所に置けるか？
    pub fn can_put_at(&self, x: usize, y: usize) -> bool {
        self.board.can_put_at(x, y, self.turn)
    }
    /// 石を置く（裏返しあり）
    pub fn put(&mut self, x: usize, y: usize) -> bool {
        if !self.board.put(x, y, self.turn) {
            return false;
        }
        self.turn = self.turn.rev();
        true
    }
}
impl Default for Context {
    fn default() -> Self {
        Context {
            board: Default::default(),
            turn: Color::Black,
        }
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.board)?;
        write!(f, "Turn: {:?}", self.turn)
    }
}
