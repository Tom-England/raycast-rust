use crate::ray;

pub struct Player{
    pub plane: (f64, f64),
    pub dir: (f64, f64),
    pub pos: (f64, f64),
    pub rays: Vec<ray::Ray>
}

impl Player {
    pub fn advance(&mut self, amount: f64, dt: f64, direction: f64, map: &[[u8; 10]; 10]) {
        let a = amount*dt;
        if map[(self.pos.0 + (self.dir.0 * direction) * a )as usize][self.pos.1 as usize] == 0 { self.pos.0 += self.dir.0 * a * direction; }
        if map[self.pos.0 as usize][(self.pos.1 + (self.dir.1 * direction) * a) as usize] == 0 { self.pos.1 += self.dir.1 * a * direction; }
    }

    pub fn turn(&mut self, amount: f64, dt: f64) {
        //both camera direction and camera plane must be rotated
        let old_dir_x = self.dir.0;
        let a = amount * dt;
        self.dir.0 = self.dir.0 * a.cos() - self.dir.1 * a.sin();
        self.dir.1 = old_dir_x * a.sin() + self.dir.1 * a.cos();
        let old_plane_x = self.plane.0;
        self.plane.0 = self.plane.0 * a.cos() - self.plane.1 * a.sin();
        self.plane.1 = old_plane_x * a.sin() + self.plane.1 * a.cos();
    }
}