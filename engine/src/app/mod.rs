use web_sys::CanvasRenderingContext2d;


pub fn render(context: CanvasRenderingContext2d) {
    let mut acc = 0;
    for row in 0..8 {
        acc += 1;
        for col in 0..8 {
            if acc % 2 == 0 {
                context.set_fill_style(&"blue".into());
            } else {
                context.set_fill_style(&"red".into());
            }
            acc += 1;

            context.fill_rect(col as f64 * 40.0, row as f64* 40.0, 40 as f64, 40 as f64);



        }
    }
}
