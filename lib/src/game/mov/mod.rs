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
    mov: Mov,
    typ: GameMoveType,
}

impl GameMove {
    pub fn try_of(piece: char, from: &str, to: &str, typ: GameMoveType) -> Option<Self> {
        let mov = Mov::try_of(piece, from, to)?;
        Some(GameMove { mov, typ })
    }

    pub fn of(piece: char, from: &str, to: &str, typ: GameMoveType) -> Self {
        GameMove { mov: Mov::of(piece, from, to), typ }
    }

    pub fn default_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::default())
    }

    pub fn default_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::default())
    }

    pub fn capture_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::capture())
    }

    pub fn capture_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::capture())
    }

    pub fn menace_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::menace())
    }

    pub fn menace_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::menace())
    }

    pub fn en_passant_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::en_passant())
    }

    pub fn en_passant_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::en_passant())
    }

    pub fn long_castling_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::long_castling())
    }

    pub fn long_castling_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::long_castling())
    }

    pub fn short_castling_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::short_castling())
    }

    pub fn short_castling_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::short_castling())
    }

    pub fn promotion_to_queen_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::promotion_to_queen())
    }

    pub fn promotion_to_queen_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::promotion_to_queen())
    }

    pub fn promotion_to_rook_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::promotion_to_rook())
    }

    pub fn promotion_to_rook_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::promotion_to_rook())
    }

    pub fn promotion_to_bishop_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::promotion_to_bishop())
    }

    pub fn promotion_to_bishop_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::promotion_to_bishop())
    }

    pub fn promotion_to_knight_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::promotion_to_knight())
    }

    pub fn promotion_to_knight_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::promotion_to_knight())
    }
}

pub fn try_game_move_vec_from_str<const N: usize>(
    bounds: &GameBounds,
    rows: [&str; N],
) -> Result<Vec<GameMove>, GameBoardErr> {
    let joined = rows.join("");
    if joined
        .find(|c| {
            c != ' ' && c != '○' && c != '◎' && c != '◌' && c != '◍' && Piece::try_of(c).is_none()
        })
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
            match str_col {
                '○' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::Default(DefaultMove),
                }),
                '◎' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::Capture(CaptureMove),
                }),
                '◌' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::Menace(MenaceMove),
                }),
                '◍' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::EnPassant(EnPassantMove),
                }),
                _ => {}
            }
        }
    }
    Ok(res)
}

pub fn game_move_vec_to_string(bounds: &GameBounds, moves: &Vec<GameMove>) -> String {
    let mut res = "".to_string();
    let mut row = bounds.y2 + 1;
    while row > bounds.y1 {
        row -= 1;
        for col in bounds.x1..=bounds.x2 {
            let pos = Pos { row, col };
            let maybe_mov = moves.iter().find(|game_move| game_move.mov.to == pos);
            if let Some(mov) = maybe_mov {
                match mov.typ {
                    GameMoveType::Default(_) => res.push('○'),
                    GameMoveType::Capture(_) => res.push('◎'),
                    GameMoveType::Menace(_) => res.push('◌'),
                    GameMoveType::EnPassant(_) => res.push('◍'),
                    GameMoveType::LongCastling(_) | GameMoveType::ShortCastling(_) => res.push('✚'),
                    GameMoveType::PromotionToQueen(_)
                    | GameMoveType::PromotionToRook(_)
                    | GameMoveType::PromotionToBishop(_)
                    | GameMoveType::PromotionToKnight(_) => res.push('●'),
                };
                continue;
            }
            let maybe_piece_mov = moves.iter().find(|game_move| game_move.mov.from == pos);
            if let Some(piece_mov) = maybe_piece_mov {
                res.push_str(&piece_mov.mov.piece.to_string());
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
    use crate::{game::mode::standard_chess, mov::Mov};

    use super::{
        CaptureMove, DefaultMove, EnPassantMove, GameMove, GameMoveType, LongCastlingMove,
        MenaceMove, PromotionToBishopMove, PromotionToKnightMove, PromotionToQueenMove,
        PromotionToRookMove, ShortCastlingMove, game_move_vec_to_string,
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
                    " ○   ○  ",
                    "   ♘    ",
                    " ○   ○  ",
                    "  ◌ ◌   ",
                    "        ",
                ]
            ),
            Ok(vec![
                GameMove::menace_of('♘', "D4", "C2"),
                GameMove::menace_of('♘', "D4", "E2"),
                GameMove::default_of('♘', "D4", "B3"),
                GameMove::default_of('♘', "D4", "F3"),
                GameMove::default_of('♘', "D4", "B5"),
                GameMove::default_of('♘', "D4", "F5"),
                GameMove::capture_of('♘', "D4", "C6"),
                GameMove::capture_of('♘', "D4", "E6"),
            ])
        );
        assert_eq!(
            try_game_move_vec_from_str(
                &mode.bounds,
                [
                    "        ",
                    "        ",
                    "     ◍○◌",
                    "      ♙ ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                ]
            ),
            Ok(vec![
                GameMove::en_passant_of('♙', "G5", "F6"),
                GameMove::default_of('♙', "G5", "G6"),
                GameMove::menace_of('♙', "G5", "H6"),
            ])
        );
    }

    #[test]
    fn test_game_move_vec_to_string_standard_chess() {
        let mode = standard_chess();
        assert_eq!(
            game_move_vec_to_string(
                &mode.bounds,
                &vec![
                    GameMove::default_of('♘', "D4", "B3"),
                    GameMove::default_of('♘', "D4", "B5"),
                    GameMove::default_of('♘', "D4", "F3"),
                    GameMove::default_of('♘', "D4", "F5"),
                    GameMove::menace_of('♘', "D4", "C2"),
                    GameMove::menace_of('♘', "D4", "E2"),
                    GameMove::capture_of('♘', "D4", "C6"),
                    GameMove::capture_of('♘', "D4", "E6"),
                ]
            ),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "  ◎ ◎   \n"
                + " ○   ○  \n"
                + "   ♘    \n"
                + " ○   ○  \n"
                + "  ◌ ◌   \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(
                &mode.bounds,
                &vec![
                    GameMove::en_passant_of('♙', "G5", "F6"),
                    GameMove::default_of('♙', "G5", "G6"),
                    GameMove::menace_of('♙', "G5", "H6"),
                ]
            ),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "     ◍○◌\n"
                + "      ♙ \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(
                &mode.bounds,
                &vec![
                    GameMove::menace_of('♙', "B7", "A8"),
                    GameMove::menace_of('♙', "B7", "C8"),
                    GameMove::promotion_to_queen_of('♙', "B7", "B8"),
                ]
            ),
            "".to_owned()
                + "◌●◌     \n"
                + " ♙      \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
        );
        assert_eq!(
            game_move_vec_to_string(
                &mode.bounds,
                &vec![
                    GameMove::default_of('♔', "E1", "F2"),
                    GameMove::default_of('♔', "E1", "F1"),
                    GameMove::default_of('♔', "E1", "D1"),
                    GameMove::default_of('♔', "E1", "D2"),
                    GameMove::default_of('♔', "E1", "E2"),
                    GameMove::long_castling_of('♔', "E1", "A1"),
                    GameMove::short_castling_of('♔', "E1", "H1"),
                ]
            ),
            "".to_owned()
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "        \n"
                + "   ○○○  \n"
                + "✚  ○♔○ ✚\n"
        );
    }

    #[test]
    fn game_move_from() {
        assert_eq!(GameMoveType::from(DefaultMove), GameMoveType::Default(DefaultMove));
        assert_eq!(GameMoveType::from(CaptureMove), GameMoveType::Capture(CaptureMove));
        assert_eq!(GameMoveType::from(MenaceMove), GameMoveType::Menace(MenaceMove));
        assert_eq!(GameMoveType::from(EnPassantMove), GameMoveType::EnPassant(EnPassantMove));
        assert_eq!(
            GameMoveType::from(LongCastlingMove),
            GameMoveType::LongCastling(LongCastlingMove)
        );
        assert_eq!(
            GameMoveType::from(ShortCastlingMove),
            GameMoveType::ShortCastling(ShortCastlingMove)
        );
        assert_eq!(
            GameMoveType::from(PromotionToQueenMove),
            GameMoveType::PromotionToQueen(PromotionToQueenMove)
        );
        assert_eq!(
            GameMoveType::from(PromotionToRookMove),
            GameMoveType::PromotionToRook(PromotionToRookMove)
        );
        assert_eq!(
            GameMoveType::from(PromotionToBishopMove),
            GameMoveType::PromotionToBishop(PromotionToBishopMove)
        );
        assert_eq!(
            GameMoveType::from(PromotionToKnightMove),
            GameMoveType::PromotionToKnight(PromotionToKnightMove)
        );
    }

    #[test]
    fn game_move() {
        assert_eq!(GameMoveType::default(), GameMoveType::Default(DefaultMove));
        assert_eq!(GameMoveType::capture(), GameMoveType::Capture(CaptureMove));
        assert_eq!(GameMoveType::menace(), GameMoveType::Menace(MenaceMove));
        assert_eq!(GameMoveType::en_passant(), GameMoveType::EnPassant(EnPassantMove));
        assert_eq!(GameMoveType::long_castling(), GameMoveType::LongCastling(LongCastlingMove));
        assert_eq!(GameMoveType::short_castling(), GameMoveType::ShortCastling(ShortCastlingMove));
        assert_eq!(
            GameMoveType::promotion_to_queen(),
            GameMoveType::PromotionToQueen(PromotionToQueenMove)
        );
        assert_eq!(
            GameMoveType::promotion_to_rook(),
            GameMoveType::PromotionToRook(PromotionToRookMove)
        );
        assert_eq!(
            GameMoveType::promotion_to_bishop(),
            GameMoveType::PromotionToBishop(PromotionToBishopMove)
        );
        assert_eq!(
            GameMoveType::promotion_to_knight(),
            GameMoveType::PromotionToKnight(PromotionToKnightMove)
        );
    }

    #[test]
    fn game_move_try_of() {
        assert_eq!(
            GameMove::try_of('♙', "A2", "A4", GameMoveType::default()),
            Some(GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::default() })
        );
        assert_eq!(
            GameMove::try_of('♙', "D4", "E5", GameMoveType::capture()),
            Some(GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::capture() })
        );
        assert_eq!(
            GameMove::try_of('♘', "D6", "C8", GameMoveType::menace()),
            Some(GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::menace() })
        );
        assert_eq!(
            GameMove::try_of('♙', "G6", "F7", GameMoveType::en_passant()),
            Some(GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::en_passant() })
        );
        assert_eq!(
            GameMove::try_of('♔', "E1", "A1", GameMoveType::long_castling()),
            Some(GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::long_castling() })
        );
        assert_eq!(
            GameMove::try_of('♔', "E1", "H1", GameMoveType::short_castling()),
            Some(GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::short_castling() })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::promotion_to_queen()),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_queen()
            })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::promotion_to_rook()),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_rook()
            })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::promotion_to_bishop()),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_bishop()
            })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::promotion_to_knight()),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_knight()
            })
        );
    }

    #[test]
    fn game_move_of() {
        assert_eq!(
            GameMove::of('♙', "A2", "A4", GameMoveType::default()),
            GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::default() }
        );
        assert_eq!(
            GameMove::of('♙', "D4", "E5", GameMoveType::capture()),
            GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::capture() }
        );
        assert_eq!(
            GameMove::of('♘', "D6", "C8", GameMoveType::menace()),
            GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::menace() }
        );
        assert_eq!(
            GameMove::of('♙', "G6", "F7", GameMoveType::en_passant()),
            GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::en_passant() }
        );
        assert_eq!(
            GameMove::of('♔', "E1", "A1", GameMoveType::long_castling()),
            GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::long_castling() }
        );
        assert_eq!(
            GameMove::of('♔', "E1", "H1", GameMoveType::short_castling()),
            GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::short_castling() }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::promotion_to_queen()),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_queen() }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::promotion_to_bishop()),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_bishop() }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::promotion_to_bishop()),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_bishop() }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::promotion_to_knight()),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_knight() }
        );
    }

    #[test]
    fn game_move_default_try_of() {
        assert_eq!(
            GameMove::default_try_of('♙', "A2", "A4"),
            Some(GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::default() })
        );
    }

    #[test]
    fn game_move_default_of() {
        assert_eq!(
            GameMove::default_of('♙', "A2", "A4"),
            GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::default() }
        );
    }

    #[test]
    fn game_move_capture_try_of() {
        assert_eq!(
            GameMove::capture_try_of('♙', "D4", "E5"),
            Some(GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::capture() })
        );
    }

    #[test]
    fn game_move_capture_of() {
        assert_eq!(
            GameMove::capture_of('♙', "D4", "E5"),
            GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::capture() }
        );
    }

    #[test]
    fn game_move_menace_try_of() {
        assert_eq!(
            GameMove::menace_try_of('♘', "D6", "C8"),
            Some(GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::menace() })
        );
    }

    #[test]
    fn game_move_menace_of() {
        assert_eq!(
            GameMove::menace_of('♘', "D6", "C8"),
            GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::menace() }
        );
    }

    #[test]
    fn game_move_en_passant_try_of() {
        assert_eq!(
            GameMove::en_passant_try_of('♙', "G6", "F7"),
            Some(GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::en_passant() })
        );
    }

    #[test]
    fn game_move_en_passant_of() {
        assert_eq!(
            GameMove::en_passant_of('♙', "G6", "F7"),
            GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::en_passant() }
        );
    }

    #[test]
    fn game_move_long_castling_try_of() {
        assert_eq!(
            GameMove::long_castling_try_of('♔', "E1", "A1"),
            Some(GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::long_castling() })
        );
    }

    #[test]
    fn game_move_long_castling_of() {
        assert_eq!(
            GameMove::long_castling_of('♔', "E1", "A1"),
            GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::long_castling() }
        );
    }

    #[test]
    fn game_move_short_castling_try_of() {
        assert_eq!(
            GameMove::short_castling_try_of('♔', "E1", "H1"),
            Some(GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::short_castling() })
        );
    }

    #[test]
    fn game_move_short_castling_of() {
        assert_eq!(
            GameMove::short_castling_of('♔', "E1", "H1"),
            GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::short_castling() }
        );
    }

    #[test]
    fn game_move_promotion_to_queen_try_of() {
        assert_eq!(
            GameMove::promotion_to_queen_try_of('♙', "H7", "H8"),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_queen()
            })
        );
    }

    #[test]
    fn game_move_promotion_to_queen_of() {
        assert_eq!(
            GameMove::promotion_to_queen_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_queen() }
        );
    }

    #[test]
    fn game_move_promotion_to_rook_try_of() {
        assert_eq!(
            GameMove::promotion_to_rook_try_of('♙', "H7", "H8"),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_rook()
            })
        );
    }

    #[test]
    fn game_move_promotion_to_rook_of() {
        assert_eq!(
            GameMove::promotion_to_rook_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_rook() }
        );
    }

    #[test]
    fn game_move_promotion_to_bishop_try_of() {
        assert_eq!(
            GameMove::promotion_to_bishop_try_of('♙', "H7", "H8"),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_bishop()
            })
        );
    }

    #[test]
    fn game_move_promotion_to_bishop_of() {
        assert_eq!(
            GameMove::promotion_to_bishop_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_bishop() }
        );
    }

    #[test]
    fn game_move_promotion_to_knight_try_of() {
        assert_eq!(
            GameMove::promotion_to_knight_try_of('♙', "H7", "H8"),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"),
                typ: GameMoveType::promotion_to_knight()
            })
        );
    }

    #[test]
    fn game_move_promotion_to_knight_of() {
        assert_eq!(
            GameMove::promotion_to_knight_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::promotion_to_knight() }
        );
    }
}
