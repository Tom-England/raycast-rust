use std::f64::consts::PI;

pub struct Ray{
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub angle: f64,
    pub length: f64,
    pub max_length: f64,
    pub collided: bool,
    pub tex: image::RgbaImage,
    pub wall_pos: f32
}

impl Ray{
    // Calculate the new endpoint of the ray
    pub fn calc_end(&mut self) -> (f64, f64){
        let a_rad: f64 = self.angle * PI/180.0;
        let y = self.max_length * a_rad.sin();
        let x = self.max_length * a_rad.cos();
        return (self.start.0 + x, self.start.1 + y);
    }
    
    pub fn calc_length(&mut self) -> f64 {
        return Ray::calc_length_of_ray(self.start, self.end);
    }

    pub fn calc_length_of_ray(start: (f64, f64), end: (f64, f64)) -> f64{
        // c = sqrt(a^2 + b^2)
        let a = end.0 - start.0;
        let b = end.1 - start.1;
        let length: f64 = (a * a + b * b).sqrt();
        return length;
    }

    // Turn the ray
    pub fn turn(&mut self, step:f64){
        self.angle += step;
        if self.angle > 360.0 {
            self.angle -= 360.0;
        }
        if self.angle < 0.0 {
            self.angle += 360.0;
        }
        self.end = self.calc_end()
    }

    // Find the point of intesection between a ray and a wall
    pub fn find_intersection(&mut self, wall_start : (f64, f64), wall_end : (f64, f64)) -> (f64, f64){
        let den = (wall_start.0 - wall_end.0) * (self.start.1 - self.end.1) - (wall_start.1 - wall_end.1) * (self.start.0 - self.end.0);
		if den == 0.0 {
			return (-1.0, -1.0);
		}
		let t = ((wall_start.0 - self.start.0) * (self.start.1 - self.end.1) - (wall_start.1 - self.start.1) * (self.start.0 - self.end.0)) / den;
		let u = -((wall_start.0 - wall_end.0) * (wall_start.1 - self.start.1) - (wall_start.1 - wall_end.1) * (wall_start.0 - self.start.0)) / den;
		if t > 0.0 && t < 1.0 && u > 0.0 {
			//std::cout << "True" << std::endl;
            let mut end: (f64,f64) = (-1.0, -1.0);
            let x = wall_start.0 + t * (wall_end.0 - wall_start.0);
			let y = wall_start.1 + t * (wall_end.1 - wall_start.1);
			let mag2 = ((x-self.start.0) * (x - self.start.0)) + ((y - self.start.1) * (y - self.start.1));
			let mag2old = (self.end.0 - self.start.0) * (self.end.0 - self.start.0) + (self.end.1 - self.start.1) * (self.end.1 - self.start.1);
			if mag2 < mag2old {
				end = (x, y);
			}
			return end;
		}
		else {
			//std::cout << "False" << std::endl;
			return (-1.0, -1.0);
			//std::cout << t << " " << u << std::endl;
		}
    }
}