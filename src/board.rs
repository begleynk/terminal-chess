use piece::{Piece, Rank};
use Side;
use fmt;

#[derive(PartialEq)]
pub struct Board {
    pub data: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn default() -> Board {
        Board {
            data: [
                [
                    Some(Piece::pack(Side::White, Rank::Rook)),
                    Some(Piece::pack(Side::White, Rank::Knight)),
                    Some(Piece::pack(Side::White, Rank::Bishop)),
                    Some(Piece::pack(Side::White, Rank::Queen)),
                    Some(Piece::pack(Side::White, Rank::King)),
                    Some(Piece::pack(Side::White, Rank::Bishop)),
                    Some(Piece::pack(Side::White, Rank::Knight)),
                    Some(Piece::pack(Side::White, Rank::Rook)),
                ],
                [
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                    Some(Piece::pack(Side::White, Rank::Pawn)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                    Some(Piece::pack(Side::Black, Rank::Pawn)),
                ],
                [
                    Some(Piece::pack(Side::Black, Rank::Rook)),
                    Some(Piece::pack(Side::Black, Rank::Knight)),
                    Some(Piece::pack(Side::Black, Rank::Bishop)),
                    Some(Piece::pack(Side::Black, Rank::Queen)),
                    Some(Piece::pack(Side::Black, Rank::King)),
                    Some(Piece::pack(Side::Black, Rank::Bishop)),
                    Some(Piece::pack(Side::Black, Rank::Knight)),
                    Some(Piece::pack(Side::Black, Rank::Rook)),
                ],
            ],
        }
    }
}

fn format_pos(pos: &Option<Piece>) -> String {
    match *pos {
        Some(ref piece) => format!("{:?}", piece),
        None => format!("  "),
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n   ---------------------------------------")?;
        write!(f, "\n8 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[7][0]), format_pos(&self.data[7][1]), format_pos(&self.data[7][2]), format_pos(&self.data[7][3]), format_pos(&self.data[7][4]), format_pos(&self.data[7][5]), format_pos(&self.data[7][6]), format_pos(&self.data[7][7]))?;
        write!(f, "\n  |---------------------------------------|")?;
        write!(f, "\n7 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[6][0]), format_pos(&self.data[6][1]), format_pos(&self.data[6][2]), format_pos(&self.data[6][3]), format_pos(&self.data[6][4]), format_pos(&self.data[6][5]), format_pos(&self.data[6][6]), format_pos(&self.data[6][7]))?;
        write!(f, "\n  |---------------------------------------|")?;
        write!(f, "\n6 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[5][0]), format_pos(&self.data[5][1]), format_pos(&self.data[5][2]), format_pos(&self.data[5][3]), format_pos(&self.data[5][4]), format_pos(&self.data[5][5]), format_pos(&self.data[5][6]), format_pos(&self.data[5][7]))?;
        write!(f, "\n  |---------------------------------------|")?;
        write!(f, "\n5 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[4][0]), format_pos(&self.data[4][1]), format_pos(&self.data[4][2]), format_pos(&self.data[4][3]), format_pos(&self.data[4][4]), format_pos(&self.data[4][5]), format_pos(&self.data[4][6]), format_pos(&self.data[4][7]))?;
        write!(f, "\n  |---------------------------------------|")?;
        write!(f, "\n4 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[3][0]), format_pos(&self.data[3][1]), format_pos(&self.data[3][2]), format_pos(&self.data[3][3]), format_pos(&self.data[3][4]), format_pos(&self.data[3][5]), format_pos(&self.data[3][6]), format_pos(&self.data[3][7]))?;
        write!(f, "\n  |---------------------------------------|")?;
        write!(f, "\n3 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[2][0]), format_pos(&self.data[2][1]), format_pos(&self.data[2][2]), format_pos(&self.data[2][3]), format_pos(&self.data[2][4]), format_pos(&self.data[2][5]), format_pos(&self.data[2][6]), format_pos(&self.data[2][7]))?;
        write!(f, "\n  |---------------------------------------|")?;
        write!(f, "\n2 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[1][0]), format_pos(&self.data[1][1]), format_pos(&self.data[1][2]), format_pos(&self.data[1][3]), format_pos(&self.data[1][4]), format_pos(&self.data[1][5]), format_pos(&self.data[1][6]), format_pos(&self.data[1][7]))?;
        write!(f, "\n  |---------------------------------------|")?;
        write!(f, "\n1 | {} | {} | {} | {} | {} | {} | {} | {} |", format_pos(&self.data[0][0]), format_pos(&self.data[0][1]), format_pos(&self.data[0][2]), format_pos(&self.data[0][3]), format_pos(&self.data[0][4]), format_pos(&self.data[0][5]), format_pos(&self.data[0][6]), format_pos(&self.data[0][7]))?;
        write!(f, "\n   ---------------------------------------")?;
        write!(f, "\n     A    B    C    D    E    F    G    H\n")
    }
}