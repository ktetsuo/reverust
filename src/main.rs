mod reversi;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::input::mouse::MouseButton;
use ggez::{timer, Context, GameResult};

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 400.0;
const CELL_SIZE: f32 = 30.0;
const STONE_SIZE: f32 = 20.0;
const BOARD_SIZE: f32 = CELL_SIZE * 8.0;
const BOARD_X: f32 = 100.0;
const BOARD_Y: f32 = 100.0;

struct MainState {
    game: reversi::context::Context,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut s = MainState {
            game: Default::default(),
        };
        s.game.init();
        Ok(s)
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // 背景色
        const BG_COLOR: Color = Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };
        // ボードの色
        const BOARD_COLOR: Color = Color {
            r: 0.0,
            g: 0.5,
            b: 0.0,
            a: 1.0,
        };
        // 背景色で画面クリア
        graphics::clear(ctx, graphics::Color::BLACK);
        let back_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, WIDTH, HEIGHT),
            BG_COLOR,
        )?;
        graphics::draw(ctx, &back_rect, (glam::Vec2::new(0.0, 0.0),))?;
        // ボードの四角
        let board_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, BOARD_SIZE, BOARD_SIZE),
            BOARD_COLOR,
        )?;
        graphics::draw(ctx, &board_rect, (glam::Vec2::new(BOARD_X, BOARD_Y),))?;
        // 白の石
        let circle_white = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            glam::Vec2::new(0.0, 0.0),
            STONE_SIZE / 2.0,
            0.1,
            Color::WHITE,
        )?;
        // 黒の石
        let circle_black = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            glam::Vec2::new(0.0, 0.0),
            STONE_SIZE / 2.0,
            0.1,
            Color::BLACK,
        )?;
        for y in 0..8 {
            for x in 0..8 {
                let xf = x as f32;
                let yf = y as f32;
                if let Some(color) = self.game.get_board().get_color(x, y) {
                    let circle_stone = match color {
                        reversi::color::Color::White => &circle_white,
                        reversi::color::Color::Black => &circle_black,
                    };
                    graphics::draw(
                        ctx,
                        circle_stone,
                        (glam::Vec2::new(
                            xf * CELL_SIZE + BOARD_X + CELL_SIZE / 2.0,
                            yf * CELL_SIZE + BOARD_Y + CELL_SIZE / 2.0,
                        ),),
                    )?;
                }
            }
        }
        // 縦の線
        let vline = graphics::Mesh::new_line(
            ctx,
            &[glam::Vec2::new(0.0, 0.0), glam::Vec2::new(0.0, BOARD_SIZE)],
            2.0,
            Color::BLACK,
        )?;
        for x in 0..9 {
            let x = x as f32;
            graphics::draw(
                ctx,
                &vline,
                (glam::Vec2::new(BOARD_X + CELL_SIZE * x, BOARD_Y),),
            )?;
        }
        // 横の線
        let hline = graphics::Mesh::new_line(
            ctx,
            &[glam::Vec2::new(0.0, 0.0), glam::Vec2::new(BOARD_SIZE, 0.0)],
            2.0,
            Color::BLACK,
        )?;
        for y in 0..9 {
            let y = y as f32;
            graphics::draw(
                ctx,
                &hline,
                (glam::Vec2::new(BOARD_X, BOARD_Y + CELL_SIZE * y),),
            )?;
        }
        // 今のターン
        let turn_stone_circle = match self.game.get_turn() {
            reversi::color::Color::White => &circle_white,
            reversi::color::Color::Black => &circle_black,
        };
        graphics::draw(ctx, turn_stone_circle, (glam::Vec2::new(30.0, 30.0),))?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        // マウスボタンが押下された時のイベント
        println!("mouse down (physical) {}, {}", x, y);
        // x,yは実画面上の座標値なのでスケールを加味した論理座標に変換する
        let (w, h) = graphics::drawable_size(ctx);
        let s_w = w / WIDTH;
        let s_h = h / HEIGHT;
        let s = if s_w < s_h { s_w } else { s_h };
        let x_logical = x / s;
        let y_logical = y / s;
        println!("mouse down (logical) {}, {}", x_logical, y_logical);
        for x in 0..8 {
            for y in 0..8 {
                let xf = x as f32;
                let yf = y as f32;
                let r = graphics::Rect {
                    x: xf * CELL_SIZE + BOARD_X,
                    y: yf * CELL_SIZE + BOARD_Y,
                    w: CELL_SIZE,
                    h: CELL_SIZE,
                };
                let p = glam::Vec2::new(x_logical as f32, y_logical as f32);
                if r.contains(p) {
                    self.game.put(x, y);
                }
            }
        }
    }
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        // リサイズ時のイベント
        println!("resize {},{}", width, height);
        // 短い辺に合わせてスケーリングする
        let s_w = width / WIDTH;
        let s_h = height / HEIGHT;
        let s = if s_w < s_h { s_w } else { s_h };
        let r: graphics::Rect = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: width / s,
            h: height / s,
        };
        graphics::set_screen_coordinates(ctx, r).unwrap();
    }
}

fn main() -> GameResult {
    println!("Start");
    let mut game: reversi::context::Context = Default::default();
    game.init();
    const POINTS: [(usize, usize); 10] = [
        (2, 4),
        (2, 5),
        (3, 5),
        (2, 3),
        (1, 4),
        (4, 5),
        (5, 4),
        (5, 3),
        (5, 2),
        (0, 4),
    ];
    println!("{}", game);
    for p in POINTS.iter() {
        if game.put(p.0, p.1) {
            println!("put ({}, {})", p.0, p.1);
            println!("{}", game);
        } else {
            println!("can not put ({}, {})", p.0, p.1);
        }
    }
    let ws = ggez::conf::WindowSetup {
        title: "reverust".to_owned(),
        icon: "".to_owned(),
        vsync: true,
        srgb: true,
        samples: ggez::conf::NumSamples::Four,
    };
    let wm = ggez::conf::WindowMode {
        width: WIDTH,
        height: HEIGHT,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
        visible: true,
        resize_on_scale_factor_change: false,
    };
    let cb = ggez::ContextBuilder::new("reverust", "ktetsuo")
        .window_setup(ws)
        .window_mode(wm);
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
