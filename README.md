# circle_rasterization
Circle Rasterization algorithm in rust inspired by [Casey Muratori](https://www.youtube.com/watch?v=JtgQJT08J1g)  
Based on Bresenham's algorithm  

## Building
1. Install [rust](https://rust-lang.org)
2. Build and Run using `cargo`  
    `cargo run --release`

## Controls
The viewport is rasterized into a square grid with the circle radius being normalized to the grid.  
Use mouse wheel to adjust the radius of the circle and `+` or `-` key to adjust the grid size.  
To switch between light and dark mode, use the `L` and `D` key.

## Screenshot
![screen_shot](https://github.com/ArchUsr64/circle_rasterization/assets/83179501/94a72a9d-25bd-4f24-ae5b-21f08b4f99b0)
