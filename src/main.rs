extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use image::GenericImageView;
use piston::window::WindowSettings;
use opengl_graphics::{OpenGL, GlGraphics, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::{PressEvent, ReleaseEvent};

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

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

    // Load brick texture and convert to an array
    let brick_dyn_image = image::open("assets/brick2.jpg").unwrap();
    let mut brick_arr: [[image::Rgba<u8>; 512]; 512] = [[image::Rgba([0,0,0,0]); 512]; 512];
    for i in 0..512{
        for j in 0..512{
            let pixel = brick_dyn_image.get_pixel(i, j);
            brick_arr[i as usize][j as usize] = pixel;
        }
    }


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
        img: brick_arr,
        sky: Texture::from_path(Path::new("assets/sky.png"), &TextureSettings::new()).unwrap(),
        grass: Texture::from_path(Path::new("assets/grass.png"), &TextureSettings::new()).unwrap(),
        turning_left: false,
        turning_right: false,
        moving_forward: false,
        moving_back: false,
        debug: true,
        last_time_step: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        dt: 0.0
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

        if let Some(_args) = e.update_args() {
            app.update();
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }

        if let Some(ref args) = e.release_args() {
            app.key_up(args);
        }
    }
}
