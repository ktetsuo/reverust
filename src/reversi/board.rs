use super::cell::Cell;
use super::color::Color;

#[derive(Default)]
pub struct Board {
    cells: [[Cell; 8]; 8],
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    /// 幅を取得する
    pub fn len_x(&self) -> usize {
        self.cells[0].len()
    }
    /// 高さを取得する
    pub fn len_y(&self) -> usize {
        self.cells.len()
    }
    /// ボードの内部か判定する
    pub fn is_inside(&self, x: usize, y: usize) -> bool {
        x < self.len_x() && y < self.len_y()
    }
    /// 色を取得する
    pub fn get_color(&self, x: usize, y: usize) -> Option<Color> {
        if !self.is_inside(x, y) {
            return None;
        }
        self.cells[x][y].get_color()
    }
    /// 石をセットする（裏返しなし, 置いてあっても上書き）
    pub fn set(&mut self, x: usize, y: usize, color: Color) -> bool {
        if !self.is_inside(x, y) {
            return false;
        }
        self.cells[x][y].set(color);
        true
    }
    /// 相対位置を取得する
    fn relative_pos(&self, x: usize, y: usize, diff: &(i32, i32)) -> Option<(usize, usize)> {
        let x2 = x as i32 + diff.0;
        let y2 = y as i32 + diff.1;
        if x2 < 0 || y2 < 0 {
            return None;
        }
        if !self.is_inside(x2 as usize, y2 as usize) {
            return None;
        }
        Some((x2 as usize, y2 as usize))
    }
    /// 8方向
    const DIFFS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    /// 石を置く（裏返しあり）
    pub fn put(&mut self, x: usize, y: usize, color: Color) -> bool {
        if !self.can_put_at(x, y, color) {
            // 置けない
            return false;
        }
        let mut success = false;
        for diff in Self::DIFFS.iter() {
            // 隣の位置を取得
            if let Some(p) = self.relative_pos(x, y, diff) {
                // 隣の位置pがあれば色を取得
                if let Some(next_color) = self.get_color(p.0, p.1) {
                    // 色があれば
                    if next_color == color.rev() {
                        // 色が相手の色だったら
                        // その次から自分の色が見つかるまでループする
                        let mut p1 = p;
                        loop {
                            // その隣の位置を取得
                            if let Some(p2) = self.relative_pos(p1.0, p1.1, diff) {
                                // 色を取得
                                if let Some(next_color) = self.get_color(p2.0, p2.1) {
                                    if next_color == color {
                                        // 自分の石が見つかった
                                        success = true; // 1回でもひっくり返せたら成功
                                                        // p1からpまで遡って自分の色にする
                                        loop {
                                            self.cells[p1.0][p1.1].set(color);
                                            p1 = self
                                                .relative_pos(p1.0, p1.1, &(-diff.0, -diff.1))
                                                .unwrap();
                                            if p1.0 == x && p1.1 == y {
                                                break;
                                            }
                                        }
                                        break; // 次の方向へ
                                    } else {
                                        // 相手の石だったら続き
                                        p1 = p2;
                                    }
                                } else {
                                    // 石がなければ次の方向へ
                                    break;
                                }
                            } else {
                                // 位置がなければ次の方向へ
                                break;
                            }
                        }
                    }
                }
            }
        }
        self.cells[x][y].put(color);
        success
    }
    /// 指定した場所に置けるか？
    pub fn can_put_at(&self, x: usize, y: usize, color: Color) -> bool {
        if !self.is_inside(x, y) {
            // 範囲外
            return false;
        }
        if self.get_color(x, y).is_some() {
            // すでに石が置かれている
            return false;
        }
        for diff in Self::DIFFS.iter() {
            // 隣の位置を取得 ★こういうときに if let 使うのか
            if let Some(mut p) = self.relative_pos(x, y, diff) {
                // 隣の位置があれば色を取得
                if let Some(next_color) = self.get_color(p.0, p.1) {
                    // 色があれば
                    if next_color == color.rev() {
                        // 色が相手の色だったら
                        // その次から自分の色が見つかるまでループする
                        loop {
                            // 隣の位置を取得
                            if let Some(p2) = self.relative_pos(p.0, p.1, diff) {
                                // 色を取得
                                if let Some(next_color) = self.get_color(p2.0, p2.1) {
                                    if next_color == color {
                                        // 自分の石が見つかった
                                        return true;
                                    } else {
                                        // 相手の石だったら続き
                                        p = p2;
                                    }
                                } else {
                                    // 石がなければ次の方向へ
                                    break;
                                }
                            } else {
                                // 位置がなければ次の方向へ
                                break;
                            }
                        }
                    }
                }
            }
        }
        // 置ける場所が見つからなかった
        false
    }
    /// どこかに置けるか？
    pub fn can_put(&self, color: Color) -> bool {
        for y in 0..self.len_y() {
            for x in 0..self.len_x() {
                if !self.can_put_at(x, y, color) {
                    return false;
                }
            }
        }
        true
    }
    pub fn clear(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.clear()
            }
        }
    }
}
