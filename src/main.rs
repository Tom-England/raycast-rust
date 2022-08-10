extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::{Image, rectangle};
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
pub mod player;
pub mod map;
pub mod sprite;
pub mod global;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("raycast", [global::X, global::Y])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut texture_atlas: Vec<[[image::Rgba<u8>; 256]; 256]> = Vec::new();
    let mut sprite_atlas: Vec<[[image::Rgba<u8>; 256]; 256]> = Vec::new();

    /// Converts a dynamic image into a 2d array of pixels (Slightly faster to access, not strictly important)
    fn new_texture(path: String) -> [[image::Rgba<u8>; 256]; 256] {
        let mut arr: [[image::Rgba<u8>; 256]; 256] = [[image::Rgba([0,0,0,0]); 256]; 256];
        let image = image::open(path).unwrap();
        for i in 0..256{
            for j in 0..256{
                let pixel = image.get_pixel(i, j);
                arr[i as usize][j as usize] = pixel;
            }
        }
        return arr;
    }

    // Load Textures
    texture_atlas.push(new_texture("assets/brick2.jpg".to_string()));
    texture_atlas.push(new_texture("assets/wood.jpg".to_string()));
    texture_atlas.push(new_texture("assets/metal.jpg".to_string()));

    // Load Sprites
    sprite_atlas.push(new_texture("assets/sprites/badguy.png".to_string()));
    sprite_atlas.push(new_texture("assets/sprites/skulls.png".to_string()));

    // Create a new game and run it.
    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        play: player::Player{
            plane: (0.0, 0.66),
            dir: (-1.0, 0.0),
            pos: (3.0, 5.0),
            rays: Vec::new(),
        },
        map: map::Map{
            map_dim: (10, 10),
            cell_arr: [
                [1,1,1,1,1,1,1,1,1,1],
                [1,1,0,1,0,0,1,0,1,1],
                [1,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,1],
                [1,1,0,1,0,0,1,0,1,1],
                [1,1,1,1,0,0,1,1,1,1],
                [1,0,0,0,0,0,0,2,2,1],
                [1,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,1],
                [1,1,1,3,3,3,3,1,1,1]
            ]
        },
        sprites: Vec::new(),
        texture_atlas: texture_atlas,
        sprite_atlas: sprite_atlas,
        sky: Texture::from_path(Path::new("assets/sky.png"), &TextureSettings::new()).unwrap(),
        turning_left: false,
        turning_right: false,
        moving_forward: false,
        moving_back: false,
        debug: false,
        last_time_step: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        dt: 0.0,
        map_image: Image::new().rect(rectangle::rectangle_by_corners(0.0, 0.0, global::X, global::Y)),
        sky_image: Image::new().rect(rectangle::rectangle_by_corners(0.0, 0.0, global::X, global::Y/2.0))
    };

    // Create some sprites
    app.sprites.push(sprite::Sprite{ pos: (8.0, 5.0), texture_index: 0, dist: 0.0});
    app.sprites.push(sprite::Sprite{ pos: (8.0, 4.0), texture_index: 1, dist: 0.0});
    app.sprites.push(sprite::Sprite{ pos: (8.0, 6.0), texture_index: 1, dist: 0.0});

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
