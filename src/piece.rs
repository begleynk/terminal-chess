use Side;
use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub struct Piece {
    repr: u8
}

impl Piece {
    pub fn pack(side: Side, rank: Rank) -> Piece {
        let mut i = match rank {
            Rank::Pawn => 0,
            Rank::Knight => 1,
            Rank::Bishop => 2,
            Rank::Rook => 3,
            Rank::Queen => 4,
            Rank::King => 5
        };

        if side == Side::White {
            i |= 0b1000_0000;
        }

        Piece { repr: i }
    }

    pub fn side(&self) -> Side {
        if self.repr >> 7 == 0 {
            Side::Black
        } else {
            Side::White
        }
    }

    pub fn rank(&self) -> Rank {
        match self.repr << 1 {
            0 => Rank::Pawn,
            2 => Rank::Knight,
            4 => Rank::Bishop,
            6 => Rank::Rook,
            8 => Rank::Queen,
            10 => Rank::King,
            i => panic!(format!("Unknown rank {:08b}", i))
        }
    }

}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.side(), self.rank())
    }
}

#[derive(PartialEq)]
pub enum Rank {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl fmt::Debug for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::Pawn => write!(f, "♙"),
            Rank::Knight => write!(f, "♘"),
            Rank::Bishop => write!(f, "♗"),
            Rank::Rook => write!(f, "♖"),
            Rank::Queen => write!(f, "♕"),
            Rank::King => write!(f, "♔"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_side_in_the_first_bit() {
        let white_pawn = Piece { repr: 0b1000_0000 };
        let black_pawn = Piece { repr: 0b0000_0000 };

        assert_eq!(white_pawn.side(), Side::White);
        assert_eq!(black_pawn.side(), Side::Black);
    }

    #[test]
    fn encodes_rank_in_lower_bits() {
        let pawn   = Piece { repr: 0b1000_0000 };
        let knight = Piece { repr: 0b1000_0001 };
        let bishop = Piece { repr: 0b1000_0010 };
        let rook   = Piece { repr: 0b1000_0011 };
        let queen  = Piece { repr: 0b1000_0100 };
        let king   = Piece { repr: 0b1000_0101 };

        assert_eq!(pawn.rank(),   Rank::Pawn);
        assert_eq!(knight.rank(), Rank::Knight);
        assert_eq!(bishop.rank(), Rank::Bishop);
        assert_eq!(rook.rank(),   Rank::Rook);
        assert_eq!(queen.rank(),  Rank::Queen);
        assert_eq!(king.rank(),   Rank::King);
    }

    #[test]
    fn can_pack_a_piece_from_a_side_and_rank() {
        assert_eq!(Piece::pack(Side::White, Rank::Pawn).rank(), Rank::Pawn);
        assert_eq!(Piece::pack(Side::White, Rank::Pawn).side(), Side::White);
        assert_eq!(Piece::pack(Side::Black, Rank::Pawn).side(), Side::Black);

        assert_eq!(Piece::pack(Side::White, Rank::Knight).rank(), Rank::Knight);
        assert_eq!(Piece::pack(Side::White, Rank::Knight).side(), Side::White);
        assert_eq!(Piece::pack(Side::Black, Rank::Knight).side(), Side::Black);

        assert_eq!(Piece::pack(Side::White, Rank::Bishop).rank(), Rank::Bishop);
        assert_eq!(Piece::pack(Side::White, Rank::Bishop).side(), Side::White);
        assert_eq!(Piece::pack(Side::Black, Rank::Bishop).side(), Side::Black);

        assert_eq!(Piece::pack(Side::White, Rank::Rook).rank(), Rank::Rook);
        assert_eq!(Piece::pack(Side::White, Rank::Rook).side(), Side::White);
        assert_eq!(Piece::pack(Side::Black, Rank::Rook).side(), Side::Black);

        assert_eq!(Piece::pack(Side::White, Rank::Queen).rank(), Rank::Queen);
        assert_eq!(Piece::pack(Side::White, Rank::Queen).side(), Side::White);
        assert_eq!(Piece::pack(Side::Black, Rank::Queen).side(), Side::Black);

        assert_eq!(Piece::pack(Side::White, Rank::King).rank(), Rank::King);
        assert_eq!(Piece::pack(Side::White, Rank::King).side(), Side::White);
        assert_eq!(Piece::pack(Side::Black, Rank::King).side(), Side::Black);
    }
}