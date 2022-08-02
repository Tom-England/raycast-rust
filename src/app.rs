use graphics::Image;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::{Button};

use piston::input::{RenderArgs};
use image::{ImageBuffer, RgbaImage};

use std::f64::consts::PI;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

use crate::player;
use crate::map;
use crate::ray;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub play: player::Player,
    pub map: map::Map,
    pub img: [[image::Rgba<u8>; 512]; 512],
    pub sky: Texture,
    pub grass: Texture,
    pub turning_left: bool,
    pub turning_right: bool,
    pub moving_forward: bool,
    pub moving_back: bool,
    pub debug: bool,
    pub last_time_step: Duration,
    pub dt: f64,
    pub map_image: Image,
    pub sky_image: Image,
    pub grass_image: Image
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            // Draw Skybox

            let ds: DrawState = DrawState::default();
            self.sky_image.draw(&self.sky, &ds, c.transform, gl);
            self.grass_image.draw(&self.grass, &ds, c.transform, gl);

            
            // Draw the level
            let t1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let map_img = App::create_texture(&self.play.rays, self.play.view_direction, &self.img, args.window_size[0], args.window_size[1]);
            let map_texture: Texture = Texture::from_image(&map_img, &TextureSettings::new());
            self.map_image.draw(&map_texture, &ds, c.transform, gl);
            let t2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            println!("Drawing frame took {0} seconds", (t2 - t1).as_secs_f64());
            // Debug drawing
            if self.debug{        
                //println!("Total frame took {0} seconds", self.dt);
                println!("fps: {0}", 1.0/self.dt);
                for ray in &self.play.rays{
                    line(RED, 1.0, [ray.start.0 * 10.0, ray.start.1 * 10.0, ray.end.0 * 10.0, ray.end.1 * 10.0], c.transform, gl);
                }
                for cell in &self.map.cells{
                    for wall in &cell.walls{
                        line(GREEN, 1.0, [wall.start.0 * 10.0, wall.start.1 * 10.0, wall.end.0 * 10.0, wall.end.1 * 10.0], c.transform, gl);
                    }
                }
                
            }

        });
    }

    fn calculate_box_height(ray: &ray::Ray, view_direction: f64) -> f64{
        if ray.collided{
            let a = (ray.angle - view_direction) * PI / 180.0;
            let z = ray.length * a.cos();
            let max = 280.0;
            let h = max / z;

            //if h > max { h = max; }
            return h;
        }
        return 0.0;
    }

    fn get_pixel(x: f32, y: f32, img: &[[image::Rgba<u8>; 512]; 512]) -> image::Rgba<u8>{ 
        let x_pos = (img.len() as f32 * x) as u32;
        let y_pos = (img[0].len() as f32 * y) as u32;

        return img[x_pos as usize][y_pos as usize];
    }

    fn create_texture(rays: &Vec<ray::Ray>, view_direction: f64, tex: &[[image::Rgba<u8>; 512]; 512], width: f64, height: f64) -> image::RgbaImage{
        
        let mut img: RgbaImage = ImageBuffer::new(width as u32, height as u32);
        let width = width / rays.len() as f64;
        // For each ray, draw a rectangle in the correct place in the image
        for i in 0..rays.len(){
            let view_dist = 1.0 - rays[i].length/rays[i].max_length;
            let h: f64 = App::calculate_box_height(&rays[i], view_direction);
            let mut dh = h;
            if dh > height {dh = height;}
            let iter = i as f64;
            

            for x in (iter * width) as u32..(iter * width+width) as u32{
                for y in (height/2.0 - dh/2.0) as u32..(height/2.0 - dh/2.0 + dh) as u32 - 1{
                    let pixel_y = (y as f64 - (height/2.0 - h/2.0)) as f32 / h as f32;
                    //if pixel_y >= 1.0 { pixel_y = 0.99; }
                    let mut pixel = App::get_pixel(rays[i].wall_pos, pixel_y, tex);
                    for i in 0..3{
                        let new_colour = pixel[i] as f64 * view_dist;
                        pixel[i] = new_colour as u8;
                    }
                    img.put_pixel(x, y, pixel);
                    
                }
            }
            
        }
        
        return img;
    }

    pub fn update(&mut self) {

        self.dt = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - self.last_time_step).as_secs_f64();
        self.last_time_step = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        if self.turning_left {
            self.play.turn(-80.0, self.dt);
        }
        else if self.turning_right {
            self.play.turn(80.0, self.dt);
        }
        if self.moving_forward {
            self.play.advance(4.0, self.dt);
        }
        else if self.moving_back {
            self.play.advance(-4.0, self.dt);
        }

        for i in 0..self.play.rays.len(){
            self.play.rays[i].end = self.play.rays[i].calc_end();
            self.play.rays[i].length = self.play.rays[i].calc_length();
            self.play.rays[i].collided = false;
            for cell in &self.map.cells{
                for mut wall in cell.walls{
                    let new_end = self.play.rays[i].find_intersection(wall.start, wall.end);
                    if new_end != (-1.0, -1.0) && (ray::Ray::calc_length_of_ray(self.play.rays[i].start, new_end) < self.play.rays[i].length){
                        self.play.rays[i].end = new_end;
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
            self.turning_left = true;
            self.turning_right = false;
        }

        if *args == Keyboard(Key::Right) {
            self.turning_left = false;
            self.turning_right = true;
        }

        if *args == Keyboard(Key::Up) {
            self.moving_forward = true;
            self.moving_back = false;
        }

        if *args == Keyboard(Key::Down) {
            self.moving_forward = false;
            self.moving_back = true;
        }
    }
    pub fn key_up(&mut self, args: &Button) {
		use piston::Button::Keyboard;
		use piston::Key;		
		
        if *args == Keyboard(Key::Left) {
            self.turning_left = false;
        }

        if *args == Keyboard(Key::Right) {
            self.turning_right = false;
        }

        if *args == Keyboard(Key::Up) {
            self.moving_forward = false;
        }

        if *args == Keyboard(Key::Down) {
            self.moving_back = false;
        }
    }
}