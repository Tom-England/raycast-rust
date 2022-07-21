#[derive(Copy, Clone)]
pub struct Wall{
    pub start: (f64, f64),
    pub end: (f64, f64),
}

impl Default for Wall{
    fn default () -> Wall {
        Wall{start: (0.0, 0.0), end: (0.0, 0.0)}
    }
}

impl Wall{
    pub fn calc_length(start: (f64, f64), end: (f64, f64)) -> f64{
        // c = sqrt(a^2 + b^2)
        let a = end.0 - start.0;
        let b = end.1 - start.1;
        let length: f64 = (a * a + b * b).sqrt();
        return length;
    }
    pub fn dist_along_wall(&mut self, point: (f64, f64)) -> f32{
        let dist: f32 = (Wall::calc_length(self.start, point) / Wall::calc_length(self.start, self.end)) as f32;
        return dist;
    }
}