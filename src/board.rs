use piece::{Piece, Rank};
use Side;
use fmt;

#[derive(PartialEq, Clone)]
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

    pub fn empty() -> Board {
        Board {
            data: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None]
            ]
        }
    }

    pub fn piece_at(&self, coordinate: Coordinate) -> &Option<Piece> {
        &self.data[coordinate.row()][coordinate.column()]
    }

    pub fn pieces_with_coordinates(&self) -> Vec<(Coordinate, Piece)> {
        self.data.into_iter()
                 .flat_map(|x| x)
                 .enumerate()
                 .filter_map(|(index, piece)| match *piece {
                     Some(p) => Some((Coordinate::new(index / 8, index % 8), p)),
                     None => None
                 })
                 .collect()
    }

    pub fn rows(&self) -> &[[Option<Piece>; 8]] {
        &self.data
    }

    pub fn is_empty(&self, coordinate: Coordinate) -> bool {
        match *self.piece_at(coordinate) {
            None => true,
            _ => false
        }
    }

    pub fn update(&mut self, coordinate: &Coordinate, piece: Option<Piece>) -> Result<(), String> {
        coordinate.check()?;

        self.data[coordinate.row()][coordinate.column()] = piece;

        Ok(())
    }

    pub fn find_pieces(&self, target_piece: Piece) -> Vec<Coordinate> {
        self.pieces_with_coordinates()
            .into_iter()
            .filter(|&(_coordinate, piece)| piece == target_piece)
            .map(|(coordinate, _piece)| coordinate )
            .collect()
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    pub fn new(row: usize, column: usize) -> Coordinate {
        Coordinate { row, column }
    }

    pub fn new_safe(row: usize, column: usize) -> Result<Coordinate, String> {
        // NOTE: usize cannot be negative
        if row <= 7 && column <= 7 {
            Ok(Coordinate::new(row, column))
        } else {
            Err("Invalid coordinates".to_string())
        }
    }

    pub fn from_human(string: String) -> Result<Coordinate, String> {
        if string.len() != 2 {
            return Err("Coordinate string must be 2 characters long.".to_string())
        }

        let mut chars = string.chars();

        let column = match chars.next().unwrap().to_string().as_ref() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => return Err(format!("Bad coordinate {}", string)),
        };

        let row = match chars.next().unwrap().to_string().parse::<usize>() {
            Ok(r) => r - 1,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Coordinate { row, column })
    }

    pub fn check(&self) -> Result<(), String> {
        if self.row <= 7 && self.column <= 7 {
            Ok(())
        } else {
            Err(format!("Invalid board coordinates: {}, {}", self.row, self.column))
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn to_human(&self) -> String {
        let col = match self.column {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!(format!("Bad column index in coordinates: {}", self.column)),
        };

        format!("{}{}", col, self.row + 1)
    }
}
impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coordinate {{ row: {}, column: {}, (notation: {}) }}", self.row(), self.column(), self.to_human())
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn coordinates_can_be_created_from_human_readable_strings() {
        // Note we store 0-indexed

        let coord = Coordinate::from_human("b5".to_owned()).unwrap();
        assert_eq!(coord.row(), 4, "Incorrect row index parserd");
        assert_eq!(coord.column(), 1, "Incorrect column index parserd");

        let coord = Coordinate::from_human("a1".to_owned()).unwrap();
        assert_eq!(coord.row(), 0, "Incorrect row index parserd");
        assert_eq!(coord.column(), 0, "Incorrect column index parserd");

        let coord = Coordinate::from_human("h8".to_owned()).unwrap();
        assert_eq!(coord.row(), 7, "Incorrect row index parserd");
        assert_eq!(coord.column(), 7, "Incorrect column index parserd");
    }

    #[test]
    fn pieces_are_at_correct_starting_position() {
        let board = Board::default();

        assert_eq!(*board.piece_at(coord!("a1")), Some(Piece::pack(Side::White, Rank::Rook)));
        assert_eq!(*board.piece_at(coord!("b1")), Some(Piece::pack(Side::White, Rank::Knight)));
        assert_eq!(*board.piece_at(coord!("c1")), Some(Piece::pack(Side::White, Rank::Bishop)));
        assert_eq!(*board.piece_at(coord!("d1")), Some(Piece::pack(Side::White, Rank::Queen)));
        assert_eq!(*board.piece_at(coord!("e1")), Some(Piece::pack(Side::White, Rank::King)));
        assert_eq!(*board.piece_at(coord!("f1")), Some(Piece::pack(Side::White, Rank::Bishop)));
        assert_eq!(*board.piece_at(coord!("g1")), Some(Piece::pack(Side::White, Rank::Knight)));
        assert_eq!(*board.piece_at(coord!("h1")), Some(Piece::pack(Side::White, Rank::Rook)));

        assert_eq!(*board.piece_at(coord!("a2")), Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("b2")), Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("c2")), Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("d2")), Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("e2")), Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("f2")), Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("g2")), Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("h2")), Some(Piece::pack(Side::White, Rank::Pawn)));

        assert_eq!(*board.piece_at(coord!("a8")), Some(Piece::pack(Side::Black, Rank::Rook)));
        assert_eq!(*board.piece_at(coord!("b8")), Some(Piece::pack(Side::Black, Rank::Knight)));
        assert_eq!(*board.piece_at(coord!("c8")), Some(Piece::pack(Side::Black, Rank::Bishop)));
        assert_eq!(*board.piece_at(coord!("d8")), Some(Piece::pack(Side::Black, Rank::Queen)));
        assert_eq!(*board.piece_at(coord!("e8")), Some(Piece::pack(Side::Black, Rank::King)));
        assert_eq!(*board.piece_at(coord!("f8")), Some(Piece::pack(Side::Black, Rank::Bishop)));
        assert_eq!(*board.piece_at(coord!("g8")), Some(Piece::pack(Side::Black, Rank::Knight)));
        assert_eq!(*board.piece_at(coord!("h8")), Some(Piece::pack(Side::Black, Rank::Rook)));

        assert_eq!(*board.piece_at(coord!("a7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("b7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("c7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("d7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("e7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("f7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("g7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
        assert_eq!(*board.piece_at(coord!("h7")), Some(Piece::pack(Side::Black, Rank::Pawn)));
    }

    #[test]
    fn finds_coordinates_of_pieces_on_the_board() {
        let board = Board::default();

        assert_eq!(
            board.find_pieces(Piece::pack(Side::Black, Rank::King)),
            vec![coord!("e8")]
        );

        assert_eq!(
            board.find_pieces(Piece::pack(Side::Black, Rank::Knight)),
            vec![
                coord!("b8"),
                coord!("g8")
            ]
        );
    }
}
