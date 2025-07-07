use crate::{movement::Movement, piece::Piece, pos::Pos};

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultMove {
    pub movement: Movement,
}

impl From<Movement> for DefaultMove {
    fn from(movement: Movement) -> Self {
        DefaultMove { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaptureMove {
    pub movement: Movement,
}

impl From<Movement> for CaptureMove {
    fn from(movement: Movement) -> Self {
        CaptureMove { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MenaceMove {
    pub movement: Movement,
}

impl From<Movement> for MenaceMove {
    fn from(movement: Movement) -> Self {
        MenaceMove { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnPassantMove {
    pub movement: Movement,
}

impl From<Movement> for EnPassantMove {
    fn from(movement: Movement) -> Self {
        EnPassantMove { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastlingMove {
    pub movement: Movement,
}

impl From<Movement> for CastlingMove {
    fn from(movement: Movement) -> Self {
        CastlingMove { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionMove {
    pub pos: Pos,
    pub piece: Piece,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMove {
    Default(DefaultMove),
    Capture(CaptureMove),
    Menace(MenaceMove),
    EnPassant(EnPassantMove),
    Castling(CastlingMove),
    Promotion(PromotionMove),
}

impl From<DefaultMove> for GameMove {
    fn from(movement: DefaultMove) -> Self {
        GameMove::Default(movement)
    }
}

impl From<CaptureMove> for GameMove {
    fn from(movement: CaptureMove) -> Self {
        GameMove::Capture(movement)
    }
}

impl From<MenaceMove> for GameMove {
    fn from(movement: MenaceMove) -> Self {
        GameMove::Menace(movement)
    }
}

impl From<EnPassantMove> for GameMove {
    fn from(movement: EnPassantMove) -> Self {
        GameMove::EnPassant(movement)
    }
}

impl From<CastlingMove> for GameMove {
    fn from(movement: CastlingMove) -> Self {
        GameMove::Castling(movement)
    }
}

impl From<PromotionMove> for GameMove {
    fn from(movement: PromotionMove) -> Self {
        GameMove::Promotion(movement)
    }
}

fn game_move_to_string(moves: Vec<GameMove>) -> String {
    String::new()
}

fn game_move_from_str(value: Vec<&str>) -> Result<Vec<GameMove>, ()> {
    Err(())
}

// impl GameMove {
//     fn try_of<const N: usize>(
//         bounds: &GameBounds,
//         rows: [&str; N],
//     ) -> Result<Vec<GameMove>> {
//         if rows.join("").find(|c| c != ' ' && Piece::try_of(c).is_none()).is_some() {
//             return Err(GameBoardErr::InvalidCharacter(InvalidCharacterErr));
//         }
//         let delta_x = usize::from(bounds.x2 - bounds.x1) + 1;
//         let delta_y: usize = usize::from(bounds.y2 - bounds.y1) + 1;
//         if rows.len() != delta_y {
//             return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
//         }
//         for line in rows {
//             if line.chars().count() != delta_x {
//                 return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
//             }
//         }
//         let mut board = HashMap::new();
//         for row in bounds.y1..=bounds.y2 {
//             for col in bounds.x1..=bounds.x2 {
//                 let row_index = bounds.y2 - row;
//                 let col_index = col - bounds.x1;
//                 let str_row = rows[row_index as usize];
//                 let str_col = str_row.chars().nth(col_index.into()).unwrap();
//                 if let Some(piece) = Piece::try_of(str_col) {
//                     board.insert(Pos { row, col }, piece);
//                 }
//             }
//         }
//         Ok(board)
//     }
// }