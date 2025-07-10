use crate::{
    game::{
        board::{GameBoardErr, InvalidCharacterErr, InvalidLengthErr},
        game::GameBounds,
    },
    mov::Mov,
    piece::Piece,
    pos::Pos,
};

pub mod default;
pub mod special;

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultMovOld {
    pub mov: Mov,
}

impl From<Mov> for DefaultMovOld {
    fn from(mov: Mov) -> Self {
        DefaultMovOld { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaptureMovOld {
    pub mov: Mov,
}

impl From<Mov> for CaptureMovOld {
    fn from(mov: Mov) -> Self {
        CaptureMovOld { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MenaceMovOld {
    pub mov: Mov,
}

impl From<Mov> for MenaceMovOld {
    fn from(mov: Mov) -> Self {
        MenaceMovOld { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnPassantMovOld {
    pub mov: Mov,
}

impl From<Mov> for EnPassantMovOld {
    fn from(mov: Mov) -> Self {
        EnPassantMovOld { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastlingMovOld {
    pub mov: Mov,
}

impl From<Mov> for CastlingMovOld {
    fn from(mov: Mov) -> Self {
        CastlingMovOld { mov }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionMovOld {
    pub pos: Pos,
    pub piece: Piece,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMovOld {
    Default(DefaultMovOld),
    Capture(CaptureMovOld),
    Menace(MenaceMovOld),
    EnPassant(EnPassantMovOld),
    Castling(CastlingMovOld),
    //ShortCastling(CastlingMovOld),
    //LongCastling(CastlingMovOld),
    Promotion(PromotionMovOld),
}

impl From<DefaultMovOld> for GameMovOld {
    fn from(mov: DefaultMovOld) -> Self {
        GameMovOld::Default(mov)
    }
}

impl From<CaptureMovOld> for GameMovOld {
    fn from(mov: CaptureMovOld) -> Self {
        GameMovOld::Capture(mov)
    }
}

impl From<MenaceMovOld> for GameMovOld {
    fn from(mov: MenaceMovOld) -> Self {
        GameMovOld::Menace(mov)
    }
}

impl From<EnPassantMovOld> for GameMovOld {
    fn from(mov: EnPassantMovOld) -> Self {
        GameMovOld::EnPassant(mov)
    }
}

impl From<CastlingMovOld> for GameMovOld {
    fn from(mov: CastlingMovOld) -> Self {
        GameMovOld::Castling(mov)
    }
}

impl From<PromotionMovOld> for GameMovOld {
    fn from(mov: PromotionMovOld) -> Self {
        GameMovOld::Promotion(mov)
    }
}

pub fn try_game_move_vec_from_str<const N: usize>(
    bounds: &GameBounds,
    rows: [&str; N],
) -> Result<Vec<GameMovOld>, GameBoardErr> {
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
    let piece = piece.ok_or(GameBoardErr::InvalidCharacter(InvalidCharacterErr))?;
    let from = from.ok_or(GameBoardErr::InvalidCharacter(InvalidCharacterErr))?;
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
                '●' => res.push(GameMovOld::from(DefaultMovOld::from(mov))),
                '◎' => res.push(GameMovOld::from(CaptureMovOld::from(mov))),
                '○' => res.push(GameMovOld::from(MenaceMovOld::from(mov))),
                _ => {}
            }
        }
    }
    Ok(res)
}

pub fn game_move_vec_to_string(bounds: &GameBounds, moves: &Vec<GameMovOld>) -> String {
    let mut res = "".to_string();

    let mut row = bounds.y2 + 1;
    while row > bounds.y1 {
        row -= 1;
        for col in bounds.x1..=bounds.x2 {
            let pos = Pos { row, col };
            let maybe_mov = moves.iter().find(|mov| match mov {
                GameMovOld::Default(default_move) => default_move.mov.to == pos,
                GameMovOld::Capture(capture_move) => capture_move.mov.to == pos,
                GameMovOld::Menace(menace_move) => menace_move.mov.to == pos,
                _ => false,
            });
            if let Some(mov) = maybe_mov {
                match mov {
                    GameMovOld::Default(_) => res.push('●'),
                    GameMovOld::Capture(_) => res.push('◎'),
                    GameMovOld::Menace(_) => res.push('○'),
                    // Default => '○'
                    // Capture => '◎'
                    // Menace => '◌'
                    // En Passant => '●'
                    // Castling => '✚'
                    _ => res.push(' '),
                };
                continue;
            }
            let maybe_piece_mov = moves.iter().find(|mov| match mov {
                GameMovOld::Default(default_move) => default_move.mov.from == pos,
                GameMovOld::Capture(capture_move) => capture_move.mov.from == pos,
                GameMovOld::Menace(menace_move) => menace_move.mov.from == pos,
                _ => false,
            });
            if let Some(piece_mov) = maybe_piece_mov {
                match piece_mov {
                    GameMovOld::Default(mov) => res.push_str(&mov.mov.piece.to_string()),
                    GameMovOld::Capture(mov) => res.push_str(&mov.mov.piece.to_string()),
                    GameMovOld::Menace(mov) => res.push_str(&mov.mov.piece.to_string()),
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



/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////
/////////////

#[derive(Debug, PartialEq, Clone)]
pub struct Default;

#[derive(Debug, PartialEq, Clone)]
pub struct Capture;

#[derive(Debug, PartialEq, Clone)]
pub struct Menace;

#[derive(Debug, PartialEq, Clone)]
pub struct EnPassant;

#[derive(Debug, PartialEq, Clone)]
pub struct LongCastling;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToQueen;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToRook;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToBishop;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToKnight;

#[derive(Debug, PartialEq, Clone)]
pub enum GameMovV2 {

}






#[cfg(test)]
mod tests {
    use crate::{game::mode::standard_chess, mov::Mov};

    use super::{
        CaptureMovOld, DefaultMovOld, GameMovOld, MenaceMovOld, game_move_vec_to_string,
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
                GameMovOld::from(MenaceMovOld::from(Mov::of('♘', "D4", "C2"))),
                GameMovOld::from(MenaceMovOld::from(Mov::of('♘', "D4", "E2"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "B3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "F3"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "B5"))),
                GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "F5"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♘', "D4", "C6"))),
                GameMovOld::from(CaptureMovOld::from(Mov::of('♘', "D4", "E6"))),
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
                    GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "B3"))),
                    GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "B5"))),
                    GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "F3"))),
                    GameMovOld::from(DefaultMovOld::from(Mov::of('♘', "D4", "F5"))),
                    GameMovOld::from(MenaceMovOld::from(Mov::of('♘', "D4", "C2"))),
                    GameMovOld::from(MenaceMovOld::from(Mov::of('♘', "D4", "E2"))),
                    GameMovOld::from(CaptureMovOld::from(Mov::of('♘', "D4", "C6"))),
                    GameMovOld::from(CaptureMovOld::from(Mov::of('♘', "D4", "E6"))),
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
