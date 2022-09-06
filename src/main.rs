use macroquad::ui::{hash, root_ui, widgets};

use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    
    let mut x = String::from("Hello, world!");

    

    loop {
        clear_background(WHITE);

        root_ui().window(hash!(), Vec2::new(200., 200.), Vec2::new(450., 200.), |ui| {
            let (mouse_x, mouse_y) = mouse_position();
            ui.label(None, &format!("Mouse position: {} {}", mouse_x, mouse_y));

            ui.input_text(hash!(), "give me some data", &mut x);

            if is_key_down(KeyCode::W) {
                draw_circle(500., 500., 100., BLUE);
            }
            if is_key_down(KeyCode::A) {
                draw_circle(300., 700., 100., RED);
            }
            if is_key_down(KeyCode::S) {
                draw_circle(500., 700., 100., YELLOW);
            }
            if is_key_down(KeyCode::D) {
                draw_circle(700., 700., 100., GREEN);
            }


            let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
            ui.label(None, &format!("Mouse wheel x: {}", mouse_wheel_x));
            ui.label(None, &format!("Mouse wheel y: {}", mouse_wheel_y));

            widgets::Group::new(hash!(), Vec2::new(200., 90.))
                .position(Vec2::new(240., 0.))
                .ui(ui, |ui| {
                    ui.label(None, "Pressed kbd keys");

 
                });

            widgets::Group::new(hash!(), Vec2::new(200., 90.))
                .position(Vec2::new(240., 92.))
                .ui(ui, |ui| {
                    ui.label(None, "Pressed mouse keys");

                    if is_mouse_button_down(MouseButton::Left) {
                        ui.label(None, "Left");
                    }
                    if is_mouse_button_down(MouseButton::Right) {
                        ui.label(None, "Right");
                    }
                    if is_mouse_button_down(MouseButton::Middle) {
                        ui.label(None, "Middle");
                    }
                });
        });

        
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .show(egui_ctx, |ui| {
                    if ui.button("Test").clicked() {

                    };
                });
        });

        // Draw things before egui

        egui_macroquad::draw();
        


        next_frame().await;
    }
}
