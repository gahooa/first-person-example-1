
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
        window_width: 2000,
        window_height: 1200,
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
        increment += 1;
        if increment % 30 == 0 {
            fps = get_fps();
        }

        let (mouse_x, mouse_y) = mouse_position();
        
        let gravity = is_mouse_button_down(MouseButton::Right);
        let g = vec2(mouse_x, mouse_y);
        
        if is_mouse_button_down(MouseButton::Left) {
            for _ in 0..10 {
                let vx:f32 = rng.gen_range(-1.0..1.0);
                let vy:f32 = rng.gen_range(-1.0..1.0);
                let vr:f32 = rng.gen_range(-1.0..1.0);
                points.push(Point::new(mouse_x, mouse_y, 0.0, vx, vy, vr, YELLOW ));
            }
        }

        let sw = screen_width();
        let sh = screen_height();

        points.retain(|p| p.x > 0.0 && p.x < sw && p.y > 0.0 && p.y < sh);

        for point in points.iter_mut(){
            if gravity {
                let p = vec2(point.x, point.y);
                let v = g - p;
                let q = (1.0 / (p.distance(g).powf(1.5).max(-1.0))) * 10.0;

                point.vx += v.x * q;
                point.vy += v.y * q;
            }

            point.vx *= 0.99;
            point.vy *= 0.99;
            point.update();
        }
        



        clear_background(BLACK);

        if gravity {
            draw_circle(mouse_x, mouse_y, 10.0, BLUE);
        }

        for point in points.iter(){
            draw_poly_lines(point.x, point.y, 3, 7.0, point.r, 1.0, point.color);
        }

        draw_rectangle(0.0,0.0, screen_width(), 60.0, GRAY);
        draw_text(format!("There are {} objects, running at {} fps", points.len(), fps).as_str(), 20.0, 50.0, 50.0, WHITE);

        

        next_frame().await;
    }
}
