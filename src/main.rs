use macroquad::prelude::*;

const GRID_SIZE: usize = 200;

fn draw_point(x: isize, y: isize) {
    let (width, height) = (screen_width(), screen_height());
    let grid_size = GRID_SIZE as f32;
    let scaling_factor = (width / grid_size, height / grid_size);
    let x = x as f32 + grid_size / 2.;
    let y = grid_size / 2. - y as f32;
    draw_rectangle(
        x as f32 * scaling_factor.0,
        y as f32 * scaling_factor.1,
        scaling_factor.0,
        scaling_factor.1,
        WHITE,
    );
}

#[macroquad::main("Circle Rasterizer")]
async fn main() {
    let r = 80;
    let signed_distance_field = |x, y| x * x + y * y - r * r;
    loop {
        let mut x = r;
        let mut y = 0;
        while y < r * 10 / 14 {
            draw_point(x, y);
            draw_point(-x, y);
            draw_point(x, -y);
            draw_point(-x, -y);
            draw_point(y, x);
            draw_point(-y, x);
            draw_point(y, -x);
            draw_point(-y, -x);
            let d1 = signed_distance_field(x, y + 1);
            let d2 = signed_distance_field(x - 1, y + 1);
            y += 1;
            if d2.abs() < d1.abs() {
                x -= 1;
            }
        }
        next_frame().await;
    }
}
