use crate::{board::pos::Pos, game_board::Board};

pub fn naive_movements_knight(board: &Board, pos: &Pos) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();
    if let Some(piece) = board.get(&pos) {
        let base = [
            pos.try_of_rel_idx(2, 1),
            pos.try_of_rel_idx(1, 2),
            pos.try_of_rel_idx(-1, 2),
            pos.try_of_rel_idx(-2, 1),
            pos.try_of_rel_idx(-2, -1),
            pos.try_of_rel_idx(-1, -2),
            pos.try_of_rel_idx(1, -2),
            pos.try_of_rel_idx(2, -1),
        ];
        for curr_pos in base {
            if let Some(curr_pos) = curr_pos {
                if let Some(curr_piece) = board.get(&curr_pos) {
                    if &curr_piece.color != &piece.color {
                        result.push(curr_pos);
                    }
                } else {
                    result.push(curr_pos);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{board::pos::Pos, game_board};

    use super::naive_movements_knight;

    #[test]
    fn naive_movements_knight_empty_board() {
        assert_eq!(naive_movements_knight(&game_board::empty(), &Pos::of_str("A1")), []);
    }

    #[test]
    fn naive_movements_knight_lonely_piece() {
        assert_eq!(
            naive_movements_knight(
                &game_board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "   ♞    ",
                    "        ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
                Pos::of_str("C2"),
                Pos::of_str("B3"),
                Pos::of_str("B5"),
                Pos::of_str("C6"),
            ]
        );
    }

    #[test]
    fn naive_movements_knight_edge() {
        assert_eq!(
            naive_movements_knight(
                &game_board::of_str([
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "♞       ",
                ]),
                &Pos::of_str("A1"),
            ),
            [Pos::of_str("B3"), Pos::of_str("C2")]
        );
    }

    #[test]
    fn naive_movements_knight_with_capture() {
        assert_eq!(
            naive_movements_knight(
                &game_board::of_str([
                    "        ",
                    "        ",
                    "    ♖   ",
                    "     ♖  ",
                    "   ♞    ",
                    " ♜      ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
                Pos::of_str("C2"),
                Pos::of_str("B5"),
                Pos::of_str("C6"),
            ]
        );
        assert_eq!(
            naive_movements_knight(
                &game_board::of_str([
                    "        ",
                    "        ",
                    "    ♜   ",
                    "     ♜  ",
                    "   ♘    ",
                    " ♖      ",
                    "        ",
                    "        ",
                ]),
                &Pos::of_str("D4"),
            ),
            [
                Pos::of_str("E6"),
                Pos::of_str("F5"),
                Pos::of_str("F3"),
                Pos::of_str("E2"),
                Pos::of_str("C2"),
                Pos::of_str("B5"),
                Pos::of_str("C6"),
            ]
        );
    }
}
