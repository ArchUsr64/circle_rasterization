use macroquad::prelude::*;

fn draw_point(x: isize, y: isize, grid_size: usize) {
    let (width, height) = (screen_width(), screen_height());
    let grid_size = grid_size as f32;
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
    let mut r_normalized = 0.8f32;
    let mut grid_size = 200;
    loop {
        let font_size = screen_height() / 20.;
        draw_text(
            format!("Radius: {r_normalized:.02}").as_str(),
            0.,
            font_size,
            font_size,
            WHITE,
        );
        draw_text(
            format!("Grid Size: {grid_size}").as_str(),
            0.,
            font_size * 2.,
            font_size,
            WHITE,
        );
        r_normalized += mouse_wheel().1 / 100.;
        let r = (r_normalized.abs() * grid_size as f32) as isize / 2;
        if is_key_down(KeyCode::Equal) {
            grid_size += 1;
        } else if is_key_down(KeyCode::Minus) {
            grid_size -= 1;
        }
        let signed_distance_field = |x, y| x * x + y * y - r * r;
        let mut x = r;
        let mut y = 0;
        while y < (r * 10 / 14) + 1 {
            draw_point(x, y, grid_size);
            draw_point(-x, y, grid_size);
            draw_point(x, -y, grid_size);
            draw_point(-x, -y, grid_size);
            draw_point(y, x, grid_size);
            draw_point(-y, x, grid_size);
            draw_point(y, -x, grid_size);
            draw_point(-y, -x, grid_size);
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
