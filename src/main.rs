pub mod core;

fn main() {
    let board_res = core::board::BitBoard::from_fen("rnbqkbnr/pppppppp/8/7p/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    match board_res {
        Ok(board) => println!("{board}"),
        Err(err) => println!("{err}")
    }
}
