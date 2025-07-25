use std::collections::HashMap;

use crate::{color::Color, game::mov::PieceMoveType, pos::Pos};

use super::capture::GameCapture;

pub type PlayerMoves = HashMap<Pos, HashMap<Pos, PieceMoveType>>;

#[derive(Debug, PartialEq, Clone)]
pub struct GamePlayer {
    pub color: Color,
    pub captures: Vec<GameCapture>,
    pub moves: PlayerMoves,
}

impl From<Color> for GamePlayer {
    fn from(color: Color) -> Self {
        GamePlayer { color, captures: Vec::new(), moves: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::color::Color;

    use super::GamePlayer;

    #[test]
    fn game_player() {
        assert_eq!(
            GamePlayer::from(Color::White),
            GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() }
        );
        assert_eq!(
            GamePlayer::from(Color::Black),
            GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() }
        );
    }
}
