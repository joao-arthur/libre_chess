use manfredo::cartesian::rect::rect_f64::RectF64;

use libre_chess_lib::{
    game::{board::GameBoard, game::GameBounds},
    piece::Piece,
    pos::Pos,
};

#[derive(Debug, PartialEq)]
pub struct RenderSettings {
    pub dim: u16,
}

#[derive(Debug, PartialEq)]
pub struct ValueToRender {
    pub piece: Piece,
    pub rect: RectF64,
}

pub fn get_values_to_render(
    board: &GameBoard,
    bounds: &GameBounds,
    settings: &RenderSettings,
) -> Vec<ValueToRender> {
    let mut values_to_render: Vec<ValueToRender> = Vec::new();
    let cell_size = settings.dim as f64 / 8.0;
    for row in bounds.iter_row() {
        for col in bounds.iter_col() {
            if let Some(piece) = board.get(&Pos { row, col }) {
                values_to_render.push(ValueToRender {
                    piece: *piece,
                    rect: RectF64::of(
                        (col as f64) * cell_size,
                        (settings.dim as f64) - (row as f64) * cell_size - cell_size,
                        (col as f64) * cell_size + cell_size,
                        (settings.dim as f64) - (row as f64) * cell_size,
                    ),
                });
            }
        }
    }
    values_to_render.sort_by(|a, b| {
        a.rect.min.x.partial_cmp(&b.rect.min.x).unwrap_or(std::cmp::Ordering::Greater)
    });
    values_to_render.sort_by(|a, b| {
        a.rect.min.y.partial_cmp(&b.rect.min.y).unwrap_or(std::cmp::Ordering::Greater)
    });
    values_to_render
}

#[cfg(test)]
mod tests {

    use libre_chess_lib::game::mode::standard_chess;

    use super::*;

    #[test]
    fn test_get_values_to_render() {
        let mode = standard_chess();
        let settings = RenderSettings { dim: 987 };
        assert_eq!(
            get_values_to_render(&mode.initial_board, &mode.bounds, &settings),
            [
                ValueToRender { piece: Piece::of('♜'), rect: RectF64::of(0.0, 0.0, 123.375, 123.375) },
                ValueToRender { piece: Piece::of('♞'), rect: RectF64::of(123.375, 0.0, 246.75, 123.375) },
                ValueToRender { piece: Piece::of('♝'), rect: RectF64::of(246.75, 0.0, 370.125, 123.375) },
                ValueToRender { piece: Piece::of('♛'), rect: RectF64::of(370.125, 0.0, 493.5, 123.375) },
                ValueToRender { piece: Piece::of('♚'), rect: RectF64::of(493.5, 0.0, 616.875, 123.375) },
                ValueToRender { piece: Piece::of('♝'), rect: RectF64::of(616.875, 0.0, 740.25, 123.375) },
                ValueToRender { piece: Piece::of('♞'), rect: RectF64::of(740.25, 0.0, 863.625, 123.375) },
                ValueToRender { piece: Piece::of('♜'), rect: RectF64::of(863.625, 0.0, 987.0, 123.375) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(0.0, 123.375, 123.375, 246.75) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(123.375, 123.375, 246.75, 246.75) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(246.75, 123.375, 370.125, 246.75) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(370.125, 123.375, 493.5, 246.75) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(493.5, 123.375, 616.875, 246.75) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(616.875, 123.375, 740.25, 246.75) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(740.25, 123.375, 863.625, 246.75) },
                ValueToRender { piece: Piece::of('♟'), rect: RectF64::of(863.625, 123.375, 987.0, 246.75) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(0.0, 740.25, 123.375, 863.625) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(123.375, 740.25, 246.75, 863.625) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(246.75, 740.25, 370.125, 863.625) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(370.125, 740.25, 493.5, 863.625) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(493.5, 740.25, 616.875, 863.625) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(616.875, 740.25, 740.25, 863.625) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(740.25, 740.25, 863.625, 863.625) },
                ValueToRender { piece: Piece::of('♙'), rect: RectF64::of(863.625, 740.25, 987.0, 863.625) },
                ValueToRender { piece: Piece::of('♖'), rect: RectF64::of(0.0, 863.625, 123.375, 987.0) },
                ValueToRender { piece: Piece::of('♘'), rect: RectF64::of(123.375, 863.625, 246.75, 987.0) },
                ValueToRender { piece: Piece::of('♗'), rect: RectF64::of(246.75, 863.625, 370.125, 987.0) },
                ValueToRender { piece: Piece::of('♕'), rect: RectF64::of(370.125, 863.625, 493.5, 987.0) },
                ValueToRender { piece: Piece::of('♔'), rect: RectF64::of(493.5, 863.625, 616.875, 987.0) },
                ValueToRender { piece: Piece::of('♗'), rect: RectF64::of(616.875, 863.625, 740.25, 987.0) },
                ValueToRender { piece: Piece::of('♘'), rect: RectF64::of(740.25, 863.625, 863.625, 987.0) },
                ValueToRender { piece: Piece::of('♖'), rect: RectF64::of(863.625, 863.625, 987.0, 987.0) },
            ]
        );
    }
}
