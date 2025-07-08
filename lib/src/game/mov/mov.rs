use crate::{
    game::{
        board::{GameBoardErr, InvalidCharacterErr, InvalidLengthErr},
        game::GameBounds,
    },
    mov::Mov,
    piece::Piece,
    pos::Pos,
};

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultMov {
    pub mov: Mov,
}

impl From<Mov> for DefaultMov {
    fn from(mov: Mov) -> Self {
        DefaultMov { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaptureMov {
    pub mov: Mov,
}

impl From<Mov> for CaptureMov {
    fn from(mov: Mov) -> Self {
        CaptureMov { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MenaceMov {
    pub mov: Mov,
}

impl From<Mov> for MenaceMov {
    fn from(mov: Mov) -> Self {
        MenaceMov { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnPassantMov {
    pub mov: Mov,
}

impl From<Mov> for EnPassantMov {
    fn from(mov: Mov) -> Self {
        EnPassantMov { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastlingMov {
    pub mov: Mov,
}

impl From<Mov> for CastlingMov {
    fn from(mov: Mov) -> Self {
        CastlingMov { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionMov {
    pub pos: Pos,
    pub piece: Piece,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMov {
    Default(DefaultMov),
    Capture(CaptureMov),
    Menace(MenaceMov),
    EnPassant(EnPassantMov),
    Castling(CastlingMov),
    Promotion(PromotionMov),
}

impl From<DefaultMov> for GameMov {
    fn from(mov: DefaultMov) -> Self {
        GameMov::Default(mov)
    }
}

impl From<CaptureMov> for GameMov {
    fn from(mov: CaptureMov) -> Self {
        GameMov::Capture(mov)
    }
}

impl From<MenaceMov> for GameMov {
    fn from(mov: MenaceMov) -> Self {
        GameMov::Menace(mov)
    }
}

impl From<EnPassantMov> for GameMov {
    fn from(mov: EnPassantMov) -> Self {
        GameMov::EnPassant(mov)
    }
}

impl From<CastlingMov> for GameMov {
    fn from(mov: CastlingMov) -> Self {
        GameMov::Castling(mov)
    }
}

impl From<PromotionMov> for GameMov {
    fn from(mov: PromotionMov) -> Self {
        GameMov::Promotion(mov)
    }
}

pub fn try_game_move_vec_from_str<const N: usize>(
    bounds: &GameBounds,
    rows: [&str; N],
) -> Result<Vec<GameMov>, GameBoardErr> {
    let joined = rows.join("");
    if joined
        .find(|c| c != ' ' && c != '●' && c != '◎' && c != '○' && Piece::try_of(c).is_none())
        .is_some()
    {
        return Err(GameBoardErr::InvalidCharacter(InvalidCharacterErr));
    }
    let delta_x = usize::from(bounds.x2 - bounds.x1) + 1;
    let delta_y: usize = usize::from(bounds.y2 - bounds.y1) + 1;
    if rows.len() != delta_y {
        return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
    }
    for row in rows {
        if row.chars().count() != delta_x {
            return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
        }
    }
    let mut piece: Option<Piece> = None;
    let mut from: Option<Pos> = None;
    for row in bounds.y1..=bounds.y2 {
        for col in bounds.x1..=bounds.x2 {
            let row_index = bounds.y2 - row;
            let col_index = col - bounds.x1;
            let str_row = rows[row_index as usize];
            let str_col = str_row.chars().nth(col_index.into()).unwrap();
            if let Some(p) = Piece::try_of(str_col) {
                piece = Some(p);
                from = Some(Pos { row, col });
                break;
            }
        }
    }
    let piece = piece.ok_or_else(|| GameBoardErr::InvalidCharacter(InvalidCharacterErr))?;
    let from = from.ok_or_else(|| GameBoardErr::InvalidCharacter(InvalidCharacterErr))?;
    let mut res = Vec::new();
    for row in bounds.y1..=bounds.y2 {
        for col in bounds.x1..=bounds.x2 {
            let row_index = bounds.y2 - row;
            let col_index = col - bounds.x1;
            let str_row = rows[row_index as usize];
            let str_col = str_row.chars().nth(col_index.into()).unwrap();
            let to = Pos { row, col };
            let mov = Mov { piece, from: from.clone(), to };
            match str_col {
                '●' => res.push(GameMov::from(DefaultMov::from(mov))),
                '◎' => res.push(GameMov::from(CaptureMov::from(mov))),
                '○' => res.push(GameMov::from(MenaceMov::from(mov))),
                _ => {}
            }
        }
    }
    return Ok(res);
}

pub fn game_move_vec_to_string(bounds: &GameBounds, moves: &Vec<GameMov>) -> String {
    let mut res = "".to_string();

    let mut row = bounds.y2 + 1;
    while row > bounds.y1 {
        row -= 1;
        for col in bounds.x1..=bounds.x2 {
            let pos = Pos { row, col };
            let maybe_mov = moves.iter().find(|mov| match mov {
                GameMov::Default(default_move) => default_move.mov.to == pos,
                GameMov::Capture(capture_move) => capture_move.mov.to == pos,
                GameMov::Menace(menace_move) => menace_move.mov.to == pos,
                _ => false,
            });
            if let Some(mov) = maybe_mov {
                match mov {
                    GameMov::Default(_) => res.push('●'),
                    GameMov::Capture(_) => res.push('◎'),
                    GameMov::Menace(_) => res.push('○'),
                    _ => res.push(' '),
                };
                continue;
            }
            let maybe_piece_mov = moves.iter().find(|mov| match mov {
                GameMov::Default(default_move) => default_move.mov.from == pos,
                GameMov::Capture(capture_move) => capture_move.mov.from == pos,
                GameMov::Menace(menace_move) => menace_move.mov.from == pos,
                _ => false,
            });
            if let Some(piece_mov) = maybe_piece_mov {
                match piece_mov {
                    GameMov::Default(mov) => res.push_str(&mov.mov.piece.to_string()),
                    GameMov::Capture(mov) => res.push_str(&mov.mov.piece.to_string()),
                    GameMov::Menace(mov) => res.push_str(&mov.mov.piece.to_string()),
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
        game::{game::GameBounds, mode::standard_chess},
        mov::Mov,
    };

    use super::{
        CaptureMov, DefaultMov, GameMov, MenaceMov, game_move_vec_to_string,
        try_game_move_vec_from_str,
    };

    #[test]
    fn test_try_game_move_vec_from_str() {
        let mode = standard_chess();
        assert_eq!(
            try_game_move_vec_from_str(
                &mode.bounds,
                [
                    "        ",
                    "        ",
                    "  ◎ ◎   ",
                    " ●   ●  ",
                    "   ♘    ",
                    " ●   ●  ",
                    "  ○ ○   ",
                    "        ",
                ]
            ),
            Ok(vec![
                GameMov::from(MenaceMov::from(Mov::of('♘', "D4", "C2"))),
                GameMov::from(MenaceMov::from(Mov::of('♘', "D4", "E2"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "B3"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "F3"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "B5"))),
                GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "F5"))),
                GameMov::from(CaptureMov::from(Mov::of('♘', "D4", "C6"))),
                GameMov::from(CaptureMov::from(Mov::of('♘', "D4", "E6"))),
            ])
        )
    }

    #[test]
    fn test_game_move_vec_to_string_standard_chess() {
        let mode = standard_chess();
        assert_eq!(
            game_move_vec_to_string(
                &mode.bounds,
                &vec![
                    GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "B3"))),
                    GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "B5"))),
                    GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "F3"))),
                    GameMov::from(DefaultMov::from(Mov::of('♘', "D4", "F5"))),
                    GameMov::from(MenaceMov::from(Mov::of('♘', "D4", "C2"))),
                    GameMov::from(MenaceMov::from(Mov::of('♘', "D4", "E2"))),
                    GameMov::from(CaptureMov::from(Mov::of('♘', "D4", "C6"))),
                    GameMov::from(CaptureMov::from(Mov::of('♘', "D4", "E6"))),
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
