use std::f64::consts::PI;

use crate::ray;

pub struct Player{
    pub view_direction: f64,
    pub pos: (f64, f64),
    pub rays: Vec<ray::Ray>,
    pub fov: u16,
}

impl Player {
    pub fn advance(&mut self, amount: f64, dt: f64) {
        let x = amount * dt * (self.view_direction * PI / 180.0).cos();
        let y = amount * dt * (self.view_direction * PI / 180.0).sin();
        self.pos.0 += x;
        self.pos.1 += y;
        for i in 0..self.rays.len(){
            self.rays[i].start = self.pos;
            self.rays[i].end = self.rays[i].calc_end();
        }
    }

    pub fn turn(&mut self, amount: f64, dt: f64) {
        self.view_direction += amount * dt;
        for i in 0..self.rays.len(){
            self.rays[i].turn(amount * dt);   
        }
        if self.view_direction > 360.0 {
            self.view_direction -= 360.0;
        }
        if self.view_direction < 0.0 {
            self.view_direction += 360.0;
        }
    }

    pub fn gen_rays(&mut self){
        
        /*let start: i32 = 0-(self.fov as i32/2);
        for i in start..(self.fov/2) as i32{
            self.rays.push(ray::Ray {
                start: self.pos,
                angle: i as f64,
                max_length: 200.0,
                end: (10.0, 10.0),
                length: 0.0,
                collided: false
            },);
        }*/

        let ray_count = 300;
        let start: i32 = 0-(self.fov as i32/2);
        let increment: f64 = self.fov as f64 / ray_count as f64;
        //print!("increment {0}\n", increment);
        for i in 0..ray_count{
            self.rays.push(ray::Ray{
                start: self.pos,
                angle: start as f64 + (increment * i as f64) ,
                max_length: 20.0,
                end: (10.0, 10.0),
                length: 0.0,
                collided: false,
                tex: image::ImageBuffer::new(1, 1),
                wall_pos: 0.0
            },);
            //print!("angle: {0}", start as f64 + (increment * i as f64) );
        }
    }
}