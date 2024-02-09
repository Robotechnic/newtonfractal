use egui::RichText;
use macroquad::{
    color::*,
    input::{is_key_down, is_mouse_button_down, mouse_position},
    material::{gl_use_default_material, gl_use_material},
    math::vec2,
    rand::{self},
    shapes::{draw_circle, draw_rectangle},
    window::{next_frame, screen_height, screen_width},
};
use newton_fractal::NewtonFractal;

const ROOT_RADIUS: f32 = 8.;

fn map(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

fn draw_roots(fractal: &mut NewtonFractal) {
    let pos = mouse_position();
    let real_range = fractal.get_real_range();
    let imag_range = fractal.get_imag_range();
    for root in fractal.get_roots() {
        let x = map(root.x, real_range.x, real_range.y, 0.0, screen_width());
        let y = map(root.y, imag_range.x, imag_range.y, 0.0, screen_height());
        if (pos.0 - x).abs() < ROOT_RADIUS * 1.2 && (pos.1 - y).abs() < ROOT_RADIUS * 1.2 {
            draw_circle(x, y, ROOT_RADIUS, WHITE);
        } else {
            draw_circle(x, y, ROOT_RADIUS, BLACK);
        }
    }
}

// X^3 - 1
// vec![
//     vec2(1.0, 0.0),
//     vec2(-0.5, 0.866_025_4),
//     vec2(-0.5, -0.866_025_4),
// ],
// vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],

// X^5 - 1
// vec![
// 	vec2(1.0, 0.0),
// 	vec2(-0.80902, -0.58779),
// 	vec2(0.30902, 0.95106),
// 	vec2(0.30902, -0.95106),
// 	vec2(-0.80902, 0.58779),
// ],
// vec![
// 	[1.0, 0.0, 0.0],
// 	[0.0, 1.0, 0.0],
// 	[0.0, 0.0, 1.0],
// 	[1.0, 1.0, 0.0],
// 	[0.0, 1.0, 1.0],
// ],

#[macroquad::main("Newton Fractal")]
async fn main() {
    let mut iter = 30;
    let mut add_root = false;

    let fractal = NewtonFractal::new(
        vec![
            vec2(1.0, 0.0),
            vec2(-0.5, 0.866_025_4),
            vec2(-0.5, -0.866_025_4),
        ],
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        iter,
        vec2(-1.0, 1.0),
        vec2(-1.0, 1.0),
    );

    if fractal.is_none() {
        panic!("Error creating fractal");
    }

    let mut fractal = fractal.unwrap();
    let mut drag_lock = false;
    let mut drag_index = -1;

    loop {
        if is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let real_range = fractal.get_real_range();
            let imag_range = fractal.get_imag_range();
            if add_root {
                add_root = false;
                let x = map(
                    mouse_position().0,
                    0.0,
                    screen_width(),
                    real_range.x,
                    real_range.y,
                );
                let y = map(
                    mouse_position().1,
                    0.0,
                    screen_height(),
                    imag_range.x,
                    imag_range.y,
                );
                fractal.add_root(
                    vec2(x, y),
                    [
                        rand::gen_range(0., 255.),
                        rand::gen_range(0., 255.),
                        rand::gen_range(0., 255.),
                    ],
                );
            } else if !drag_lock {
                for (i, root) in fractal.get_roots().iter().enumerate() {
                    let x = map(root.x, real_range.x, real_range.y, 0.0, screen_width());
                    let y = map(root.y, imag_range.x, imag_range.y, 0.0, screen_height());
                    if (mouse_position().0 - x).abs() < ROOT_RADIUS * 1.2
                        && (mouse_position().1 - y).abs() < ROOT_RADIUS * 1.2
                    {
                        drag_index = i as i32;
                        break;
                    }
                }
                drag_lock = true;
            } else if drag_index != -1 {
                let root = &mut fractal.get_roots()[drag_index as usize];
                root.x = map(
                    mouse_position().0,
                    0.0,
                    screen_width(),
                    real_range.x,
                    real_range.y,
                );
                root.y = map(
                    mouse_position().1,
                    0.0,
                    screen_height(),
                    imag_range.x,
                    imag_range.y,
                );
            }
        } else {
            drag_lock = false;
            drag_index = -1;
        }

        if is_key_down(macroquad::miniquad::KeyCode::Left) {
            // go left
            let real_range = fractal.get_real_range_mut();
            real_range.x -= 0.01;
            real_range.y -= 0.01;
        }
        if is_key_down(macroquad::miniquad::KeyCode::Right) {
            // go right
            let real_range = fractal.get_real_range_mut();
            real_range.x += 0.01;
            real_range.y += 0.01;
        }
        if is_key_down(macroquad::miniquad::KeyCode::Down) {
            // go down
            let imag_range = fractal.get_imag_range_mut();
            imag_range.x += 0.01;
            imag_range.y += 0.01;
        }
        if is_key_down(macroquad::miniquad::KeyCode::Up) {
            // go up
            let imag_range = fractal.get_imag_range_mut();
            imag_range.x -= 0.01;
            imag_range.y -= 0.01;
        }

        // gui
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Configuration").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Render").strong());
                });
                ui.label("Iterations");
                ui.add(egui::DragValue::new(&mut iter).speed(1.0));
                ui.separator();
                ui.label("Real Range");
                ui.horizontal(|ui| {
                    let range = fractal.get_real_range_mut();
                    ui.add(egui::DragValue::new(&mut range.x).speed(0.01));
                    ui.add(egui::DragValue::new(&mut range.y).speed(0.01));
                });
                ui.label("Imaginary Range");
                ui.horizontal(|ui| {
                    let range = fractal.get_imag_range_mut();
                    ui.add(egui::DragValue::new(&mut range.x).speed(0.01));
                    ui.add(egui::DragValue::new(&mut range.y).speed(0.01));
                });

                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Roots").strong());
                });
                if ui.button("Add Root").clicked() {
                    add_root = !add_root;
                }
                for i in 0..fractal.len() {
                    if i >= fractal.len() {
                        continue;
                    }
                    ui.label(format!("Root {}", i + 1));
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut fractal.get_roots()[i].x).speed(0.01));
                        ui.add(egui::DragValue::new(&mut fractal.get_roots()[i].y).speed(0.01));
                        ui.color_edit_button_rgb(&mut fractal.get_colors()[i]);
                        if ui.button("Remove").clicked() {
                            fractal.remove_root(i);
                        }
                    });
                }
            });
        });

        fractal.set_max_iterations(iter);
        fractal.update();

        gl_use_material(*fractal.get_material());
        draw_rectangle(0.0, 0., screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        draw_roots(&mut fractal);

        egui_macroquad::draw();

        next_frame().await
    }
}
