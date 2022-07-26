use graphics::Image;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use piston::input::{RenderArgs};
use image::{ImageBuffer, RgbaImage};

use std::time::{SystemTime, Duration, UNIX_EPOCH};

use crate::{player, global};
use crate::map;
use crate::ray;
use crate::sprite;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub play: player::Player,
    pub map: map::Map,
    pub sprites: Vec<sprite::Sprite>,
    pub texture_atlas: Vec<[[image::Rgba<u8>; 256]; 256]>,
    pub sprite_atlas: Vec<[[image::Rgba<u8>; 256]; 256]>,
    pub sky: Texture,
    pub debug: bool,
    pub last_time_step: Duration,
    pub dt: f64,
    pub map_image: Image,
    pub sky_image: Image,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREY: [f32; 4] = [0.2,0.2,0.2, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        
        // Create the world texture and draw the sprites
        let mut map_img = App::create_texture(&self.play.rays, &self.texture_atlas, args.window_size[0], args.window_size[1]);
        self.draw_sprites(&mut map_img);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREY, gl);
            
            // Draw Skybox
            let ds: DrawState = DrawState::default();
            self.sky_image.draw(&self.sky, &ds, c.transform, gl);

            // Draw the level
            let map_texture: Texture = Texture::from_image(&map_img, &TextureSettings::new());
            self.map_image.draw(&map_texture, &ds, c.transform, gl);

            // DO NOT DELETE - SCENE DOES NOT DRAW WITHOUT THIS LINE FOR SOME REASON????
            line(GREEN, 1.0, [0.0, 0.0, 0.0, 0.0], c.transform, gl);

            // Debug
            if self.debug{       
                print!("\rfps: {:05.0}", (1.0/self.dt).floor());
            }

        });
    }

    /// Gets the nearest pixel from a texture for the given co-ordinate
    fn get_pixel(x: i32, y: f64, img: &[[image::Rgba<u8>; 256]; 256]) -> image::Rgba<u8>{ 
        //let x_pos = (img.len() as f64 * x) as usize;
        let y_pos = (img[0].len() as f64 * y) as usize;

        return img[x as usize][y_pos];
    }

    /// Uses the length of the provided rays to draw the world as a series of textured rectangles
    fn create_texture(rays: &Vec<ray::Ray>, tex: &Vec<[[image::Rgba<u8>; 256]; 256]>, width: f64, height: f64) -> image::RgbaImage{
        let mut img: RgbaImage = ImageBuffer::new(width as u32, height as u32);
        // Calculate the width of each ray (for best results, ensure that the raycount is a factor of the screen width)
        let width = width / rays.len() as f64;
        let max_len = 10.0;
        for i in 0..rays.len(){
            let mut shadow: bool = false;
            if rays[i].side == 0 || rays[i].side == 2 { shadow = true; }
            
            // Calculate how far between the player and the max render distance the intersected wall is
            let view_dist = 1.0 - rays[i].length/max_len;

            // Calculate the height of the wall segment
            let h: f64 = global::Y / rays[i].length;
            let mut dh = h;
            if dh > height {dh = height;}
            let iter = i as f64;
            
            // Wall drawing loop
            for x in (iter * width) as u32..(iter * width+width) as u32{
                for y in (height/2.0 - dh/2.0) as u32..(height/2.0 - dh/2.0 + dh) as u32 - 1{
                    // Get the correct pixel colour and shade it based off the view distance
                    let pixel_y = (y as f64 - (height/2.0 - h/2.0)) / h;
                    let index: usize = (rays[i].texture_index - 1) as usize;
                    let mut pixel = App::get_pixel(rays[i].texture_pos, pixel_y, &tex[index]);
                    for i in 0..3{
                        let mut new_colour = pixel[i] as f64 * view_dist;
                        if shadow {
                            new_colour = new_colour / 2.0;
                            
                        }
                        
                        pixel[i] = new_colour as u8;
                    }
                    // Draw the pixel to the image
                    img.put_pixel(x, y, pixel);
                    
                }
            }
        }
        return img;
    }

    fn sample_depth_buffer(depth_buffer: &Vec<ray::Ray>, pos: i32, screen_width: i32) -> f64 {
        let ratio = pos as f64 /screen_width as f64;
        let index = (depth_buffer.len() - 1) as f64 * ratio;
        return depth_buffer[index as usize].length;
    }

    /// Method for overlaying the games sprites over the pre-drawn environment
    fn draw_sprites(&mut self, tex: &mut image::RgbaImage) {
        let depth_buffer = &self.play.rays;
        // Update distances from player
        for i in 0..self.sprites.len(){
            self.sprites[i].dist = self.sprites[i].eucl_dist(self.play.pos);
        }
        // Sort the sprites by their distance from the player
        self.sprites.sort_by(|a, b| b.dist.partial_cmp(&a.dist).unwrap());

        let (pos_x, pos_y) = self.play.pos;
        let (plane_x, plane_y): (f64, f64) = self.play.plane;
        let (dir_x, dir_y): (f64, f64) = self.play.dir;

        // Draw the sprites
        for i in 0..self.sprites.len(){
            //translate sprite position to relative to camera
            let sprite_x: f64 = self.sprites[i].pos.0 - pos_x;
            let sprite_y: f64 = self.sprites[i].pos.1 - pos_y;

            let inv_det: f64 = 1.0 / (plane_x * dir_y - dir_x * plane_y); //required for correct matrix multiplication

            let transform_x: f64 = inv_det * (dir_y * sprite_x - dir_x * sprite_y);
            let transform_y: f64 = inv_det * (-plane_y * sprite_x + plane_x * sprite_y); //this is actually the depth inside the screen, that what Z is in 3D

            let sprite_screen_x: i32 = ((global::X / 2.0) * (1.0 + transform_x / transform_y)) as i32;

            //calculate height of the sprite on screen
            let sprite_height: i32 = ((global::Y / transform_y) as i32).abs(); //using 'transformY' instead of the real distance prevents fisheye
            //calculate lowest and highest pixel to fill in current stripe
            let mut draw_start_y: i32 = -sprite_height / 2 + global::Y as i32 / 2;
            if draw_start_y < 0 { draw_start_y = 0; }
            let mut draw_end_y: i32 = sprite_height / 2 + global::Y as i32 / 2;
            if draw_end_y >= global::Y as i32 { draw_end_y = global::Y as i32 - 1; }

            //calculate width of the sprite
            let sprite_width = ((global::Y / transform_y) as i32).abs();
            let mut draw_start_x: i32 = -sprite_width / 2 + sprite_screen_x;
            if draw_start_x < 0 { draw_start_x = 0; }
            let mut draw_end_x: i32 = sprite_width / 2 + sprite_screen_x;
            if draw_end_x >= global::X as i32 { draw_end_x = global::X as i32 - 1; }

            //loop through every vertical stripe of the sprite on screen
            for stripe in draw_start_x..draw_end_x
            {
                let tex_x = 256 * (stripe - (-sprite_width / 2 + sprite_screen_x)) * 256 / sprite_width / 256;
                //the conditions in the if are:
                //1) it's in front of camera plane so you don't see things behind you
                //2) it's on the screen (left)
                //3) it's on the screen (right)
                //4) ZBuffer, with perpendicular distance
                if transform_y > 0.0 && stripe > 0 && stripe < global::X as i32 && transform_y < App::sample_depth_buffer(depth_buffer, stripe, global::X as i32) {
                    for y in draw_start_y..draw_end_y //for every pixel of the current stripe
                    {
                        let d: i32 = (y) * 256 - global::Y as i32 * 128 + sprite_height * 128; //256 and 128 factors to avoid floats
                        let tex_y: i32 = ((d * 256) / sprite_height) / 256;
                        if tex_y < 256 && tex_x < 256 && tex_y >= 0 && tex_x >= 0{
                            let mut pixel = (self.sprite_atlas[self.sprites[i].texture_index as usize])[tex_x as usize][tex_y as usize];
                            if pixel != image::Rgba([0,0,0,0]) { 
                                for i in 0..3{
                                    let view_dist = 1.0 - App::sample_depth_buffer(depth_buffer, stripe, global::X as i32)/10.0;
                                    let new_colour = pixel[i] as f64 * view_dist;
                                    pixel[i] = new_colour as u8;
                                }
                                if pixel[3] < 255 {
                                    let bg = tex.get_pixel(stripe as u32, y as u32);
                                    let fga = pixel[3] as f64 / 255.0;
                                    for i in 0..3{
                                        //fg.R * fg.A / r.A + bg.R * bg.A * (1 - fg.A) / r.A;
                                        pixel[i] = (pixel[i] as f64 * fga + bg[i] as f64 * 1.0 * (1.0 - fga)) as u8;
                                    }
                                    if bg[3] == 255 {pixel[3] = 255;}
                                }
                                tex.put_pixel(stripe as u32, y as u32, pixel); 
                            }
                        }
                        
                    }
                }
                
            }
        } 
    }

    /// Method for handling updates in the game such as moving the player and updating the raycasts
    pub fn update(&mut self) {

        self.dt = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - self.last_time_step).as_secs_f64();
        self.last_time_step = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        self.play.update(self.dt, &self.map.cell_arr);
        self.find_ray_intersections();
    }

    /// Calculates the ray intersections and updates the Z-Buffer through the players ray vector
    fn find_ray_intersections(&mut self){
        let rc: i32 = 600;
        let (plane_x, plane_y): (f64, f64) = self.play.plane;
        let (dir_x, dir_y): (f64, f64) = self.play.dir;
        let (pos_x, pos_y) = self.play.pos;

        self.play.rays.clear();

        for r in 0..rc{
            //calculate ray position and direction
            let camera_x: f64 = 2.0 * r as f64 / rc as f64 - 1.0; //x-coordinate in camera space
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;
            let (mut map_x, mut map_y): (i32, i32) = (self.play.pos.0 as i32, self.play.pos.1 as i32);
            let (mut side_dist_x, mut side_dist_y): (f64, f64);
            //length of ray from one x or y-side to next x or y-side
            let (delta_dist_x, delta_dist_y): (f64, f64);
            if ray_dir_x == 0.0 { delta_dist_x = 10000.0; } else { delta_dist_x = (1.0 / ray_dir_x).abs(); }
            if ray_dir_y == 0.0 { delta_dist_y = 10000.0; } else { delta_dist_y = (1.0 / ray_dir_y).abs(); }
            let perp_wall_dist: f64;

            //what direction to step in x or y-direction (either +1 or -1)
            let step_x: i32;
            let step_y: i32;

            let mut hit: i32 = 0; //was there a wall hit?
            let mut side: i32 = 0; //was a NS or a EW wall hit?

            //calculate step and initial sideDist
            if ray_dir_x < 0.0
            {
                step_x = -1;
                side_dist_x = (pos_x - map_x as f64) * delta_dist_x;
            }
            else
            {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - pos_x) * delta_dist_x;
            }
            if ray_dir_y < 0.0
            {
                step_y = -1;
                side_dist_y = (pos_y - map_y as f64) * delta_dist_y;
            }
            else
            {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - pos_y) * delta_dist_y;
            }

            let mut ray = ray::Ray{
                length: 0.0,
                texture_index: 0,
                texture_pos: 0,
                side: 0
            };

            //perform DDA
            while hit == 0
            {
                //jump to next map square, either in x-direction, or in y-direction
                if side_dist_x < side_dist_y
                {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    if ray_dir_x < 0.0{
                        side = 0;
                    }
                    else {
                        side = 1;
                    }
                }
                else
                {
                    side_dist_y += delta_dist_y;
                    //println!("{0} + {1}", map_y, step_y);
                    map_y += step_y;
                    if ray_dir_y < 0.0{
                        side = 2;
                    }
                    else {
                        side = 3;
                    }
                }
                //Check if ray has hit a wall
                let ti: u8;
                if map_x >= 0 && map_x < self.map.map_dim.0 && map_y >= 0 && map_y < self.map.map_dim.1 {
                    //println!("{0}, {1}", map_x, map_y);
                    ti = self.map.cell_arr[map_x as usize][map_y as usize];
                    if ti > 0 { hit = 1; ray.texture_index = ti; }
                }
            }
            //Calculate distance projected on camera direction (Euclidean distance would give fisheye effect!)
            if side == 0 || side == 1{ perp_wall_dist = side_dist_x - delta_dist_x; }
            else { perp_wall_dist = side_dist_y - delta_dist_y; }

            //texturing calculations
            //calculate value of wallX
            let mut wall_x: f64; //where exactly the wall was hit
            if side == 0 || side == 1 { wall_x = pos_y + perp_wall_dist * ray_dir_y; }
            else { wall_x = pos_x + perp_wall_dist * ray_dir_x; }
            wall_x -= wall_x.floor();

            //x coordinate on the texture
            let mut tex_x = (wall_x * 256.0).floor() as i32;
            if side == 0 || side == 1 && ray_dir_x > 0.0 { tex_x = 256 - tex_x - 1; }
            if side == 2 || side == 3 && ray_dir_y < 0.0 { tex_x = 256 - tex_x - 1; }

            ray.length = perp_wall_dist;
            ray.texture_pos = tex_x;
            ray.side = side;

            self.play.rays.push(ray);
        }
    }


}