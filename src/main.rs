use macroquad::{
    color::WHITE,
    material::{gl_use_default_material, gl_use_material},
    math::{vec2, vec3},
    shapes::draw_rectangle,
    window::{next_frame, screen_height, screen_width},
};
use newton_fractal::NewtonFractal;

#[macroquad::main("Newton Fractal")]
async fn main() {
    let fractal = NewtonFractal::new(
        vec![
            vec2(1.0, 0.0),
            vec2(-0.5, 0.86602540378), 
            vec2(-0.5, -0.86602540378),
        ],
        vec![
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 1.0),
        ],
		10
    );

    if fractal.is_err() {
        panic!("Error creating fractal");
    }

    let fractal = fractal.unwrap();

    loop {
        gl_use_material(&fractal.get_material());
        draw_rectangle(-1.0, -1., screen_width(), screen_height(), WHITE);
        gl_use_default_material();
        next_frame().await
    }
}
