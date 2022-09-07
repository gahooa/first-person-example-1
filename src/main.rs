
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};


struct Point{
    x: f32,
    y: f32,
    r: f32,
    vx: f32,
    vy: f32,
    vr: f32,
    color: Color,
}

impl Point{
    fn new(x: f32, y: f32, r: f32, vx: f32, vy: f32, vr: f32, color: Color) -> Self{
        Self{
            x,
            y,
            r,
            vx: vx,
            vy: vy,
            vr: vr,
            color: color,
        }
    }

    fn update(&mut self){
        self.x += self.vx;
        self.y += self.vy;
        self.r += self.vr;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Triangles"),
        window_width: 1000,
        window_height: 700,
        fullscreen: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    
    let mut rng = thread_rng();

    let mut points:Vec<Point> = vec![];
    
    let mut fps = 0;
    let mut increment:i64 = 0;

    loop {
        if increment % 30 == 0 {
            fps = get_fps();

        }
        increment += 1;

        if points.len() > 0{
            points.remove(0);
        }

        for point in points.iter_mut(){
            point.update();
        }
        

        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            for _ in 0..100 {
                let vx:f32 = rng.gen_range(-1.0..1.0);
                let vy:f32 = rng.gen_range(-1.0..1.0);
                let vr:f32 = rng.gen_range(-10.0..10.0);
                points.push(Point::new(mouse_x, mouse_y, 0.0, vx, vy, vr, YELLOW ));
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            for _ in 0..100 {
                let vx:f32 = rng.gen_range(-1.0..1.0);
                let vy:f32 = rng.gen_range(-1.0..1.0);
                let vr:f32 = rng.gen_range(-1.0..1.0);
                points.push(Point::new(mouse_x, mouse_y, 0.0, vx, vy, vr, ORANGE ));
            }
        }


        clear_background(BLACK);

        for point in points.iter(){
            draw_poly_lines(point.x, point.y, 3, 7.0, point.r, 1.0, point.color);
        }

        draw_rectangle(0.0,0.0, screen_width(), 60.0, GRAY);
        draw_text(format!("There are {} objects, running at {} fps", points.len(), fps).as_str(), 20.0, 50.0, 50.0, WHITE);

        next_frame().await;
    }
}
