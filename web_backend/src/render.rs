use libre_chess_lib::{board::pos::Pos, game::GameBoard, piece::Piece};

#[derive(Debug, PartialEq)]
pub struct RectF64 {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Debug, PartialEq)]
pub struct RenderSettings {
    pub dim: u16,
}

#[derive(Debug, PartialEq)]
pub struct ValueToRender {
    pub piece: Piece,
    pub rect: RectF64,
}

pub fn get_values_to_render(board: &GameBoard, settings: &RenderSettings) -> Vec<ValueToRender> {
    let mut values_to_render: Vec<ValueToRender> = Vec::new();
    let cell_size = settings.dim as f64 / 8.0;
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.get(&Pos { row, col }) {
                values_to_render.push(ValueToRender {
                    piece: piece.clone(),
                    rect: RectF64 {
                        x1: (col as f64) * cell_size,
                        y1: (settings.dim as f64) - (row as f64) * cell_size - cell_size,
                        x2: (col as f64) * cell_size + cell_size,
                        y2: (settings.dim as f64) - (row as f64) * cell_size,
                    },
                });
            }
        }
    }
    values_to_render
        .sort_by(|a, b| a.rect.x1.partial_cmp(&b.rect.x1).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render
        .sort_by(|a, b| a.rect.y1.partial_cmp(&b.rect.y1).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render
}

#[cfg(test)]
mod tests {

    use libre_chess_lib::game::mode::standard_chess;

    use super::*;

    #[test]
    fn test_get_values_to_render() {
        let board = standard_chess().initial_board;
        let settings = RenderSettings { dim: 987 };
        assert_eq!(
            get_values_to_render(&board, &settings),
            [
                ValueToRender { piece: Piece::of_str("♜"), rect: RectF64 { x1: 0.0, y1: 0.0, x2: 123.375, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♞"), rect: RectF64 { x1: 123.375, y1: 0.0, x2: 246.75, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♝"), rect: RectF64 { x1: 246.75, y1: 0.0, x2: 370.125, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♛"), rect: RectF64 { x1: 370.125, y1: 0.0, x2: 493.5, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♚"), rect: RectF64 { x1: 493.5, y1: 0.0, x2: 616.875, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♝"), rect: RectF64 { x1: 616.875, y1: 0.0, x2: 740.25, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♞"), rect: RectF64 { x1: 740.25, y1: 0.0, x2: 863.625, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♜"), rect: RectF64 { x1: 863.625, y1: 0.0, x2: 987.0, y2: 123.375 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 0.0, y1: 123.375, x2: 123.375, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 123.375, y1: 123.375, x2: 246.75, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 246.75, y1: 123.375, x2: 370.125, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 370.125, y1: 123.375, x2: 493.5, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 493.5, y1: 123.375, x2: 616.875, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 616.875, y1: 123.375, x2: 740.25, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 740.25, y1: 123.375, x2: 863.625, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♟"), rect: RectF64 { x1: 863.625, y1: 123.375, x2: 987.0, y2: 246.75 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 0.0, y1: 740.25, x2: 123.375, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 123.375, y1: 740.25, x2: 246.75, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 246.75, y1: 740.25, x2: 370.125, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 370.125, y1: 740.25, x2: 493.5, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 493.5, y1: 740.25, x2: 616.875, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 616.875, y1: 740.25, x2: 740.25, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 740.25, y1: 740.25, x2: 863.625, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♙"), rect: RectF64 { x1: 863.625, y1: 740.25, x2: 987.0, y2: 863.625 } },
                ValueToRender { piece: Piece::of_str("♖"), rect: RectF64 { x1: 0.0, y1: 863.625, x2: 123.375, y2: 987.0 } },
                ValueToRender { piece: Piece::of_str("♘"), rect: RectF64 { x1: 123.375, y1: 863.625, x2: 246.75, y2: 987.0 } },
                ValueToRender { piece: Piece::of_str("♗"), rect: RectF64 { x1: 246.75, y1: 863.625, x2: 370.125, y2: 987.0 } },
                ValueToRender { piece: Piece::of_str("♕"), rect: RectF64 { x1: 370.125, y1: 863.625, x2: 493.5, y2: 987.0 } },
                ValueToRender { piece: Piece::of_str("♔"), rect: RectF64 { x1: 493.5, y1: 863.625, x2: 616.875, y2: 987.0 } },
                ValueToRender { piece: Piece::of_str("♗"), rect: RectF64 { x1: 616.875, y1: 863.625, x2: 740.25, y2: 987.0 } },
                ValueToRender { piece: Piece::of_str("♘"), rect: RectF64 { x1: 740.25, y1: 863.625, x2: 863.625, y2: 987.0 } },
                ValueToRender { piece: Piece::of_str("♖"), rect: RectF64 { x1: 863.625, y1: 863.625, x2: 987.0, y2: 987.0 } },
            ]
        );
    }
}
