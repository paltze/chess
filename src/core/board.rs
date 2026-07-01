//! Bitboards and stuff
//!

pub struct BitBoard([u64; 12]);

impl BitBoard {
    fn new() -> BitBoard {
        BitBoard([0; 12])
    }
}
