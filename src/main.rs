extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{OpenGL, GlGraphics, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::{PressEvent, ReleaseEvent};

use std::path::Path;

pub mod app;
pub mod ray;
pub mod wall;
pub mod player;
pub mod map;

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
    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        play: player::Player{
            view_direction: 0.0,
            pos: (3.0, 5.0),
            rays: Vec::new(),
            fov: 80
        },
        map: map::Map{
            cells: Vec::new()
        },
        img: image::open("assets/brick2.jpg").unwrap(),
        sky: Texture::from_path(Path::new("assets/sky.png"), &TextureSettings::new()).unwrap(),
        grass: Texture::from_path(Path::new("assets/grass.png"), &TextureSettings::new()).unwrap(),
        turning_left: false,
        turning_right: false,
        moving_forward: false,
        moving_back: false
    };

    // Add some rays
    app.play.gen_rays();

    // Add some walls
    app.map.make_map();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }

        if let Some(ref args) = e.release_args() {
            app.key_up(args);
        }
    }
}
