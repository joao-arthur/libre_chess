use crate::color::Color;

use super::play::Play;

pub fn get_turn(play: &Play) -> Color {
    if play.history.len() % 2 == 0 { Color::White } else { Color::Black }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        game::{movement::Movement, play::Play},
    };

    use super::get_turn;

    #[test]
    fn test_get_turn() {
        assert_eq!(get_turn(&Play { history: Vec::new(), ..Default::default() }), Color::White);
        assert_eq!(
            get_turn(&Play {
                history: Vec::from([Movement::of_str("♟", "D2", "D4")]),
                ..Default::default()
            }),
            Color::Black
        );
        assert_eq!(
            get_turn(&Play {
                history: Vec::from([
                    Movement::of_str("♟", "D2", "D4"),
                    Movement::of_str("♟", "A2", "A3")
                ]),
                ..Default::default()
            }),
            Color::White
        );
    }
}
