//! Bitboards and stuff
//!

use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum BoardError {
    #[error("Error CB001: Invalid character in given FEN '{0}'")]
    FENInvalidChar(char),

    #[error("Error CB002: Rank {0} incomplete in given FEN")]
    FENIncompleteRank(u8),

    #[error("Error CB003: Rank {0} too long in given FEN")]
    FENRankTooLong(u8),

    #[error("Error CB004: Too many ranks in given FEN")]
    FENTooManyRanks,

    #[error("Error CB005: Not enough ranks in given FEN")]
    FENNotEnoughRanks,

    #[error("Error CB006: Too many pieces in the given FEN")]
    FENTooManyPieces,

    #[error("Error CB007: Number of white kings in given FEN not exactly one")]
    FENInvalidWhiteKing,

    #[error("Error CB008: Number of black kings in given FEN not exactly one")]
    FENInvalidBlackKing,
}

fn piece_index(c: char) -> usize {
    match c {
        'K' => 0,
        'Q' => 1,
        'R' => 2,
        'N' => 3,
        'B' => 4,
        'P' => 5,

        'k' => 6,
        'q' => 7,
        'r' => 8,
        'n' => 9,
        'b' => 10,
        'p' => 11,

        _ => unreachable!(),
    }
}

fn index_piece(i: usize) -> char {
    match i {
        0 => 'K',
        1 => 'Q',
        2 => 'R',
        3 => 'N',
        4 => 'B',
        5 => 'P',

        6 => 'k',
        7 => 'q',
        8 => 'r',
        9 => 'n',
        10 => 'b',
        11 => 'p',

        _ => unreachable!(),
    }
}

/// # Implementation of BitBoard
///
/// ## Index of array:
/// 0: White king
/// 1: White queen
/// 2: White rooks
/// 3: White knights
/// 4: White bishops
/// 5: White pawns
///
/// +6 each index for black variants
pub struct BitBoard(pub [u64; 12]);

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::from("");

        for rank in (0..8).rev() {
            for file in 0..8 {
                let mut found = false;

                for piece in 0..12 {
                    if self.0[piece] & (1u64 << (rank * 8 + file)) != 0 {
                        out.push(index_piece(piece));
                        out.push(' ');
                        found = true;
                    }
                }

                if !found {
                    out.push_str(". ");
                }
            }

            out.push('\n');
        }

        write!(f, "{out}")
    }
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard([0; 12])
    }

    /// # Build a BitBoard from an FEN
    ///
    /// Validates the input FEN
    ///
    /// ## Checks for:
    /// 1. One and only one white and black king each
    /// 2. At minimum two pieces on the board, at most 32 pieces on the board
    /// 3. Exactly 8 squares in each rank
    /// 4. Exactly 8 ranks
    pub fn from_fen(fen: &str) -> Result<BitBoard, BoardError> {
        let mut board = BitBoard::new();

        let mut file = 0;
        let mut rank = 7;
        let mut piece_count = 0;
        let mut white_king = 0;
        let mut black_king = 0;

        for c in fen.chars() {
            match c {
                ' ' => break,

                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as u8;

                    if file > 8 {
                        return Err(BoardError::FENRankTooLong(rank + 1));
                    }
                }

                'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                    board.0[piece_index(c)] |= 1u64 << (rank * 8 + file);

                    if c == 'K' {
                        white_king += 1;
                    }

                    if c == 'k' {
                        black_king += 1;
                    }

                    piece_count += 1;

                    file += 1;
                }

                '/' => {
                    if file != 8 {
                        return Err(BoardError::FENIncompleteRank(rank + 1));
                    }

                    if rank == 0 {
                        return Err(BoardError::FENTooManyRanks);
                    }

                    rank -= 1;
                    file = 0;
                }

                c => {
                    return Err(BoardError::FENInvalidChar(c));
                }
            }
        }

        if file != 8 {
            return Err(BoardError::FENIncompleteRank(1));
        } else if rank != 0 {
            return Err(BoardError::FENNotEnoughRanks);
        } else if piece_count > 32 {
            return Err(BoardError::FENTooManyPieces);
        } else if white_king != 1 {
            return Err(BoardError::FENInvalidWhiteKing);
        } else if black_king != 1 {
            return Err(BoardError::FENInvalidBlackKing);
        }

        Ok(board)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen_starting_position() {
        assert!(BitBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").is_ok());
    }

    #[test]
    fn fen_invalid_character() {
        assert!(matches!(
            BitBoard::from_fen("rnbxkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            Err(BoardError::FENInvalidChar('x'))
        ));
    }

    #[test]
    fn fen_incomplete_rank() {
        assert!(matches!(
            BitBoard::from_fen("2r5/1rPR2p1/K1R1pk1/7p/P7/7P/6P1/8 b - - 2 46"), // From a real game
            Err(BoardError::FENIncompleteRank(6))
        ));
    }

    #[test]
    fn fen_rank_too_long() {
        assert!(matches!(
            BitBoard::from_fen("2r5/1rPR2p1/K1R2pk1/7p/P8/7P/6P1/8 b - - 2 46"),
            Err(BoardError::FENRankTooLong(4))
        ));
    }

    #[test]
    fn fen_too_many_ranks() {
        assert!(matches!(
            BitBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            Err(BoardError::FENTooManyRanks)
        ));
    }

    #[test]
    fn fen_not_enough_ranks() {
        assert!(matches!(
            BitBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR"),
            Err(BoardError::FENNotEnoughRanks)
        ));
    }

    #[test]
    fn fen_too_many_pieces() {
        assert!(matches!(
            BitBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPPP/RNBQKBNR"),
            Err(BoardError::FENTooManyPieces)
        ));
    }

    #[test]
    fn fen_invalid_white_king() {
        assert!(matches!(
            BitBoard::from_fen("2r5/1rPR2p1/K1R2pk1/7p/P7/7P/6K1/8 b - - 2 46"),
            Err(BoardError::FENInvalidWhiteKing)
        ));
    }

    #[test]
    fn fen_invalid_black_king() {
        assert!(matches!(
            BitBoard::from_fen("2r5/1rPR2p1/K1R2pk1/7p/P7/7P/6k1/8 b - - 2 46"),
            Err(BoardError::FENInvalidBlackKing)
        ));
    }
}
