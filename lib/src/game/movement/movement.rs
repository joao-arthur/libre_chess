use crate::{movement::Movement, piece::Piece, pos::Pos};

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultMovement {
    pub movement: Movement,
}

impl From<Movement> for DefaultMovement {
    fn from(movement: Movement) -> Self {
        DefaultMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaptureMovement {
    pub movement: Movement,
}

impl From<Movement> for CaptureMovement {
    fn from(movement: Movement) -> Self {
        CaptureMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MenaceMovement {
    pub movement: Movement,
}

impl From<Movement> for MenaceMovement {
    fn from(movement: Movement) -> Self {
        MenaceMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnPassantMovement {
    pub movement: Movement,
}

impl From<Movement> for EnPassantMovement {
    fn from(movement: Movement) -> Self {
        EnPassantMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastlingMovement {
    pub movement: Movement,
}

impl From<Movement> for CastlingMovement {
    fn from(movement: Movement) -> Self {
        CastlingMovement { movement }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionMovement {
    pub pos: Pos,
    pub piece: Piece,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMovement {
    Default(DefaultMovement),
    Capture(CaptureMovement),
    Menace(MenaceMovement),
    EnPassant(EnPassantMovement),
    Castling(CastlingMovement),
    Promotion(PromotionMovement),
}

impl From<DefaultMovement> for GameMovement {
    fn from(movement: DefaultMovement) -> Self {
        GameMovement::Default(movement)
    }
}

impl From<CaptureMovement> for GameMovement {
    fn from(movement: CaptureMovement) -> Self {
        GameMovement::Capture(movement)
    }
}

impl From<MenaceMovement> for GameMovement {
    fn from(movement: MenaceMovement) -> Self {
        GameMovement::Menace(movement)
    }
}

impl From<EnPassantMovement> for GameMovement {
    fn from(movement: EnPassantMovement) -> Self {
        GameMovement::EnPassant(movement)
    }
}

impl From<CastlingMovement> for GameMovement {
    fn from(movement: CastlingMovement) -> Self {
        GameMovement::Castling(movement)
    }
}

impl From<PromotionMovement> for GameMovement {
    fn from(movement: PromotionMovement) -> Self {
        GameMovement::Promotion(movement)
    }
}

fn game_movements_to_string(moves: Vec<GameMovement>) -> String {
    String::new()
}

fn game_movements_from_str(value: Vec<&str>) -> Result<Vec<GameMovement>, ()> {
    Err(())
}

// impl GameMovement {
//     fn try_of<const N: usize>(
//         bounds: &GameBounds,
//         rows: [&str; N],
//     ) -> Result<Vec<GameMovement>> {
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