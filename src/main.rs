use engine::Engine;

mod piece;
mod board;
mod utils;
mod engine;

fn main() {
    let v = vec![
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
        "w".to_string(),
        "KQkq".to_string(),
        "-".to_string(),
        "0".to_string(),
        "1".to_string(),
    ];

    let engine = Engine::from_fen(&v);
    println!("{}", engine.is_white_turn);
}
