use libre_chess_lib::{
    board::{pos::Pos, Board},
    piece::Piece,
};

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
    pub p: Piece,
    pub rect: RectF64,
}

pub fn get_values_to_render(b: &Board, s: &RenderSettings) -> Vec<ValueToRender> {
    let mut values_to_render: Vec<ValueToRender> = Vec::new();
    let cell_size = s.dim as f64 / 8.0;
    for row in 0..8 {
        for col in 0..8 {
            let p = b[Pos::of_idx(row, col)];
            if let Some(p) = p {
                values_to_render.push(ValueToRender {
                    p,
                    rect: RectF64 {
                        x1: col as f64 * cell_size,
                        y1: row as f64 * cell_size,
                        x2: col as f64 * cell_size + cell_size,
                        y2: row as f64 * cell_size + cell_size,
                    },
                });
            }
        }
    }
    values_to_render.sort_by(|a, b| a.rect.x1.partial_cmp(&b.rect.x1).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render.sort_by(|a, b| a.rect.y1.partial_cmp(&b.rect.y1).unwrap_or(std::cmp::Ordering::Greater));
    values_to_render
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_values_to_render() {
        let u = Board::get_initial_board();
        let s = RenderSettings { dim: 987 };
        assert_eq!(
            get_values_to_render(&u, &s),
            vec![
                ValueToRender { p: Piece::of_str("♜"), rect: RectF64 { x1: 0.0, y1: 0.0, x2: 123.375, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♞"), rect: RectF64 { x1: 123.375, y1: 0.0, x2: 246.75, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♝"), rect: RectF64 { x1: 246.75, y1: 0.0, x2: 370.125, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♛"), rect: RectF64 { x1: 370.125, y1: 0.0, x2: 493.5, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♚"), rect: RectF64 { x1: 493.5, y1: 0.0, x2: 616.875, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♝"), rect: RectF64 { x1: 616.875, y1: 0.0, x2: 740.25, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♞"), rect: RectF64 { x1: 740.25, y1: 0.0, x2: 863.625, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♜"), rect: RectF64 { x1: 863.625, y1: 0.0, x2: 987.0, y2: 123.375 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 0.0, y1: 123.375, x2: 123.375, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 123.375, y1: 123.375, x2: 246.75, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 246.75, y1: 123.375, x2: 370.125, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 370.125, y1: 123.375, x2: 493.5, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 493.5, y1: 123.375, x2: 616.875, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 616.875, y1: 123.375, x2: 740.25, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 740.25, y1: 123.375, x2: 863.625, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♟"), rect: RectF64 { x1: 863.625, y1: 123.375, x2: 987.0, y2: 246.75 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 0.0, y1: 740.25, x2: 123.375, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 123.375, y1: 740.25, x2: 246.75, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 246.75, y1: 740.25, x2: 370.125, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 370.125, y1: 740.25, x2: 493.5, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 493.5, y1: 740.25, x2: 616.875, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 616.875, y1: 740.25, x2: 740.25, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 740.25, y1: 740.25, x2: 863.625, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♙"), rect: RectF64 { x1: 863.625, y1: 740.25, x2: 987.0, y2: 863.625 } },
                ValueToRender { p: Piece::of_str("♖"), rect: RectF64 { x1: 0.0, y1: 863.625, x2: 123.375, y2: 987.0 } },
                ValueToRender { p: Piece::of_str("♘"), rect: RectF64 { x1: 123.375, y1: 863.625, x2: 246.75, y2: 987.0 } },
                ValueToRender { p: Piece::of_str("♗"), rect: RectF64 { x1: 246.75, y1: 863.625, x2: 370.125, y2: 987.0 } },
                ValueToRender { p: Piece::of_str("♕"), rect: RectF64 { x1: 370.125, y1: 863.625, x2: 493.5, y2: 987.0 } },
                ValueToRender { p: Piece::of_str("♔"), rect: RectF64 { x1: 493.5, y1: 863.625, x2: 616.875, y2: 987.0 } },
                ValueToRender { p: Piece::of_str("♗"), rect: RectF64 { x1: 616.875, y1: 863.625, x2: 740.25, y2: 987.0 } },
                ValueToRender { p: Piece::of_str("♘"), rect: RectF64 { x1: 740.25, y1: 863.625, x2: 863.625, y2: 987.0 } },
                ValueToRender { p: Piece::of_str("♖"), rect: RectF64 { x1: 863.625, y1: 863.625, x2: 987.0, y2: 987.0 }  }
            ]
        );
    }
}
