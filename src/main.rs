use macroquad::{
    color::{BLACK, WHITE},
    input::{is_mouse_button_down, mouse_position},
    material::{gl_use_default_material, gl_use_material},
    math::vec2,
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
    for root in fractal.get_roots() {
        let x = map(root.x, -1.0, 1.0, 0.0, screen_width());
        let y = map(root.y, -1.0, 1.0, 0.0, screen_height());
        if (pos.0 - x).abs() < ROOT_RADIUS * 1.2 && (pos.1 - y).abs() < ROOT_RADIUS * 1.2 {
            draw_circle(x, y, ROOT_RADIUS, WHITE);
        } else {
            draw_circle(x, y, ROOT_RADIUS, BLACK);
        }
    }
}

#[macroquad::main("Newton Fractal")]
async fn main() {
    let mut iter = 30;
    let fractal = NewtonFractal::new(
        vec![
            vec2(1.0, 0.0),
            vec2(-0.5, 0.866_025_4),
            vec2(-0.5, -0.866_025_4),
        ],
        vec![
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ],
        iter,
    );

    if fractal.is_none() {
        panic!("Error creating fractal");
    }

    let mut fractal = fractal.unwrap();
	let mut drag_lock = false;
	let mut drag_index = -1;

    loop {
        gl_use_material(*fractal.get_material());
        draw_rectangle(0.0, 0., screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        draw_roots(&mut fractal);
		if is_mouse_button_down(macroquad::input::MouseButton::Left) {
			if !drag_lock {
				for (i, root) in fractal.get_roots().iter().enumerate() {
					let x = map(root.x, -1.0, 1.0, 0.0, screen_width());
					let y = map(root.y, -1.0, 1.0, 0.0, screen_height());
					if (mouse_position().0 - x).abs() < ROOT_RADIUS * 1.2 && (mouse_position().1 - y).abs() < ROOT_RADIUS * 1.2 {
						drag_index = i as i32;
						break;
					}
				}
				drag_lock = true;
			} else if drag_index != -1 {
				let root = &mut fractal.get_roots()[drag_index as usize];
				root.x = map(mouse_position().0, 0.0, screen_width(), -1.0, 1.0);
				root.y = map(mouse_position().1, 0.0, screen_height(), -1.0, 1.0);
			}
		} else {
			drag_lock = false;
			drag_index = -1;
		}
        // gui
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Configuration").show(ctx, |ui| {
                ui.label("Iterations");
                ui.add(egui::DragValue::new(&mut iter).speed(1.0));
                ui.label("Roots");
				let root_and_colors = fractal.get_colored_roots();
				for (i, (root, color)) in root_and_colors.enumerate() {
					ui.label(format!("Root {}", i + 1));
					ui.horizontal(|ui| {
						ui.add(egui::DragValue::new(&mut root.x).speed(0.01));
						ui.add(egui::DragValue::new(&mut root.y).speed(0.01));
						ui.color_edit_button_rgb(color);
					});
				}
            });
        });

        egui_macroquad::draw();

        fractal.set_max_iterations(iter);
        fractal.update();

        next_frame().await
    }
}
