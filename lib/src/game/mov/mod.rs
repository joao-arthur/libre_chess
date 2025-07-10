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
pub struct DefaultMove;

#[derive(Debug, PartialEq, Clone)]
pub struct CaptureMove;

#[derive(Debug, PartialEq, Clone)]
pub struct MenaceMove;

#[derive(Debug, PartialEq, Clone)]
pub struct EnPassantMove;

#[derive(Debug, PartialEq, Clone)]
pub struct LongCastlingMove;

#[derive(Debug, PartialEq, Clone)]
pub struct ShortCastlingMove;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToQueenMove;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToRookMove;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToBishopMove;

#[derive(Debug, PartialEq, Clone)]
pub struct PromotionToKnightMove;

#[derive(Debug, PartialEq, Clone)]
pub enum GameMoveType {
    Default(DefaultMove),
    Capture(CaptureMove),
    Menace(MenaceMove),
    EnPassant(EnPassantMove),
    LongCastling(LongCastlingMove),
    ShortCastling(ShortCastlingMove),
    PromotionToQueen(PromotionToQueenMove),
    PromotionToRook(PromotionToRookMove),
    PromotionToBishop(PromotionToBishopMove),
    PromotionToKnight(PromotionToKnightMove),
}

impl From<DefaultMove> for GameMoveType {
    fn from(mov: DefaultMove) -> Self {
        GameMoveType::Default(mov)
    }
}

impl From<CaptureMove> for GameMoveType {
    fn from(mov: CaptureMove) -> Self {
        GameMoveType::Capture(mov)
    }
}

impl From<MenaceMove> for GameMoveType {
    fn from(mov: MenaceMove) -> Self {
        GameMoveType::Menace(mov)
    }
}

impl From<EnPassantMove> for GameMoveType {
    fn from(mov: EnPassantMove) -> Self {
        GameMoveType::EnPassant(mov)
    }
}

impl From<LongCastlingMove> for GameMoveType {
    fn from(mov: LongCastlingMove) -> Self {
        GameMoveType::LongCastling(mov)
    }
}

impl From<ShortCastlingMove> for GameMoveType {
    fn from(mov: ShortCastlingMove) -> Self {
        GameMoveType::ShortCastling(mov)
    }
}

impl From<PromotionToQueenMove> for GameMoveType {
    fn from(mov: PromotionToQueenMove) -> Self {
        GameMoveType::PromotionToQueen(mov)
    }
}

impl From<PromotionToRookMove> for GameMoveType {
    fn from(mov: PromotionToRookMove) -> Self {
        GameMoveType::PromotionToRook(mov)
    }
}

impl From<PromotionToBishopMove> for GameMoveType {
    fn from(mov: PromotionToBishopMove) -> Self {
        GameMoveType::PromotionToBishop(mov)
    }
}

impl From<PromotionToKnightMove> for GameMoveType {
    fn from(mov: PromotionToKnightMove) -> Self {
        GameMoveType::PromotionToKnight(mov)
    }
}

impl GameMoveType {
    fn default() -> Self {
        GameMoveType::from(DefaultMove)
    }

    fn capture() -> Self {
        GameMoveType::from(CaptureMove)
    }

    fn menace() -> Self {
        GameMoveType::from(MenaceMove)
    }

    fn en_passant() -> Self {
        GameMoveType::from(EnPassantMove)
    }

    fn long_castling() -> Self {
        GameMoveType::from(LongCastlingMove)
    }

    fn short_castling() -> Self {
        GameMoveType::from(ShortCastlingMove)
    }

    fn promotion_to_queen() -> Self {
        GameMoveType::from(PromotionToQueenMove)
    }

    fn promotion_to_rook() -> Self {
        GameMoveType::from(PromotionToRookMove)
    }

    fn promotion_to_bishop() -> Self {
        GameMoveType::from(PromotionToBishopMove)
    }

    fn promotion_to_knight() -> Self {
        GameMoveType::from(PromotionToKnightMove)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameMove {
    from: Pos,
    to: Pos,
    t: GameMoveType,
}

impl GameMove {
    pub fn try_of(from: &str, to: &str, t: GameMoveType) -> Option<Self> {
        let from = Pos::try_of_str(from)?;
        let to = Pos::try_of_str(to)?;
        Some(GameMove { from, to, t })
    }

    pub fn of(from: &str, to: &str, t: GameMoveType) -> Self {
        Self::try_of(from, to, t).unwrap()
    }

    pub fn default_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::default())
    }

    pub fn default_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::default())
    }

    pub fn capture_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::capture())
    }

    pub fn capture_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::capture())
    }

    pub fn menace_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::menace())
    }

    pub fn menace_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::menace())
    }

    pub fn en_passant_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::en_passant())
    }

    pub fn en_passant_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::en_passant())
    }

    pub fn long_castling_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::long_castling())
    }

    pub fn long_castling_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::long_castling())
    }

    pub fn short_castling_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::short_castling())
    }

    pub fn short_castling_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::short_castling())
    }

    pub fn promotion_to_queen_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::promotion_to_queen())
    }

    pub fn promotion_to_queen_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::promotion_to_queen())
    }

    pub fn promotion_to_rook_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::promotion_to_rook())
    }

    pub fn promotion_to_rook_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::promotion_to_rook())
    }

    pub fn promotion_to_bishop_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::promotion_to_bishop())
    }

    pub fn promotion_to_bishop_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::promotion_to_bishop())
    }

    pub fn promotion_to_knight_try_of(from: &str, to: &str) -> Option<Self> {
        Self::try_of(from, to, GameMoveType::promotion_to_knight())
    }

    pub fn promotion_to_knight_of(from: &str, to: &str) -> Self {
        Self::of(from, to, GameMoveType::promotion_to_knight())
    }
}

#[cfg(test)]
mod tests {
    use crate::{game::mode::standard_chess, mov::Mov, pos::Pos};

    use super::{
        CaptureMovOld, CaptureMove, DefaultMovOld, DefaultMove, EnPassantMove, GameMovOld,
        GameMove, GameMoveType, LongCastlingMove, MenaceMovOld, MenaceMove, PromotionToBishopMove,
        PromotionToKnightMove, PromotionToQueenMove, PromotionToRookMove, ShortCastlingMove,
        game_move_vec_to_string, try_game_move_vec_from_str,
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

    fn game_move_from() {
        assert_eq!(GameMoveType::from(DefaultMove), GameMoveType::Default(DefaultMove));
        assert_eq!(GameMoveType::from(CaptureMove), GameMoveType::Capture(CaptureMove));
        assert_eq!(GameMoveType::from(MenaceMove), GameMoveType::Menace(MenaceMove));
        assert_eq!(GameMoveType::from(EnPassantMove), GameMoveType::EnPassant(EnPassantMove));
        assert_eq!(GameMoveType::from(LongCastlingMove), GameMoveType::LongCastling(LongCastlingMove));
        assert_eq!(GameMoveType::from(ShortCastlingMove), GameMoveType::ShortCastling(ShortCastlingMove));
        assert_eq!(GameMoveType::from(PromotionToQueenMove), GameMoveType::PromotionToQueen(PromotionToQueenMove));
        assert_eq!(GameMoveType::from(PromotionToRookMove), GameMoveType::PromotionToRook(PromotionToRookMove));
        assert_eq!(GameMoveType::from(PromotionToBishopMove), GameMoveType::PromotionToBishop(PromotionToBishopMove));
        assert_eq!(GameMoveType::from(PromotionToKnightMove), GameMoveType::PromotionToKnight(PromotionToKnightMove));
    }

    fn game_move() {
        assert_eq!(GameMoveType::default(), GameMoveType::Default(DefaultMove));
        assert_eq!(GameMoveType::capture(), GameMoveType::Capture(CaptureMove));
        assert_eq!(GameMoveType::menace(), GameMoveType::Menace(MenaceMove));
        assert_eq!(GameMoveType::en_passant(), GameMoveType::EnPassant(EnPassantMove));
        assert_eq!(GameMoveType::long_castling(), GameMoveType::LongCastling(LongCastlingMove));
        assert_eq!(GameMoveType::short_castling(), GameMoveType::ShortCastling(ShortCastlingMove));
        assert_eq!(GameMoveType::promotion_to_queen(), GameMoveType::PromotionToQueen(PromotionToQueenMove));
        assert_eq!(GameMoveType::promotion_to_rook(), GameMoveType::PromotionToRook(PromotionToRookMove));
        assert_eq!(GameMoveType::promotion_to_bishop(), GameMoveType::PromotionToBishop(PromotionToBishopMove));
        assert_eq!(GameMoveType::promotion_to_knight(), GameMoveType::PromotionToKnight(PromotionToKnightMove));
    }

    fn game_move_try_of() {
        assert_eq!(
            GameMove::try_of("A2", "A4", GameMoveType::default()),
            Some(GameMove {
                from: Pos { col: 0, row: 1 },
                to: Pos { col: 0, row: 3 },
                t: GameMoveType::default()
            })
        );
        assert_eq!(
            GameMove::try_of("D4", "E5", GameMoveType::capture()),
            Some(GameMove { from: Pos { col: 3, row: 3 }, to: Pos { col: 4, row: 4 }, t: GameMoveType::capture()
            })
        );
    }

    fn game_move_of() {
        assert_eq!(
            GameMove::of("A2", "A4", GameMoveType::default()),
            GameMove { from: Pos { col: 0, row: 1 }, to: Pos { col: 0, row: 3 }, t: GameMoveType::default()
            }
        );
        assert_eq!(
            GameMove::of("D4", "E5", GameMoveType::capture()),
            GameMove { from: Pos { col: 3, row: 3 }, to: Pos { col: 4, row: 4 }, t: GameMoveType::capture()
            }
        );
    }

    fn game_move_try_default_of() {
        assert_eq!(
            GameMove::default_try_of("A2", "A4"),
            Some(GameMove { from: Pos { col: 0, row: 1 }, to: Pos { col: 0, row: 3 }, t: GameMoveType::default()
            })
        );
        assert_eq!(
            GameMove::default_try_of("D4", "E5"),
            Some(GameMove { from: Pos { col: 3, row: 3 }, to: Pos { col: 4, row: 4 }, t: GameMoveType::capture()
            })
        );
    }

    fn game_move_default_of() {
        assert_eq!(
            GameMove::default_of("A2", "A4"),
            GameMove { from: Pos { col: 0, row: 1 }, to: Pos { col: 0, row: 3 }, t: GameMoveType::default()
            }
        );
        assert_eq!(
            GameMove::default_of("D4", "E5"),
            GameMove { from: Pos { col: 3, row: 3 }, to: Pos { col: 4, row: 4 }, t: GameMoveType::capture()
            }
        );
    }
}
