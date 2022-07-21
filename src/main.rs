extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f64::consts::PI;
use std::path::Path;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::{Button, PressEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use image::{GenericImageView, ImageBuffer, RgbaImage, DynamicImage};

pub mod ray;
pub mod wall;
pub mod player;
pub mod map;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    play: player::Player,
    map: map::Map,
    img: DynamicImage,
    sky: Texture,
    grass: Texture
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

            // Draw Skybox
            let sky_image: Image = Image::new().rect(rectangle::rectangle_by_corners(0.0, 0.0, args.window_size[0], args.window_size[1]));

            let grass_image: Image = Image::new().rect(rectangle::rectangle_by_corners(0.0, args.window_size[1] / 2.0, args.window_size[0], args.window_size[1]));

            let ds: DrawState = DrawState::default();
            sky_image.draw(&self.sky, &ds, c.transform, gl);
            grass_image.draw(&self.grass, &ds, c.transform, gl);

            // Draw the level
            let map_image: Image = Image::new().rect(rectangle::rectangle_by_corners(0.0, 0.0, args.window_size[0], args.window_size[1]));
            let map_img = App::create_texture(&self.play.rays, self.play.view_direction, &self.img, args.window_size[0], args.window_size[1]);
            let map_texture: Texture = Texture::from_image(&map_img, &TextureSettings::new());
            map_image.draw(&map_texture, &ds, c.transform, gl);

            let debug: bool = true;
            // Debug drawing
            if debug{
                for ray in &self.play.rays{
                    line(RED, 1.0, [ray.start.0, ray.start.1, ray.end.0, ray.end.1], c.transform, gl);
                }
                for cell in &self.map.cells{
                    for wall in &cell.walls{
                        line(GREEN, 1.0, [wall.start.0, wall.start.1, wall.end.0, wall.end.1], c.transform, gl);
                    }
                }
            }

        });
    }

    fn calculate_box_height(ray: &ray::Ray, view_direction: f64) -> f64{
        if ray.collided{
            let a = (ray.angle - view_direction) * PI / 180.0;
            let z = ray.length * a.cos();
            let max = 400.0;
            let h = max / z;

            //if h > max { h = max; }
            return h;
        }
        return 0.0;
    }

    fn get_pixel(x: f32, y: f32, img: &DynamicImage) -> image::Rgba<u8>{ 
        let mut x_pos = (img.dimensions().0 as f32 * x) as u32;
        let mut y_pos = (img.dimensions().1 as f32 * y) as u32;

        if x_pos >= img.dimensions().0 {x_pos = img.dimensions().0 - 1};
        if y_pos >= img.dimensions().1 {y_pos = img.dimensions().1 - 1};

        return img.get_pixel(x_pos, y_pos);
    }

    fn create_texture(rays: &Vec<ray::Ray>, view_direction: f64, tex: &DynamicImage, width: f64, height: f64) -> image::RgbaImage{
        let mut img: RgbaImage = ImageBuffer::new(width as u32, height as u32);
        let width = width / rays.len() as f64;
        // For each ray, draw a rectangle in the correct place in the image
        for i in 0..rays.len(){
            let view_dist = 1.0 - rays[i].length/10.0;
            let h: f64 = App::calculate_box_height(&rays[i], view_direction);
            let mut dh = h;
            if dh > height {dh = height;}
            let iter = i as f64;

            for x in (iter * width) as u32..(iter * width+width) as u32{
                for y in (height/2.0 - dh/2.0) as u32..(height/2.0 - dh/2.0 + dh) as u32 - 1{
                    let mut pixel_y = (y as f64 - (height/2.0 - h/2.0)) as f32 / h as f32;
                    if pixel_y >= 1.0 { pixel_y = 0.99; }
                    let mut pixel = App::get_pixel(rays[i].wall_pos, pixel_y, tex);
                    for i in 0..3{
                        let new_colour = pixel[i] as f64 * view_dist;
                        pixel[i] = new_colour as u8;
                    }
                    img.put_pixel(x, y, pixel)
                }
            }
        }
        return img;
    }

    fn update(&mut self) {
        for i in 0..self.play.rays.len(){
            self.play.rays[i].end = self.play.rays[i].calc_end();
            self.play.rays[i].length = self.play.rays[i].calc_length();
            self.play.rays[i].collided = false;
            for cell in &self.map.cells{
                for mut wall in cell.walls{
                    let new_end = self.play.rays[i].find_intersection(wall.start, wall.end);
                    if new_end != (-1.0, -1.0) && (ray::Ray::calc_length_of_ray(self.play.rays[i].start, new_end) < self.play.rays[i].length){
                        self.play.rays[i].end = self.play.rays[i].find_intersection(wall.start, wall.end);
                        self.play.rays[i].length = self.play.rays[i].calc_length();
                        self.play.rays[i].collided = true;
                        self.play.rays[i].wall_pos = wall.dist_along_wall(self.play.rays[i].end);
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
            self.play.advance(0.5);
        }

        if *args == Keyboard(Key::Down) {
            self.play.advance(-0.5);
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
            pos: (3.0, 3.0),
            rays: Vec::new(),
            fov: 80,
        },
        map: map::Map{
            cells: Vec::new()
        },
        img: image::open("assets/brick2.jpg").unwrap(),
        sky: Texture::from_path(Path::new("assets/sky.png"), &TextureSettings::new()).unwrap(),
        grass: Texture::from_path(Path::new("assets/grass.png"), &TextureSettings::new()).unwrap(),
    };

    // Add some rays
    app.play.gen_rays();

    // Add some walls

    let mut walls: Vec<map::Cell> = Vec::new();
    walls.push(map::Cell{
        walls: [wall::Wall::default(); 4],
        pos: (0.0, 0.0),
        l: 1.3
    });
    walls.push(map::Cell{
        walls: [wall::Wall::default(); 4],
        pos: (1.0, 0.0),
        l: 1.3
    });
    walls.push(map::Cell{
        walls: [wall::Wall::default(); 4],
        pos: (2.0, 0.0),
        l: 1.3
    });
    walls.push(map::Cell{
        walls: [wall::Wall::default(); 4],
        pos: (0.0, 1.0),
        l: 1.3
    });
    for mut cell in walls {
        cell.create_walls();
        app.map.cells.push(cell);
    }
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
    }
}
