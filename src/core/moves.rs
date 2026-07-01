//! Structs related to moves
//!

#[derive(Debug, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

/// ## Format:
/// ```
/// 00 XXXXXX
/// ```
///
/// The six bits represent a square. Enumeration starts at a1 and ends as h8 in rank-major order.
/// - a1 === 0
/// - b1 === 1
/// - c1 === 2
/// - a2 === 8
/// - h8 === 63

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Square(u8);

impl Square {
    pub fn from(a: u8) -> Option<Square> {
        // Only 0-63 are valid square indices
        if a >= 64 { None } else { Some(Square(a)) }
    }

    pub fn get_human_readable(self) -> (char, u8) {
        // File is basically self % 8, with some ASCII thingy to make it a to h
        // Rank is self / 8. +1 to make it 1 to 8
        ((b'a' + (self.0 % 8)) as char, self.0 / 8 + 1)
    }
}

/// ## Format:
///
/// ```
/// XXXX XXXXXX XXXXXX
///    1      2      3
/// ```
///
/// 1.  Special flags
///     - `0`: Normal move
///     - `1`: Double pawn push
///     - `2`: Kingside Castle
///     - `3`: Queenside Castle
///     - `4`: Capture
///     - `5`: En Passant
///     - `8`: Knight promotion
///     - `9`: Bishop promotion
///     - `10`: Rook promotion
///     - `11`: Queen promotion
///     - `12` - `15`: Similar, but also means capture
///
///
/// 2.  To square
/// 3.  From square
///
/// That combination of flag pattern might look weird, but consider
///
/// Notice that capture is when:
/// - 4: 0100
/// - 5: 0101
/// - 12 - 15: 11XX
///
/// In no other combination, X1XX is found, hence `flags & 0x4 != 0` becomes a simple check for captures
///
/// Similarly, 1XXX is for promotion, so `flags & 0x8 != 0` is a simple check
///

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move(u16);

impl Move {
    pub fn from(a: u16) -> Move {
        Move(a)
    }

    fn get_flags(self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }

    pub fn is_capture(self) -> bool {
        self.get_flags() & 0x4 != 0
    }

    pub fn is_promotion(self) -> bool {
        self.get_flags() & 0x8 != 0
    }

    pub fn is_double_pawn_push(self) -> bool {
        self.get_flags() == 1
    }

    pub fn is_en_passant(self) -> bool {
        self.get_flags() == 5
    }

    pub fn is_kingside_castle(self) -> bool {
        self.get_flags() == 2
    }

    pub fn is_queenside_castle(self) -> bool {
        self.get_flags() == 3
    }

    pub fn get_promotion_piece(self) -> Option<Piece> {
        let test = self.get_flags() & 0b1011;

        if test == 8 {
            Some(Piece::Knight)
        } else if test == 9 {
            Some(Piece::Bishop)
        } else if test == 10 {
            Some(Piece::Rook)
        } else if test == 11 {
            Some(Piece::Queen)
        } else {
            None
        }
    }

    pub fn from_square(self) -> Option<Square> {
        Square::from((self.0 & 0x3F) as u8)
    }

    pub fn to_square(self) -> Option<Square> {
        Square::from((self.0 >> 6 & 0x3F) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_test_1() {
        let mut test = Square::from(0);
        assert_eq!(test, Some(Square(0)));

        test = Square::from(72);
        assert_eq!(test, None);
    }

    #[test]
    fn square_test_2() {
        let mut test = Square(0);
        assert_eq!(test.get_human_readable(), ('a', 1));

        test = Square(1);
        assert_eq!(test.get_human_readable(), ('b', 1));

        test = Square(58);
        assert_eq!(test.get_human_readable(), ('c', 8));

        test = Square(21);
        assert_eq!(test.get_human_readable(), ('f', 3));
    }

    #[test]
    fn move_test_1() {
        let mut test = Move::from(0b0000_000000_000000);

        assert_eq!(test.is_capture(), false);
        assert_eq!(test.is_double_pawn_push(), false);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), false);
        assert_eq!(test.get_promotion_piece(), None);

        test = Move::from(0b0001_000000_000000);

        assert_eq!(test.is_capture(), false);
        assert_eq!(test.is_double_pawn_push(), true);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), false);
        assert_eq!(test.get_promotion_piece(), None);

        test = Move::from(0b0001_000000_000000);

        assert_eq!(test.is_capture(), false);
        assert_eq!(test.is_double_pawn_push(), true);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), false);
        assert_eq!(test.get_promotion_piece(), None);

        test = Move::from(0b0010_000000_000000);

        assert_eq!(test.is_capture(), false);
        assert_eq!(test.is_double_pawn_push(), false);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), true);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), false);
        assert_eq!(test.get_promotion_piece(), None);

        test = Move::from(0b0011_000000_000000);

        assert_eq!(test.is_capture(), false);
        assert_eq!(test.is_double_pawn_push(), false);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), true);
        assert_eq!(test.is_promotion(), false);
        assert_eq!(test.get_promotion_piece(), None);

        test = Move::from(0b0100_000000_000000);

        assert_eq!(test.is_capture(), true);
        assert_eq!(test.is_double_pawn_push(), false);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), false);
        assert_eq!(test.get_promotion_piece(), None);

        test = Move::from(0b0101_000000_000000);

        assert_eq!(test.is_capture(), true);
        assert_eq!(test.is_double_pawn_push(), false);
        assert_eq!(test.is_en_passant(), true);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), false);
        assert_eq!(test.get_promotion_piece(), None);

        test = Move::from(0b1000_000000_000000);

        assert_eq!(test.is_capture(), false);
        assert_eq!(test.is_double_pawn_push(), false);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), true);
        assert_eq!(test.get_promotion_piece(), Some(Piece::Knight));

        test = Move::from(0b1110_000000_000000);

        assert_eq!(test.is_capture(), true);
        assert_eq!(test.is_double_pawn_push(), false);
        assert_eq!(test.is_en_passant(), false);
        assert_eq!(test.is_kingside_castle(), false);
        assert_eq!(test.is_queenside_castle(), false);
        assert_eq!(test.is_promotion(), true);
        assert_eq!(test.get_promotion_piece(), Some(Piece::Rook));
    }

    #[test]
    fn move_test_2() {
        let test = Move::from(0b0001_011100_001100);

        assert_eq!(
            test.to_square().expect("Error").get_human_readable(),
            ('e', 4)
        );

        assert_eq!(
            test.from_square().expect("Error").get_human_readable(),
            ('e', 2)
        );
    }
}
