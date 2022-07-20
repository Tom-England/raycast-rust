use crate::wall;

pub struct Cell{
    pub walls: [wall::Wall; 4],
    pub pos: (f64, f64),
    pub l: f64
}
impl Cell {
    pub fn create_walls(&mut self){
        // Top
        self.walls[0] = wall::Wall {
            start: (self.pos.0 * self.l, self.pos.1 * self.l), 
            end: (self.pos.0 * self.l + self.l, self.pos.1 * self.l)
        };
        // Right
        self.walls[1] = wall::Wall {
            start: (self.pos.0 * self.l + self.l, self.pos.1 * self.l), 
            end: (self.pos.0 * self.l + self.l, self.pos.1 * self.l + self.l)
        };
        // Bottom
        self.walls[2] = wall::Wall {
            start: (self.pos.0 * self.l, self.pos.1 * self.l + self.l), 
            end: (self.pos.0 * self.l + self.l, self.pos.1 * self.l + self.l)
        };
        // Left
        self.walls[3] = wall::Wall {
            start: (self.pos.0 * self.l, self.pos.1 * self.l), 
            end: (self.pos.0 * self.l, self.pos.1 * self.l + self.l)
        };
    }
}

pub struct Map{
    pub cells: Vec<Cell>
}

impl Map {
    
}