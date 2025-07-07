use crate::{game::game::GameBounds, movement::Movement, piece::Piece, pos::Pos};

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

fn try_game_move_from_str<const N: usize>(
    bounds: &GameBounds,
    rows: [&str; N],
) -> Result<Vec<GameMove>, ()> {
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
    Err(())
}

fn game_move_to_string(bounds: &GameBounds, moves: &Vec<GameMove>) -> String {
    let mut res = "".to_string();

    let mut row = bounds.y2 + 1;
    while row > bounds.y1 {
        row -= 1;
        for col in bounds.x1..=bounds.x2 {
            let pos = Pos { row, col };
            let maybe_mov = moves.iter().find(|mov| match mov {
                GameMove::Default(default_move) => default_move.movement.to == pos,
                GameMove::Capture(capture_move) => capture_move.movement.to == pos,
                GameMove::Menace(menace_move) => menace_move.movement.to == pos,
                _ => false,
            });
            if let Some(mov) = maybe_mov {
                match mov {
                    GameMove::Default(_) => res.push('●'),
                    GameMove::Capture(_) => res.push('◎'),
                    GameMove::Menace(_) => res.push('○'),
                    _ => res.push(' '),
                };
                continue;
            }
            let maybe_piece_mov = moves.iter().find(|mov| match mov {
                GameMove::Default(default_move) => default_move.movement.from == pos,
                GameMove::Capture(capture_move) => capture_move.movement.from == pos,
                GameMove::Menace(menace_move) => menace_move.movement.from == pos,
                _ => false,
            });
            if let Some(piece_mov) = maybe_piece_mov {
                match piece_mov {
                    GameMove::Default(mov) => res.push_str(&mov.movement.piece.to_string()),
                    GameMove::Capture(mov) => res.push_str(&mov.movement.piece.to_string()),
                    GameMove::Menace(mov) => res.push_str(&mov.movement.piece.to_string()),
                    _ => res.push(' '),
                };
                continue;
            }
            res.push(' ');
        }
        res.push('\n')
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::{
        game::{
            game::GameBounds,
            mode::standard_chess,
            movement::movement::{CaptureMove, DefaultMove, GameMove, MenaceMove},
        },
        movement::Movement,
    };

    use super::{game_move_to_string, try_game_move_from_str};

    #[test]
    fn test_game_move_to_string_standard_chess() {
        let mode = standard_chess();
        assert_eq!(
            game_move_to_string(
                &mode.bounds,
                &vec![
                    GameMove::from(DefaultMove::from(Movement::of('♘', "D4", "B3"))),
                    GameMove::from(DefaultMove::from(Movement::of('♘', "D4", "B5"))),
                    GameMove::from(DefaultMove::from(Movement::of('♘', "D4", "F3"))),
                    GameMove::from(DefaultMove::from(Movement::of('♘', "D4", "F5"))),
                    GameMove::from(MenaceMove::from(Movement::of('♘', "D4", "C2"))),
                    GameMove::from(MenaceMove::from(Movement::of('♘', "D4", "E2"))),
                    GameMove::from(CaptureMove::from(Movement::of('♘', "D4", "C6"))),
                    GameMove::from(CaptureMove::from(Movement::of('♘', "D4", "E6"))),
                ]
            ),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "  ◎ ◎   \n"
                + " ●   ●  \n"
                + "   ♘    \n"
                + " ●   ●  \n"
                + "  ○ ○   \n"
                + "        \n"
        );
    }
}
