use crate::domain::{board::BoardPos, piece::Piece};

use super::board::{get_initial_white_board, Board};

#[derive(Debug, PartialEq, Clone)]
pub struct PlayMove {
    p: Piece,
    from: BoardPos,
    to: BoardPos,
}

impl PlayMove {
    fn try_of_str(p: &str, from: &str, to: &str) -> Option<Self> {
        let p = Piece::try_of_str(p)?;
        let from = BoardPos::try_of_str(from)?;
        let to = BoardPos::try_of_str(to)?;
        Some(PlayMove { p, from, to })
    }

    fn of_str(p: &str, from: &str, to: &str) -> Self {
        Self::try_of_str(p, from, to).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Play {
    pub b: Board,
    pub moves: Vec<PlayMove>,
}

impl Default for Play {
    fn default() -> Self {
        Play { b: get_initial_white_board(), moves: Vec::new() }
    }
}

fn is_white_turn(p: &Play) -> bool {
    p.moves.len() % 2 == 0
}

fn is_black_turn(p: &Play) -> bool {
    p.moves.len() % 2 == 1
}

mod test {
    use super::*;

    #[test]
    fn test_play_move_try_of_str_some() {
        assert_eq!(
            PlayMove::try_of_str("♟", "D2", "D4"),
            Some(PlayMove {
                p: Piece::of_str("♟"),
                from: BoardPos::of_str("D2"),
                to: BoardPos::of_str("D4")
            })
        );
    }

    #[test]
    fn test_play_move_try_of_str_none() {
        assert_eq!(PlayMove::try_of_str("P", "D2", "D4"), None);
        assert_eq!(PlayMove::try_of_str("♟", "K9", "D4"), None);
        assert_eq!(PlayMove::try_of_str("♟", "D2", "K9"), None);
    }

    #[test]
    fn test_play_move_of_str() {
        assert_eq!(
            PlayMove::of_str("♟", "D2", "D4"),
            PlayMove {
                p: Piece::of_str("♟"),
                from: BoardPos::of_str("D2"),
                to: BoardPos::of_str("D4")
            }
        );
    }

    #[test]
    fn test_is_white_turn() {
        assert_eq!(is_white_turn(&Play::default()), true);
        assert_eq!(
            is_white_turn(&Play {
                moves: Vec::from([PlayMove::of_str("♟", "D2", "D4")]),
                ..Default::default()
            }),
            false
        );
    }

    #[test]
    fn test_is_black_turn() {
        assert_eq!(is_black_turn(&Play::default()), false);
        assert_eq!(
            is_black_turn(&Play {
                moves: Vec::from([PlayMove::of_str("♟", "D2", "D4")]),
                ..Default::default()
            }),
            true
        );
    }
}
