pub struct Sprite{
    pub pos: (f64, f64),
    pub texture_index: u8,
    pub dist: f64
}

impl Sprite{
    pub fn eucl_dist(&mut self, pos2: (f64, f64)) -> f64{
        // c^2 = a^2 + b^2
        let a = pos2.0 - self.pos.0;
        let b = pos2.1 - self.pos.1;
        return (a*a + b*b).sqrt();
    }
}