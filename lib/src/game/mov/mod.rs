use manfredo::matrix::rect::rect_u8::{len_col, len_row};

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

pub enum HistoryMoveType {
    Default,
    Capture,
    Menace,
    EnPassant,
    LongCastling,
    ShortCastling,
    PromotionToQueen,
    PromotionToRook,
    PromotionToBishop,
    PromotionToKnight,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PieceMoveType {
    Default,
    EnPassant,
    LongCastling,
    ShortCastling,
    PromotionToQueen,
    PromotionToRook,
    PromotionToBishop,
    PromotionToKnight,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameMoveType {
    Default,
    Capture,
    Menace,
    EnPassant,
    LongCastling,
    ShortCastling,
    PromotionToQueen,
    PromotionToRook,
    PromotionToBishop,
    PromotionToKnight,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameMove {
    pub mov: Mov,
    pub typ: GameMoveType,
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
        Self::try_of(piece, from, to, GameMoveType::Default)
    }

    pub fn default_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::Default)
    }

    pub fn capture_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::Capture)
    }

    pub fn capture_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::Capture)
    }

    pub fn menace_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::Menace)
    }

    pub fn menace_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::Menace)
    }

    pub fn en_passant_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::EnPassant)
    }

    pub fn en_passant_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::EnPassant)
    }

    pub fn long_castling_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::LongCastling)
    }

    pub fn long_castling_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::LongCastling)
    }

    pub fn short_castling_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::ShortCastling)
    }

    pub fn short_castling_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::ShortCastling)
    }

    pub fn promotion_to_queen_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::PromotionToQueen)
    }

    pub fn promotion_to_queen_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::PromotionToQueen)
    }

    pub fn promotion_to_rook_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::PromotionToRook)
    }

    pub fn promotion_to_rook_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::PromotionToRook)
    }

    pub fn promotion_to_bishop_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::PromotionToBishop)
    }

    pub fn promotion_to_bishop_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::PromotionToBishop)
    }

    pub fn promotion_to_knight_try_of(piece: char, from: &str, to: &str) -> Option<Self> {
        Self::try_of(piece, from, to, GameMoveType::PromotionToKnight)
    }

    pub fn promotion_to_knight_of(piece: char, from: &str, to: &str) -> Self {
        Self::of(piece, from, to, GameMoveType::PromotionToKnight)
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
    if rows.len() != usize::from(len_row(bounds)) {
        return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
    }
    for row in rows {
        if row.chars().count() != usize::from(len_col(bounds)) {
            return Err(GameBoardErr::InvalidLength(InvalidLengthErr));
        }
    }
    let mut piece: Option<Piece> = None;
    let mut from: Option<Pos> = None;
    for row in bounds.iter_row() {
        for col in bounds.iter_col() {
            let row_index = bounds.max.row - row;
            let col_index = col - bounds.min.col;
            let str_row = rows[row_index as usize];
            let str_col = str_row.chars().nth(col_index.into()).unwrap();
            if let Some(p) = Piece::try_of(str_col) {
                piece = Some(p);
                from = Some(Pos::of(row, col));
                break;
            }
        }
    }
    let piece = piece.ok_or(GameBoardErr::InvalidCharacter(InvalidCharacterErr))?;
    let from = from.ok_or(GameBoardErr::InvalidCharacter(InvalidCharacterErr))?;
    let mut res = Vec::new();
    for row in bounds.iter_row() {
        for col in bounds.iter_col() {
            let row_index = bounds.max.row - row;
            let col_index = col - bounds.min.col;
            let str_row = rows[row_index as usize];
            let str_col = str_row.chars().nth(col_index.into()).unwrap();
            let to = Pos::of(row, col);
            match str_col {
                '○' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::Default,
                }),
                '◎' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::Capture,
                }),
                '◌' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::Menace,
                }),
                '◍' => res.push(GameMove {
                    mov: Mov { piece, from: from.clone(), to },
                    typ: GameMoveType::EnPassant,
                }),
                _ => {}
            }
        }
    }
    Ok(res)
}

pub fn game_move_vec_to_string(bounds: &GameBounds, moves: &Vec<GameMove>) -> String {
    let mut res = "".to_string();
    let mut row = bounds.max.row + 1;
    while row > bounds.min.row {
        row -= 1;
        let it_col = bounds.iter_col();
        for col in it_col {
            let pos = Pos::of(row, col);
            let maybe_mov = moves.iter().find(|game_move| game_move.mov.to == pos);
            if let Some(mov) = maybe_mov {
                match mov.typ {
                    GameMoveType::Default => res.push('○'),
                    GameMoveType::Capture => res.push('◎'),
                    GameMoveType::Menace => res.push('◌'),
                    GameMoveType::EnPassant => res.push('◍'),
                    GameMoveType::LongCastling | GameMoveType::ShortCastling => res.push('✚'),
                    GameMoveType::PromotionToQueen
                    | GameMoveType::PromotionToRook
                    | GameMoveType::PromotionToBishop
                    | GameMoveType::PromotionToKnight => res.push('●'),
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

    use super::{GameMove, GameMoveType, game_move_vec_to_string, try_game_move_vec_from_str};

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
    fn game_move_try_of() {
        assert_eq!(
            GameMove::try_of('♙', "A2", "A4", GameMoveType::Default),
            Some(GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::Default })
        );
        assert_eq!(
            GameMove::try_of('♙', "D4", "E5", GameMoveType::Capture),
            Some(GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::Capture })
        );
        assert_eq!(
            GameMove::try_of('♘', "D6", "C8", GameMoveType::Menace),
            Some(GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::Menace })
        );
        assert_eq!(
            GameMove::try_of('♙', "G6", "F7", GameMoveType::EnPassant),
            Some(GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::EnPassant })
        );
        assert_eq!(
            GameMove::try_of('♔', "E1", "A1", GameMoveType::LongCastling),
            Some(GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::LongCastling })
        );
        assert_eq!(
            GameMove::try_of('♔', "E1", "H1", GameMoveType::ShortCastling),
            Some(GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::ShortCastling })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::PromotionToQueen),
            Some(GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToQueen })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::PromotionToRook),
            Some(GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToRook })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::PromotionToBishop),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToBishop
            })
        );
        assert_eq!(
            GameMove::try_of('♙', "H7", "H8", GameMoveType::PromotionToKnight),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToKnight
            })
        );
    }

    #[test]
    fn game_move_of() {
        assert_eq!(
            GameMove::of('♙', "A2", "A4", GameMoveType::Default),
            GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::Default }
        );
        assert_eq!(
            GameMove::of('♙', "D4", "E5", GameMoveType::Capture),
            GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::Capture }
        );
        assert_eq!(
            GameMove::of('♘', "D6", "C8", GameMoveType::Menace),
            GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::Menace }
        );
        assert_eq!(
            GameMove::of('♙', "G6", "F7", GameMoveType::EnPassant),
            GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::EnPassant }
        );
        assert_eq!(
            GameMove::of('♔', "E1", "A1", GameMoveType::LongCastling),
            GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::LongCastling }
        );
        assert_eq!(
            GameMove::of('♔', "E1", "H1", GameMoveType::ShortCastling),
            GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::ShortCastling }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::PromotionToQueen),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToQueen }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::PromotionToBishop),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToBishop }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::PromotionToBishop),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToBishop }
        );
        assert_eq!(
            GameMove::of('♙', "H7", "H8", GameMoveType::PromotionToKnight),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToKnight }
        );
    }

    #[test]
    fn game_move_default_try_of() {
        assert_eq!(
            GameMove::default_try_of('♙', "A2", "A4"),
            Some(GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::Default })
        );
    }

    #[test]
    fn game_move_default_of() {
        assert_eq!(
            GameMove::default_of('♙', "A2", "A4"),
            GameMove { mov: Mov::of('♙', "A2", "A4"), typ: GameMoveType::Default }
        );
    }

    #[test]
    fn game_move_capture_try_of() {
        assert_eq!(
            GameMove::capture_try_of('♙', "D4", "E5"),
            Some(GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::Capture })
        );
    }

    #[test]
    fn game_move_capture_of() {
        assert_eq!(
            GameMove::capture_of('♙', "D4", "E5"),
            GameMove { mov: Mov::of('♙', "D4", "E5"), typ: GameMoveType::Capture }
        );
    }

    #[test]
    fn game_move_menace_try_of() {
        assert_eq!(
            GameMove::menace_try_of('♘', "D6", "C8"),
            Some(GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::Menace })
        );
    }

    #[test]
    fn game_move_menace_of() {
        assert_eq!(
            GameMove::menace_of('♘', "D6", "C8"),
            GameMove { mov: Mov::of('♘', "D6", "C8"), typ: GameMoveType::Menace }
        );
    }

    #[test]
    fn game_move_en_passant_try_of() {
        assert_eq!(
            GameMove::en_passant_try_of('♙', "G6", "F7"),
            Some(GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::EnPassant })
        );
    }

    #[test]
    fn game_move_en_passant_of() {
        assert_eq!(
            GameMove::en_passant_of('♙', "G6", "F7"),
            GameMove { mov: Mov::of('♙', "G6", "F7"), typ: GameMoveType::EnPassant }
        );
    }

    #[test]
    fn game_move_long_castling_try_of() {
        assert_eq!(
            GameMove::long_castling_try_of('♔', "E1", "A1"),
            Some(GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::LongCastling })
        );
    }

    #[test]
    fn game_move_long_castling_of() {
        assert_eq!(
            GameMove::long_castling_of('♔', "E1", "A1"),
            GameMove { mov: Mov::of('♔', "E1", "A1"), typ: GameMoveType::LongCastling }
        );
    }

    #[test]
    fn game_move_short_castling_try_of() {
        assert_eq!(
            GameMove::short_castling_try_of('♔', "E1", "H1"),
            Some(GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::ShortCastling })
        );
    }

    #[test]
    fn game_move_short_castling_of() {
        assert_eq!(
            GameMove::short_castling_of('♔', "E1", "H1"),
            GameMove { mov: Mov::of('♔', "E1", "H1"), typ: GameMoveType::ShortCastling }
        );
    }

    #[test]
    fn game_move_promotion_to_queen_try_of() {
        assert_eq!(
            GameMove::promotion_to_queen_try_of('♙', "H7", "H8"),
            Some(GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToQueen })
        );
    }

    #[test]
    fn game_move_promotion_to_queen_of() {
        assert_eq!(
            GameMove::promotion_to_queen_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToQueen }
        );
    }

    #[test]
    fn game_move_promotion_to_rook_try_of() {
        assert_eq!(
            GameMove::promotion_to_rook_try_of('♙', "H7", "H8"),
            Some(GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToRook })
        );
    }

    #[test]
    fn game_move_promotion_to_rook_of() {
        assert_eq!(
            GameMove::promotion_to_rook_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToRook }
        );
    }

    #[test]
    fn game_move_promotion_to_bishop_try_of() {
        assert_eq!(
            GameMove::promotion_to_bishop_try_of('♙', "H7", "H8"),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToBishop
            })
        );
    }

    #[test]
    fn game_move_promotion_to_bishop_of() {
        assert_eq!(
            GameMove::promotion_to_bishop_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToBishop }
        );
    }

    #[test]
    fn game_move_promotion_to_knight_try_of() {
        assert_eq!(
            GameMove::promotion_to_knight_try_of('♙', "H7", "H8"),
            Some(GameMove {
                mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToKnight
            })
        );
    }

    #[test]
    fn game_move_promotion_to_knight_of() {
        assert_eq!(
            GameMove::promotion_to_knight_of('♙', "H7", "H8"),
            GameMove { mov: Mov::of('♙', "H7", "H8"), typ: GameMoveType::PromotionToKnight }
        );
    }
}
