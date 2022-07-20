extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f64::consts::PI;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Button, PressEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub mod ray;
pub mod wall;
pub mod player;
pub mod map;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    play: player::Player,
    map: map::Map
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
            
            rectangle([0.0,0.0,1.0,1.0], rectangle::rectangle_by_corners(0.0, 0.0, 500.0, 200.0), c.transform, gl);
            rectangle([0.0,1.0,0.0,1.0], rectangle::rectangle_by_corners(0.0, 201.0, 500.0, 400.0), c.transform, gl);


            let width = 500.0 / self.play.rays.len() as f64;
            for i in 0..self.play.rays.len(){
                if self.play.rays[i].collided{
                    let view_dist = 1.0 - self.play.rays[i].length/200.0;
                    let col: f32 = 1.0 * view_dist as f32;
                    let a = (self.play.rays[i].angle - self.play.view_direction) * PI / 180.0;
                    let z = self.play.rays[i].length * a.cos();
                    let max = 400.0 * 20.0;
                    let mut h = max / z;

                    if h > max { h = max; }
                    let colour = [col, col, col, 1.0];
                    let rec = rectangle::rectangle_by_corners(i as f64 * width, 200.0 - h/2.0, i as f64 * width + width, 200.0 - h/2.0 + h);
                    rectangle(colour, rec, c.transform, gl);
                }
            }

            for ray in &self.play.rays{
                line(RED, 1.0, [ray.start.0, ray.start.1, ray.end.0, ray.end.1], c.transform, gl);
            }/*
            for wall in &self.walls{
                line(GREEN, 1.0, [wall.start.0, wall.start.1, wall.end.0, wall.end.1], c.transform, gl);
            }*/

            for cell in &self.map.cells{
                for wall in &cell.walls{
                    line(GREEN, 1.0, [wall.start.0, wall.start.1, wall.end.0, wall.end.1], c.transform, gl);
                }
            }
        });
    }

    fn update(&mut self) {
        for i in 0..self.play.rays.len(){
            self.play.rays[i].end = self.play.rays[i].calc_end();
            self.play.rays[i].length = self.play.rays[i].calc_length();
            self.play.rays[i].collided = false;
            for cell in &self.map.cells{
                for wall in cell.walls{
                    let new_end = self.play.rays[i].find_intersection(wall.start, wall.end);
                    if new_end != (-1.0, -1.0) && (ray::Ray::calc_length_of_ray(self.play.rays[i].start, new_end) < self.play.rays[i].length){
                        self.play.rays[i].end = self.play.rays[i].find_intersection(wall.start, wall.end);
                        self.play.rays[i].length = self.play.rays[i].calc_length();
                        self.play.rays[i].collided = true;
                    }
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
        map: map::Map{
            cells: Vec::new()
        }
    };

    // Add some rays
    app.play.gen_rays();

    // Add some walls

    let mut cell1: map::Cell = map::Cell{
        walls: [wall::Wall::default(); 4],
        pos: (0.0, 0.0),
        l: 40.0
    };
    let mut cell2: map::Cell = map::Cell{
        walls: [wall::Wall::default(); 4],
        pos: (1.0, 0.0),
        l: 40.0
    };
    cell1.create_walls();
    cell2.create_walls();
    app.map.cells.push(cell1);
    app.map.cells.push(cell2);
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
