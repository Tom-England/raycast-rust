extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub mod ray;
pub mod wall;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rays: Vec<ray::Ray>,
    walls: Vec<wall::Wall>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);

            //print!("Drawing line from: {0},{1} to {2},{3}", self.test_ray.start.0, self.test_ray.start.1, self.test_ray.end.0, self.test_ray.end.1);
            for ray in &self.rays{
                line(RED, 1.0, [ray.start.0, ray.start.1, ray.end.0, ray.end.1], c.transform, gl);
            }
            for wall in &self.walls{
                line(GREEN, 1.0, [wall.start.0, wall.start.1, wall.end.0, wall.end.1], c.transform, gl);
            }
        });
    }

    fn update(&mut self) {
        for i in 0..self.rays.len(){
            self.rays[i].end = self.rays[i].calc_end();
            for j in 0..self.walls.len(){
                let new_end = self.rays[i].find_intersection(self.walls[j].start, self.walls[j].end);
                if new_end != (-1.0, -1.0){
                    self.rays[i].end = self.rays[i].find_intersection(self.walls[j].start, self.walls[j].end);
                }
            }
            
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("raycast", [500, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rays: Vec::new(), 
        walls: Vec::new()
    };

    // Add some rays

    const FOV:i16 = 80;
    for i in 0..FOV{
        app.rays.push(ray::Ray {
            start: (10.0, 10.0),
            angle: i as f64,
            max_length: 200.0,
            end: (10.0, 10.0),
            length: 0.0,
        },);
    }

    

    // Add some walls
    app.walls.push(wall::Wall {start: (50.0, 10.0), end: (50.0, 100.0)});

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.update();
            app.render(&args);
        }

        /*if let Some(args) = e.update_args() {
            app.update(&args);
        }*/
    }
}
