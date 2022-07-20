use std::f64::consts::PI;

use crate::ray;

pub struct Player{
    pub view_direction: f64,
    pub pos: (f64, f64),
    pub rays: Vec<ray::Ray>,
    pub fov: u16,
}

impl Player {
    pub fn advance(&mut self, amount: f64) {
        let x = amount * (self.view_direction * PI / 180.0).cos();
        let y = amount * (self.view_direction * PI / 180.0).sin();
        self.pos.0 += x;
        self.pos.1 += y;
        for i in 0..self.rays.len(){
            self.rays[i].start = self.pos;
            self.rays[i].end = self.rays[i].calc_end();
        }
    }

    pub fn turn(&mut self, amount: f64) {
        self.view_direction += amount;
        for i in 0..self.rays.len(){
            self.rays[i].turn(amount);   
        }
        if self.view_direction > 360.0 {
            self.view_direction -= 360.0;
        }
        if self.view_direction < 0.0 {
            self.view_direction += 360.0;
        }
    }

    pub fn gen_rays(&mut self){
        let start: i32 = 0-(self.fov as i32/2);
        for i in start..(self.fov/2) as i32{
            self.rays.push(ray::Ray {
                start: self.pos,
                angle: i as f64,
                max_length: 200.0,
                end: (10.0, 10.0),
                length: 0.0,
                collided: false
            },);
        }
    }
}