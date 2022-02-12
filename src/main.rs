mod reversi;

fn main() {
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
}
