use std::fmt;

use crate::domain::piece::Piece;

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
    fn try_of_idx(i: u8) -> Option<Self> {
        match i {
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

    fn of_idx(i: u8) -> Self {
        Self::try_of_idx(i).unwrap()
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

    fn try_of_str(s: &str) -> Option<Self> {
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

    fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
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
    fn try_of_idx(i: u8) -> Option<Self> {
        match i {
            7 => Some(BoardY::_1),
            6 => Some(BoardY::_2),
            5 => Some(BoardY::_3),
            4 => Some(BoardY::_4),
            3 => Some(BoardY::_5),
            2 => Some(BoardY::_6),
            1 => Some(BoardY::_7),
            0 => Some(BoardY::_8),
            _ => None,
        }
    }

    fn of_idx(i: u8) -> Self {
        Self::try_of_idx(i).unwrap()
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

    fn try_of_str(s: &str) -> Option<BoardY> {
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

    fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
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

#[derive(Debug, PartialEq, Clone)]
pub struct BoardPos {
    pub x: BoardX,
    pub y: BoardY,
}

impl BoardPos {
    pub fn try_of_idx(x: u8, y: u8) -> Option<Self> {
        let x = BoardX::try_of_idx(x)?;
        let y = BoardY::try_of_idx(y)?;
        Some(BoardPos { x, y })
    }

    pub fn of_idx(x: u8, y: u8) -> Self {
        Self::try_of_idx(x, y).unwrap()
    }

    pub fn try_of_rel_idx(&self, x: i8, y: i8) -> Option<Self> {
        let self_x = self.x.to_idx() as i8;
        let self_y = self.y.to_idx() as i8;
        if x < 0 && x.abs() > self_x {
            return None;
        }
        if y < 0 && y.abs() > self_y {
            return None;
        }
        BoardPos::try_of_idx((self_x + x) as u8, (self_y + y) as u8)
    }

    pub fn of_rel_idx(&self, x: i8, y: i8) -> Self {
        self.try_of_rel_idx(x, y).unwrap()
    }

    fn try_of_str(s: &str) -> Option<Self> {
        let mut chars = s.chars();
        let x = chars.next().and_then(|pos| BoardX::try_of_str(&pos.to_string()))?;
        let y = chars.next().and_then(|pos| BoardY::try_of_str(&pos.to_string()))?;
        Some(BoardPos { x, y })
    }

    pub fn of_str(s: &str) -> Self {
        Self::try_of_str(s).unwrap()
    }

    pub fn to_string(&self) -> String {
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

fn try_of_str(rows: [&str; 8]) -> Result<Board, FromStringErr> {
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
    for row in 0..=7 {
        for col in 0..=7 {
            res[row as usize][col as usize] =
                Piece::try_of_str(&rows[row as usize].chars().nth(col).unwrap().to_string());
        }
    }
    Ok(res)
}

fn of_str(rows: [&str; 8]) -> Board {
    try_of_str(rows).unwrap()
}

pub fn get_initial_white_board() -> Board {
    of_str([
        "♖♘♗♕♔♗♘♖",
        "♙♙♙♙♙♙♙♙",
        "        ",
        "        ",
        "        ",
        "        ",
        "♟♟♟♟♟♟♟♟",
        "♜♞♝♛♚♝♞♜",
    ])
}

pub fn get_initial_black_board() -> Board {
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
}

fn get_board_piece(b: &Board, pos: &BoardPos) -> Option<Piece> {
    b[pos.y.to_idx() as usize][pos.x.to_idx() as usize]
}

fn board_to_string(b: &Board) -> String {
    let mut res: String = String::from("");
    for row in b {
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
    fn test_board_x_try_of_idx_some() {
        assert_eq!(BoardX::try_of_idx(0), Some(BoardX::A));
        assert_eq!(BoardX::try_of_idx(1), Some(BoardX::B));
        assert_eq!(BoardX::try_of_idx(2), Some(BoardX::C));
        assert_eq!(BoardX::try_of_idx(3), Some(BoardX::D));
        assert_eq!(BoardX::try_of_idx(4), Some(BoardX::E));
        assert_eq!(BoardX::try_of_idx(5), Some(BoardX::F));
        assert_eq!(BoardX::try_of_idx(6), Some(BoardX::G));
        assert_eq!(BoardX::try_of_idx(7), Some(BoardX::H));
    }

    #[test]
    fn test_board_x_try_of_idx_none() {
        assert_eq!(BoardX::try_of_idx(8), None);
        assert_eq!(BoardX::try_of_idx(9), None);
        assert_eq!(BoardX::try_of_idx(10), None);
    }

    #[test]
    fn test_board_x_from_idx() {
        assert_eq!(BoardX::of_idx(0), BoardX::A);
        assert_eq!(BoardX::of_idx(1), BoardX::B);
        assert_eq!(BoardX::of_idx(2), BoardX::C);
        assert_eq!(BoardX::of_idx(3), BoardX::D);
        assert_eq!(BoardX::of_idx(4), BoardX::E);
        assert_eq!(BoardX::of_idx(5), BoardX::F);
        assert_eq!(BoardX::of_idx(6), BoardX::G);
        assert_eq!(BoardX::of_idx(7), BoardX::H);
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
    fn test_board_x_try_of_str_some() {
        assert_eq!(BoardX::try_of_str("A"), Some(BoardX::A));
        assert_eq!(BoardX::try_of_str("B"), Some(BoardX::B));
        assert_eq!(BoardX::try_of_str("C"), Some(BoardX::C));
        assert_eq!(BoardX::try_of_str("D"), Some(BoardX::D));
        assert_eq!(BoardX::try_of_str("E"), Some(BoardX::E));
        assert_eq!(BoardX::try_of_str("F"), Some(BoardX::F));
        assert_eq!(BoardX::try_of_str("G"), Some(BoardX::G));
        assert_eq!(BoardX::try_of_str("H"), Some(BoardX::H));
    }

    #[test]
    fn test_board_x_try_of_str_none() {
        assert_eq!(BoardX::try_of_str("I"), None);
        assert_eq!(BoardX::try_of_str("J"), None);
        assert_eq!(BoardX::try_of_str("K"), None);
    }

    #[test]
    fn test_board_x_from_str() {
        assert_eq!(BoardX::of_str("A"), BoardX::A);
        assert_eq!(BoardX::of_str("B"), BoardX::B);
        assert_eq!(BoardX::of_str("C"), BoardX::C);
        assert_eq!(BoardX::of_str("D"), BoardX::D);
        assert_eq!(BoardX::of_str("E"), BoardX::E);
        assert_eq!(BoardX::of_str("F"), BoardX::F);
        assert_eq!(BoardX::of_str("G"), BoardX::G);
        assert_eq!(BoardX::of_str("H"), BoardX::H);
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
    fn test_board_y_try_of_idx_some() {
        assert_eq!(BoardY::try_of_idx(0), Some(BoardY::_8));
        assert_eq!(BoardY::try_of_idx(1), Some(BoardY::_7));
        assert_eq!(BoardY::try_of_idx(2), Some(BoardY::_6));
        assert_eq!(BoardY::try_of_idx(3), Some(BoardY::_5));
        assert_eq!(BoardY::try_of_idx(4), Some(BoardY::_4));
        assert_eq!(BoardY::try_of_idx(5), Some(BoardY::_3));
        assert_eq!(BoardY::try_of_idx(6), Some(BoardY::_2));
        assert_eq!(BoardY::try_of_idx(7), Some(BoardY::_1));
    }

    #[test]
    fn test_board_y_try_of_idx_none() {
        assert_eq!(BoardY::try_of_idx(8), None);
        assert_eq!(BoardY::try_of_idx(9), None);
        assert_eq!(BoardY::try_of_idx(10), None);
    }

    #[test]
    fn test_board_y_from_idx() {
        assert_eq!(BoardY::of_idx(0), BoardY::_8);
        assert_eq!(BoardY::of_idx(1), BoardY::_7);
        assert_eq!(BoardY::of_idx(2), BoardY::_6);
        assert_eq!(BoardY::of_idx(3), BoardY::_5);
        assert_eq!(BoardY::of_idx(4), BoardY::_4);
        assert_eq!(BoardY::of_idx(5), BoardY::_3);
        assert_eq!(BoardY::of_idx(6), BoardY::_2);
        assert_eq!(BoardY::of_idx(7), BoardY::_1);
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
    fn test_board_y_try_of_str_some() {
        assert_eq!(BoardY::try_of_str("1"), Some(BoardY::_1));
        assert_eq!(BoardY::try_of_str("2"), Some(BoardY::_2));
        assert_eq!(BoardY::try_of_str("3"), Some(BoardY::_3));
        assert_eq!(BoardY::try_of_str("4"), Some(BoardY::_4));
        assert_eq!(BoardY::try_of_str("5"), Some(BoardY::_5));
        assert_eq!(BoardY::try_of_str("6"), Some(BoardY::_6));
        assert_eq!(BoardY::try_of_str("7"), Some(BoardY::_7));
        assert_eq!(BoardY::try_of_str("8"), Some(BoardY::_8));
    }

    #[test]
    fn test_board_y_try_of_str_none() {
        assert_eq!(BoardY::try_of_str("0"), None);
        assert_eq!(BoardY::try_of_str("9"), None);
        assert_eq!(BoardY::try_of_str("10"), None);
    }

    #[test]
    fn test_board_y_from_str() {
        assert_eq!(BoardY::of_str("1"), BoardY::_1);
        assert_eq!(BoardY::of_str("2"), BoardY::_2);
        assert_eq!(BoardY::of_str("3"), BoardY::_3);
        assert_eq!(BoardY::of_str("4"), BoardY::_4);
        assert_eq!(BoardY::of_str("5"), BoardY::_5);
        assert_eq!(BoardY::of_str("6"), BoardY::_6);
        assert_eq!(BoardY::of_str("7"), BoardY::_7);
        assert_eq!(BoardY::of_str("8"), BoardY::_8);
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
    fn test_board_pos_try_of_idx_some() {
        assert_eq!(BoardPos::try_of_idx(0, 7), Some(BoardPos { x: BoardX::A, y: BoardY::_1 }));
        assert_eq!(BoardPos::try_of_idx(0, 6), Some(BoardPos { x: BoardX::A, y: BoardY::_2 }));
        assert_eq!(BoardPos::try_of_idx(0, 5), Some(BoardPos { x: BoardX::A, y: BoardY::_3 }));
        assert_eq!(BoardPos::try_of_idx(0, 4), Some(BoardPos { x: BoardX::A, y: BoardY::_4 }));
        assert_eq!(BoardPos::try_of_idx(0, 3), Some(BoardPos { x: BoardX::A, y: BoardY::_5 }));
        assert_eq!(BoardPos::try_of_idx(0, 2), Some(BoardPos { x: BoardX::A, y: BoardY::_6 }));
        assert_eq!(BoardPos::try_of_idx(0, 1), Some(BoardPos { x: BoardX::A, y: BoardY::_7 }));
        assert_eq!(BoardPos::try_of_idx(0, 0), Some(BoardPos { x: BoardX::A, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_idx(1, 0), Some(BoardPos { x: BoardX::B, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_idx(2, 0), Some(BoardPos { x: BoardX::C, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_idx(3, 0), Some(BoardPos { x: BoardX::D, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_idx(4, 0), Some(BoardPos { x: BoardX::E, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_idx(5, 0), Some(BoardPos { x: BoardX::F, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_idx(6, 0), Some(BoardPos { x: BoardX::G, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_idx(7, 0), Some(BoardPos { x: BoardX::H, y: BoardY::_8 }));
    }

    #[test]
    fn test_board_pos_try_of_idx_none() {
        assert_eq!(BoardPos::try_of_idx(8, 8), None);
        assert_eq!(BoardPos::try_of_idx(9, 9), None);
        assert_eq!(BoardPos::try_of_idx(10, 10), None);
    }

    #[test]
    fn test_board_pos_from_idx() {
        assert_eq!(BoardPos::of_idx(0, 7), BoardPos { x: BoardX::A, y: BoardY::_1 });
        assert_eq!(BoardPos::of_idx(0, 6), BoardPos { x: BoardX::A, y: BoardY::_2 });
        assert_eq!(BoardPos::of_idx(0, 5), BoardPos { x: BoardX::A, y: BoardY::_3 });
        assert_eq!(BoardPos::of_idx(0, 4), BoardPos { x: BoardX::A, y: BoardY::_4 });
        assert_eq!(BoardPos::of_idx(0, 3), BoardPos { x: BoardX::A, y: BoardY::_5 });
        assert_eq!(BoardPos::of_idx(0, 2), BoardPos { x: BoardX::A, y: BoardY::_6 });
        assert_eq!(BoardPos::of_idx(0, 1), BoardPos { x: BoardX::A, y: BoardY::_7 });
        assert_eq!(BoardPos::of_idx(0, 0), BoardPos { x: BoardX::A, y: BoardY::_8 });
        assert_eq!(BoardPos::of_idx(1, 0), BoardPos { x: BoardX::B, y: BoardY::_8 });
        assert_eq!(BoardPos::of_idx(2, 0), BoardPos { x: BoardX::C, y: BoardY::_8 });
        assert_eq!(BoardPos::of_idx(3, 0), BoardPos { x: BoardX::D, y: BoardY::_8 });
        assert_eq!(BoardPos::of_idx(4, 0), BoardPos { x: BoardX::E, y: BoardY::_8 });
        assert_eq!(BoardPos::of_idx(5, 0), BoardPos { x: BoardX::F, y: BoardY::_8 });
        assert_eq!(BoardPos::of_idx(6, 0), BoardPos { x: BoardX::G, y: BoardY::_8 });
        assert_eq!(BoardPos::of_idx(7, 0), BoardPos { x: BoardX::H, y: BoardY::_8 });
    }

    #[test]
    fn test_board_pos_try_of_rel_idx_some() {
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(0, 0), Some(BoardPos::of_idx(7, 7)));
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(-1, -1), Some(BoardPos::of_idx(6, 6)));
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(-2, -2), Some(BoardPos::of_idx(5, 5)));
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(-3, -3), Some(BoardPos::of_idx(4, 4)));
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(-4, -4), Some(BoardPos::of_idx(3, 3)));
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(-5, -5), Some(BoardPos::of_idx(2, 2)));
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(-6, -6), Some(BoardPos::of_idx(1, 1)));
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(-7, -7), Some(BoardPos::of_idx(0, 0)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(0, 0), Some(BoardPos::of_idx(0, 0)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(1, 1), Some(BoardPos::of_idx(1, 1)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(2, 2), Some(BoardPos::of_idx(2, 2)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(3, 3), Some(BoardPos::of_idx(3, 3)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(4, 4), Some(BoardPos::of_idx(4, 4)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(5, 5), Some(BoardPos::of_idx(5, 5)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(6, 6), Some(BoardPos::of_idx(6, 6)));
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(7, 7), Some(BoardPos::of_idx(7, 7)));
    }

    #[test]
    fn test_board_pos_try_of_rel_idx_none() {
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(-1, -1), None);
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(-2, -2), None);
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(-3, -3), None);
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(-4, -4), None);
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(-5, -5), None);
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(-6, -6), None);
        assert_eq!(BoardPos::of_idx(0, 0).try_of_rel_idx(-7, -7), None);
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(1, 1), None);
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(2, 2), None);
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(3, 3), None);
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(4, 4), None);
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(5, 5), None);
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(6, 6), None);
        assert_eq!(BoardPos::of_idx(7, 7).try_of_rel_idx(7, 7), None);
    }

    #[test]
    fn test_board_pos_of_rel_idx() {
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(0, 0), BoardPos::of_idx(7, 7));
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(-1, -1), BoardPos::of_idx(6, 6));
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(-2, -2), BoardPos::of_idx(5, 5));
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(-3, -3), BoardPos::of_idx(4, 4));
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(-4, -4), BoardPos::of_idx(3, 3));
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(-5, -5), BoardPos::of_idx(2, 2));
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(-6, -6), BoardPos::of_idx(1, 1));
        assert_eq!(BoardPos::of_idx(7, 7).of_rel_idx(-7, -7), BoardPos::of_idx(0, 0));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(0, 0), BoardPos::of_idx(0, 0));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(1, 1), BoardPos::of_idx(1, 1));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(2, 2), BoardPos::of_idx(2, 2));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(3, 3), BoardPos::of_idx(3, 3));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(4, 4), BoardPos::of_idx(4, 4));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(5, 5), BoardPos::of_idx(5, 5));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(6, 6), BoardPos::of_idx(6, 6));
        assert_eq!(BoardPos::of_idx(0, 0).of_rel_idx(7, 7), BoardPos::of_idx(7, 7));
    }

    #[test]
    fn test_board_pos_try_of_str_some() {
        assert_eq!(BoardPos::try_of_str("A1"), Some(BoardPos { x: BoardX::A, y: BoardY::_1 }));
        assert_eq!(BoardPos::try_of_str("A2"), Some(BoardPos { x: BoardX::A, y: BoardY::_2 }));
        assert_eq!(BoardPos::try_of_str("A3"), Some(BoardPos { x: BoardX::A, y: BoardY::_3 }));
        assert_eq!(BoardPos::try_of_str("A4"), Some(BoardPos { x: BoardX::A, y: BoardY::_4 }));
        assert_eq!(BoardPos::try_of_str("A5"), Some(BoardPos { x: BoardX::A, y: BoardY::_5 }));
        assert_eq!(BoardPos::try_of_str("A6"), Some(BoardPos { x: BoardX::A, y: BoardY::_6 }));
        assert_eq!(BoardPos::try_of_str("A7"), Some(BoardPos { x: BoardX::A, y: BoardY::_7 }));
        assert_eq!(BoardPos::try_of_str("A8"), Some(BoardPos { x: BoardX::A, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_str("B8"), Some(BoardPos { x: BoardX::B, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_str("C8"), Some(BoardPos { x: BoardX::C, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_str("D8"), Some(BoardPos { x: BoardX::D, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_str("E8"), Some(BoardPos { x: BoardX::E, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_str("F8"), Some(BoardPos { x: BoardX::F, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_str("G8"), Some(BoardPos { x: BoardX::G, y: BoardY::_8 }));
        assert_eq!(BoardPos::try_of_str("H8"), Some(BoardPos { x: BoardX::H, y: BoardY::_8 }));
    }

    #[test]
    fn test_board_pos_try_of_str_none() {
        assert_eq!(BoardPos::try_of_str("A0"), None);
        assert_eq!(BoardPos::try_of_str("A9"), None);
        assert_eq!(BoardPos::try_of_str("I1"), None);
        assert_eq!(BoardPos::try_of_str("J2"), None);
        assert_eq!(BoardPos::try_of_str("K3"), None);
    }

    #[test]
    fn test_board_pos_from_str() {
        assert_eq!(BoardPos::of_str("A1"), BoardPos { x: BoardX::A, y: BoardY::_1 });
        assert_eq!(BoardPos::of_str("A2"), BoardPos { x: BoardX::A, y: BoardY::_2 });
        assert_eq!(BoardPos::of_str("A3"), BoardPos { x: BoardX::A, y: BoardY::_3 });
        assert_eq!(BoardPos::of_str("A4"), BoardPos { x: BoardX::A, y: BoardY::_4 });
        assert_eq!(BoardPos::of_str("A5"), BoardPos { x: BoardX::A, y: BoardY::_5 });
        assert_eq!(BoardPos::of_str("A6"), BoardPos { x: BoardX::A, y: BoardY::_6 });
        assert_eq!(BoardPos::of_str("A7"), BoardPos { x: BoardX::A, y: BoardY::_7 });
        assert_eq!(BoardPos::of_str("A8"), BoardPos { x: BoardX::A, y: BoardY::_8 });
        assert_eq!(BoardPos::of_str("B8"), BoardPos { x: BoardX::B, y: BoardY::_8 });
        assert_eq!(BoardPos::of_str("C8"), BoardPos { x: BoardX::C, y: BoardY::_8 });
        assert_eq!(BoardPos::of_str("D8"), BoardPos { x: BoardX::D, y: BoardY::_8 });
        assert_eq!(BoardPos::of_str("E8"), BoardPos { x: BoardX::E, y: BoardY::_8 });
        assert_eq!(BoardPos::of_str("F8"), BoardPos { x: BoardX::F, y: BoardY::_8 });
        assert_eq!(BoardPos::of_str("G8"), BoardPos { x: BoardX::G, y: BoardY::_8 });
        assert_eq!(BoardPos::of_str("H8"), BoardPos { x: BoardX::H, y: BoardY::_8 });
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
    fn test_from_str() {
        assert_eq!(
            try_of_str([
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
                    Piece::try_of_str("♜"),
                    Piece::try_of_str("♞"),
                    Piece::try_of_str("♝"),
                    Piece::try_of_str("♛"),
                    Piece::try_of_str("♚"),
                    Piece::try_of_str("♝"),
                    Piece::try_of_str("♞"),
                    Piece::try_of_str("♜"),
                ],
                [
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                    Piece::try_of_str("♟"),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                    Piece::try_of_str("♙"),
                ],
                [
                    Piece::try_of_str("♖"),
                    Piece::try_of_str("♘"),
                    Piece::try_of_str("♗"),
                    Piece::try_of_str("♕"),
                    Piece::try_of_str("♔"),
                    Piece::try_of_str("♗"),
                    Piece::try_of_str("♘"),
                    Piece::try_of_str("♖"),
                ],
            ])
        );
    }

    #[test]
    fn test_get_board_piece() {
        let b = get_initial_black_board();
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("A8")), Piece::try_of_str("♜"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("B8")), Piece::try_of_str("♞"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("C8")), Piece::try_of_str("♝"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("D8")), Piece::try_of_str("♛"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("E8")), Piece::try_of_str("♚"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("F8")), Piece::try_of_str("♝"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("G8")), Piece::try_of_str("♞"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H8")), Piece::try_of_str("♜"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H8")), Piece::try_of_str("♜"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H7")), Piece::try_of_str("♟"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H6")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H5")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H4")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H3")), Piece::try_of_str(" "));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H2")), Piece::try_of_str("♙"));
        assert_eq!(get_board_piece(&b, &BoardPos::of_str("H1")), Piece::try_of_str("♖"));
    }

    #[test]
    fn test_board_to_string() {
        let brd = get_initial_black_board();
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
