use std::fmt;

use crate::piece::Piece;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BoardX {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl BoardX {
    fn of_idx(num: u8) -> Option<BoardX> {
        match num {
            0 => Some(BoardX::A),
            1 => Some(BoardX::B),
            2 => Some(BoardX::C),
            3 => Some(BoardX::D),
            4 => Some(BoardX::E),
            5 => Some(BoardX::F),
            6 => Some(BoardX::G),
            7 => Some(BoardX::H),
            _ => None,
        }
    }

    pub fn to_idx(&self) -> u8 {
        match self {
            BoardX::A => 0,
            BoardX::B => 1,
            BoardX::C => 2,
            BoardX::D => 3,
            BoardX::E => 4,
            BoardX::F => 5,
            BoardX::G => 6,
            BoardX::H => 7,
        }
    }

    fn of_str(s: &str) -> Option<BoardX> {
        match s {
            "A" => Some(BoardX::A),
            "B" => Some(BoardX::B),
            "C" => Some(BoardX::C),
            "D" => Some(BoardX::D),
            "E" => Some(BoardX::E),
            "F" => Some(BoardX::F),
            "G" => Some(BoardX::G),
            "H" => Some(BoardX::H),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            BoardX::A => "A",
            BoardX::B => "B",
            BoardX::C => "C",
            BoardX::D => "D",
            BoardX::E => "E",
            BoardX::F => "F",
            BoardX::G => "G",
            BoardX::H => "H",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BoardY {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl BoardY {
    fn of_idx(i: u8) -> Option<BoardY> {
        match i {
            0 => Some(BoardY::_8),
            1 => Some(BoardY::_7),
            2 => Some(BoardY::_6),
            3 => Some(BoardY::_5),
            4 => Some(BoardY::_4),
            5 => Some(BoardY::_3),
            6 => Some(BoardY::_2),
            7 => Some(BoardY::_1),
            _ => None,
        }
    }

    pub fn to_idx(&self) -> u8 {
        match self {
            BoardY::_1 => 7,
            BoardY::_2 => 6,
            BoardY::_3 => 5,
            BoardY::_4 => 4,
            BoardY::_5 => 3,
            BoardY::_6 => 2,
            BoardY::_7 => 1,
            BoardY::_8 => 0,
        }
    }

    fn of_str(s: &str) -> Option<BoardY> {
        match s {
            "1" => Some(BoardY::_1),
            "2" => Some(BoardY::_2),
            "3" => Some(BoardY::_3),
            "4" => Some(BoardY::_4),
            "5" => Some(BoardY::_5),
            "6" => Some(BoardY::_6),
            "7" => Some(BoardY::_7),
            "8" => Some(BoardY::_8),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            BoardY::_1 => "1",
            BoardY::_2 => "2",
            BoardY::_3 => "3",
            BoardY::_4 => "4",
            BoardY::_5 => "5",
            BoardY::_6 => "6",
            BoardY::_7 => "7",
            BoardY::_8 => "8",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BoardPos {
    pub x: BoardX,
    pub y: BoardY,
}

impl BoardPos {
    pub fn of_idx(x: u8, y: u8) -> Option<BoardPos> {
        let x = BoardX::of_idx(x)?;
        let y = BoardY::of_idx(y)?;
        Some(BoardPos { x, y })
    }

    pub fn of_str(s: &str) -> Option<BoardPos> {
        let mut chars = s.chars();
        let x = chars.next().and_then(|pos| BoardX::of_str(&pos.to_string()))?;
        let y = chars.next().and_then(|pos| BoardY::of_str(&pos.to_string()))?;
        Some(BoardPos { x, y })
    }

    fn to_string(&self) -> String {
        format!("{}{}", self.x.to_str(), self.y.to_str())
    }
}

pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, PartialEq)]
pub struct InvalidCharacterErr;

impl fmt::Display for InvalidCharacterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Only [0-9] characters and spaces are allowed!")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidLengthErr;

impl fmt::Display for InvalidLengthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Every line must be 8 characters long")
    }
}

#[derive(Debug, PartialEq)]
pub enum FromStringErr {
    InvalidCharacter(InvalidCharacterErr),
    InvalidLength(InvalidLengthErr),
}

pub fn of_str(rows: [&str; 8]) -> Result<Board, FromStringErr> {
    if !rows
        .join("")
        .replace("♖", "")
        .replace("♘", "")
        .replace("♗", "")
        .replace("♕", "")
        .replace("♔", "")
        .replace("♙", "")
        .replace("♜", "")
        .replace("♞", "")
        .replace("♝", "")
        .replace("♛", "")
        .replace("♚", "")
        .replace("♟", "")
        .replace(" ", "")
        .is_empty()
    {
        return Err(FromStringErr::InvalidCharacter(InvalidCharacterErr));
    }
    for line in rows {
        if line.chars().count() != 8 {
            return Err(FromStringErr::InvalidLength(InvalidLengthErr));
        }
    }
    let mut res: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    for row in 0..8 {
        for col in 0..8 {
            res[row as usize][col as usize] =
                Piece::of_str(&rows[row as usize].chars().nth(col).unwrap().to_string());
        }
    }
    Ok(res)
}

fn get_default_board() -> Board {
    of_str([
        "♜♞♝♛♚♝♞♜",
        "♟♟♟♟♟♟♟♟",
        "        ",
        "        ",
        "        ",
        "        ",
        "♙♙♙♙♙♙♙♙",
        "♖♘♗♕♔♗♘♖",
    ])
    .unwrap()
}

fn get_board_piece(board: &Board, board_pos: &BoardPos) -> Option<Piece> {
    board[board_pos.y.to_idx() as usize][board_pos.x.to_idx() as usize]
}

fn board_to_string(board: &Board) -> String {
    let mut res: String = String::from("");
    for row in board {
        for col in row {
            match col {
                Some(p) => res.push_str(Piece::to_str(&p)),
                None => res.push_str(" "),
            }
        }
        res.push_str("\n")
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_x_of_idx() {
        assert_eq!(BoardX::of_idx(0), Some(BoardX::A));
        assert_eq!(BoardX::of_idx(1), Some(BoardX::B));
        assert_eq!(BoardX::of_idx(2), Some(BoardX::C));
        assert_eq!(BoardX::of_idx(3), Some(BoardX::D));
        assert_eq!(BoardX::of_idx(4), Some(BoardX::E));
        assert_eq!(BoardX::of_idx(5), Some(BoardX::F));
        assert_eq!(BoardX::of_idx(6), Some(BoardX::G));
        assert_eq!(BoardX::of_idx(7), Some(BoardX::H));
        assert_eq!(BoardX::of_idx(8), None);
    }

    #[test]
    fn test_board_x_to_idx() {
        assert_eq!(BoardX::A.to_idx(), 0);
        assert_eq!(BoardX::B.to_idx(), 1);
        assert_eq!(BoardX::C.to_idx(), 2);
        assert_eq!(BoardX::D.to_idx(), 3);
        assert_eq!(BoardX::E.to_idx(), 4);
        assert_eq!(BoardX::F.to_idx(), 5);
        assert_eq!(BoardX::G.to_idx(), 6);
        assert_eq!(BoardX::H.to_idx(), 7);
    }

    #[test]
    fn test_board_x_of_str() {
        assert_eq!(BoardX::of_str("A"), Some(BoardX::A));
        assert_eq!(BoardX::of_str("B"), Some(BoardX::B));
        assert_eq!(BoardX::of_str("C"), Some(BoardX::C));
        assert_eq!(BoardX::of_str("D"), Some(BoardX::D));
        assert_eq!(BoardX::of_str("E"), Some(BoardX::E));
        assert_eq!(BoardX::of_str("F"), Some(BoardX::F));
        assert_eq!(BoardX::of_str("G"), Some(BoardX::G));
        assert_eq!(BoardX::of_str("H"), Some(BoardX::H));
        assert_eq!(BoardX::of_str("I"), None);
    }

    #[test]
    fn test_board_x_to_str() {
        assert_eq!(BoardX::A.to_str(), "A");
        assert_eq!(BoardX::B.to_str(), "B");
        assert_eq!(BoardX::C.to_str(), "C");
        assert_eq!(BoardX::D.to_str(), "D");
        assert_eq!(BoardX::E.to_str(), "E");
        assert_eq!(BoardX::F.to_str(), "F");
        assert_eq!(BoardX::G.to_str(), "G");
        assert_eq!(BoardX::H.to_str(), "H");
    }

    #[test]
    fn test_board_y_of_idx() {
        assert_eq!(BoardY::of_idx(0), Some(BoardY::_8));
        assert_eq!(BoardY::of_idx(1), Some(BoardY::_7));
        assert_eq!(BoardY::of_idx(2), Some(BoardY::_6));
        assert_eq!(BoardY::of_idx(3), Some(BoardY::_5));
        assert_eq!(BoardY::of_idx(4), Some(BoardY::_4));
        assert_eq!(BoardY::of_idx(5), Some(BoardY::_3));
        assert_eq!(BoardY::of_idx(6), Some(BoardY::_2));
        assert_eq!(BoardY::of_idx(7), Some(BoardY::_1));
        assert_eq!(BoardY::of_idx(8), None);
    }

    #[test]
    fn test_board_y_to_idx() {
        assert_eq!(BoardY::_1.to_idx(), 7);
        assert_eq!(BoardY::_2.to_idx(), 6);
        assert_eq!(BoardY::_3.to_idx(), 5);
        assert_eq!(BoardY::_4.to_idx(), 4);
        assert_eq!(BoardY::_5.to_idx(), 3);
        assert_eq!(BoardY::_6.to_idx(), 2);
        assert_eq!(BoardY::_7.to_idx(), 1);
        assert_eq!(BoardY::_8.to_idx(), 0);
    }

    #[test]
    fn test_board_y_of_str() {
        assert_eq!(BoardY::of_str("0"), None);
        assert_eq!(BoardY::of_str("1"), Some(BoardY::_1));
        assert_eq!(BoardY::of_str("2"), Some(BoardY::_2));
        assert_eq!(BoardY::of_str("3"), Some(BoardY::_3));
        assert_eq!(BoardY::of_str("4"), Some(BoardY::_4));
        assert_eq!(BoardY::of_str("5"), Some(BoardY::_5));
        assert_eq!(BoardY::of_str("6"), Some(BoardY::_6));
        assert_eq!(BoardY::of_str("7"), Some(BoardY::_7));
        assert_eq!(BoardY::of_str("8"), Some(BoardY::_8));
    }

    #[test]
    fn test_board_y_to_str() {
        assert_eq!(BoardY::_1.to_str(), "1");
        assert_eq!(BoardY::_2.to_str(), "2");
        assert_eq!(BoardY::_3.to_str(), "3");
        assert_eq!(BoardY::_4.to_str(), "4");
        assert_eq!(BoardY::_5.to_str(), "5");
        assert_eq!(BoardY::_6.to_str(), "6");
        assert_eq!(BoardY::_7.to_str(), "7");
        assert_eq!(BoardY::_8.to_str(), "8");
    }

    #[test]
    fn test_board_pos_of_idx_some() {
        assert_eq!(BoardPos::of_idx(0, 7), Some(BoardPos { x: BoardX::A, y: BoardY::_1 }));
        assert_eq!(BoardPos::of_idx(0, 6), Some(BoardPos { x: BoardX::A, y: BoardY::_2 }));
        assert_eq!(BoardPos::of_idx(0, 5), Some(BoardPos { x: BoardX::A, y: BoardY::_3 }));
        assert_eq!(BoardPos::of_idx(0, 4), Some(BoardPos { x: BoardX::A, y: BoardY::_4 }));
        assert_eq!(BoardPos::of_idx(0, 3), Some(BoardPos { x: BoardX::A, y: BoardY::_5 }));
        assert_eq!(BoardPos::of_idx(0, 2), Some(BoardPos { x: BoardX::A, y: BoardY::_6 }));
        assert_eq!(BoardPos::of_idx(0, 1), Some(BoardPos { x: BoardX::A, y: BoardY::_7 }));
        assert_eq!(BoardPos::of_idx(0, 0), Some(BoardPos { x: BoardX::A, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_idx(1, 0), Some(BoardPos { x: BoardX::B, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_idx(2, 0), Some(BoardPos { x: BoardX::C, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_idx(3, 0), Some(BoardPos { x: BoardX::D, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_idx(4, 0), Some(BoardPos { x: BoardX::E, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_idx(5, 0), Some(BoardPos { x: BoardX::F, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_idx(6, 0), Some(BoardPos { x: BoardX::G, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_idx(7, 0), Some(BoardPos { x: BoardX::H, y: BoardY::_8 }));
    }

    #[test]
    fn test_board_pos_of_idx_none() {
        assert_eq!(BoardPos::of_idx(8, 8), None);
        assert_eq!(BoardPos::of_idx(9, 9), None);
    }

    #[test]
    fn test_board_pos_of_str_some() {
        assert_eq!(BoardPos::of_str("A1"), Some(BoardPos { x: BoardX::A, y: BoardY::_1 }));
        assert_eq!(BoardPos::of_str("A2"), Some(BoardPos { x: BoardX::A, y: BoardY::_2 }));
        assert_eq!(BoardPos::of_str("A3"), Some(BoardPos { x: BoardX::A, y: BoardY::_3 }));
        assert_eq!(BoardPos::of_str("A4"), Some(BoardPos { x: BoardX::A, y: BoardY::_4 }));
        assert_eq!(BoardPos::of_str("A5"), Some(BoardPos { x: BoardX::A, y: BoardY::_5 }));
        assert_eq!(BoardPos::of_str("A6"), Some(BoardPos { x: BoardX::A, y: BoardY::_6 }));
        assert_eq!(BoardPos::of_str("A7"), Some(BoardPos { x: BoardX::A, y: BoardY::_7 }));
        assert_eq!(BoardPos::of_str("A8"), Some(BoardPos { x: BoardX::A, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_str("B8"), Some(BoardPos { x: BoardX::B, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_str("C8"), Some(BoardPos { x: BoardX::C, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_str("D8"), Some(BoardPos { x: BoardX::D, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_str("E8"), Some(BoardPos { x: BoardX::E, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_str("F8"), Some(BoardPos { x: BoardX::F, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_str("G8"), Some(BoardPos { x: BoardX::G, y: BoardY::_8 }));
        assert_eq!(BoardPos::of_str("H8"), Some(BoardPos { x: BoardX::H, y: BoardY::_8 }));
    }

    #[test]
    fn test_board_pos_of_str_none() {
        assert_eq!(BoardPos::of_str("A0"), None);
        assert_eq!(BoardPos::of_str("A9"), None);
        assert_eq!(BoardPos::of_str("I1"), None);
        assert_eq!(BoardPos::of_str("I2"), None);
    }

    #[test]
    fn test_board_pos_to_string() {
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_1 }.to_string(), String::from("A1"));
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_2 }.to_string(), String::from("A2"));
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_3 }.to_string(), String::from("A3"));
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_4 }.to_string(), String::from("A4"));
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_5 }.to_string(), String::from("A5"));
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_6 }.to_string(), String::from("A6"));
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_7 }.to_string(), String::from("A7"));
        assert_eq!(BoardPos { x: BoardX::A, y: BoardY::_8 }.to_string(), String::from("A8"));
        assert_eq!(BoardPos { x: BoardX::B, y: BoardY::_8 }.to_string(), String::from("B8"));
        assert_eq!(BoardPos { x: BoardX::C, y: BoardY::_8 }.to_string(), String::from("C8"));
        assert_eq!(BoardPos { x: BoardX::D, y: BoardY::_8 }.to_string(), String::from("D8"));
        assert_eq!(BoardPos { x: BoardX::E, y: BoardY::_8 }.to_string(), String::from("E8"));
        assert_eq!(BoardPos { x: BoardX::F, y: BoardY::_8 }.to_string(), String::from("F8"));
        assert_eq!(BoardPos { x: BoardX::G, y: BoardY::_8 }.to_string(), String::from("G8"));
        assert_eq!(BoardPos { x: BoardX::H, y: BoardY::_8 }.to_string(), String::from("H8"));
    }

    #[test]
    fn test_of_str() {
        assert_eq!(
            of_str([
                "♜♞♝♛♚♝♞♜",
                "♟♟♟♟♟♟♟♟",
                "        ",
                "        ",
                "        ",
                "        ",
                "♙♙♙♙♙♙♙♙",
                "♖♘♗♕♔♗♘♖",
            ]),
            Ok([
                [
                    Piece::of_str("♜"),
                    Piece::of_str("♞"),
                    Piece::of_str("♝"),
                    Piece::of_str("♛"),
                    Piece::of_str("♚"),
                    Piece::of_str("♝"),
                    Piece::of_str("♞"),
                    Piece::of_str("♜"),
                ],
                [
                    Piece::of_str("♟"),
                    Piece::of_str("♟"),
                    Piece::of_str("♟"),
                    Piece::of_str("♟"),
                    Piece::of_str("♟"),
                    Piece::of_str("♟"),
                    Piece::of_str("♟"),
                    Piece::of_str("♟"),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Piece::of_str("♙"),
                    Piece::of_str("♙"),
                    Piece::of_str("♙"),
                    Piece::of_str("♙"),
                    Piece::of_str("♙"),
                    Piece::of_str("♙"),
                    Piece::of_str("♙"),
                    Piece::of_str("♙"),
                ],
                [
                    Piece::of_str("♖"),
                    Piece::of_str("♘"),
                    Piece::of_str("♗"),
                    Piece::of_str("♕"),
                    Piece::of_str("♔"),
                    Piece::of_str("♗"),
                    Piece::of_str("♘"),
                    Piece::of_str("♖"),
                ],
            ])
        );
    }

    #[test]
    fn test_get_board_piece() {
        let brd = get_default_board();
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("A8").unwrap()), Piece::of_str("♜"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("B8").unwrap()), Piece::of_str("♞"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("C8").unwrap()), Piece::of_str("♝"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("D8").unwrap()), Piece::of_str("♛"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("E8").unwrap()), Piece::of_str("♚"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("F8").unwrap()), Piece::of_str("♝"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("G8").unwrap()), Piece::of_str("♞"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H8").unwrap()), Piece::of_str("♜"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H8").unwrap()), Piece::of_str("♜"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H7").unwrap()), Piece::of_str("♟"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H6").unwrap()), Piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H5").unwrap()), Piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H4").unwrap()), Piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H3").unwrap()), Piece::of_str(" "));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H2").unwrap()), Piece::of_str("♙"));
        assert_eq!(get_board_piece(&brd, &BoardPos::of_str("H1").unwrap()), Piece::of_str("♖"));
    }

    #[test]
    fn test_board_to_string() {
        let brd = get_default_board();
        let res = board_to_string(&brd);
        assert_eq!(
            res.to_owned(),
            "".to_owned()
                + "♜♞♝♛♚♝♞♜\n"
                + "♟♟♟♟♟♟♟♟\n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "♙♙♙♙♙♙♙♙\n"
                + "♖♘♗♕♔♗♘♖\n"
        );
    }
}
