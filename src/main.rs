extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, PressEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub mod ray;
pub mod wall;
pub mod player;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    play: player::Player,
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
            for ray in &self.play.rays{
                line(RED, 1.0, [ray.start.0, ray.start.1, ray.end.0, ray.end.1], c.transform, gl);
            }
            for wall in &self.walls{
                line(GREEN, 1.0, [wall.start.0, wall.start.1, wall.end.0, wall.end.1], c.transform, gl);
            }
        });
    }

    fn update(&mut self) {
        for i in 0..self.play.rays.len(){
            self.play.rays[i].end = self.play.rays[i].calc_end();
            self.play.rays[i].length = self.play.rays[i].calc_length();
            for j in 0..self.walls.len(){
                let new_end = self.play.rays[i].find_intersection(self.walls[j].start, self.walls[j].end);
                if new_end != (-1.0, -1.0) && (ray::Ray::calc_length_of_ray(self.play.rays[i].start, new_end) < self.play.rays[i].length){
                    self.play.rays[i].end = self.play.rays[i].find_intersection(self.walls[j].start, self.walls[j].end);
                    self.play.rays[i].length = self.play.rays[i].calc_length();
                }
            }
            
        }
    }

    pub fn key_press(&mut self, args: &Button) {
		use piston::Button::Keyboard;
		use piston::Key;		
		
        if *args == Keyboard(Key::Left) {
            self.play.turn(-2.0);
        }

        if *args == Keyboard(Key::Right) {
            self.play.turn(2.0);
        }

        if *args == Keyboard(Key::Up) {
            self.play.advance(2.0);
        }

        if *args == Keyboard(Key::Down) {
            self.play.advance(-2.0);
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
        play: player::Player{
            view_direction: 0.0,
            pos: (10.0, 10.0),
            rays: Vec::new(),
            fov: 80
        },
        walls: Vec::new()
    };

    // Add some rays
    app.play.gen_rays();


    

    // Add some walls
    app.walls.push(wall::Wall {start: (50.0, 9.0), end: (50.0, 100.0)});
    app.walls.push(wall::Wall {start: (70.0, 9.0), end: (70.0, 100.0)});
    app.walls.push(wall::Wall {start: (10.0, 70.0), end: (100.0, 100.0)});

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update();
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }
    }
}
