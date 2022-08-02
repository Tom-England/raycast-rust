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
    pub fn make_map(&mut self){
        let map_arr: [[u8; 10]; 10] = [
            [1,1,1,1,1,1,1,1,1,1],
            [1,0,0,0,0,0,0,0,0,1],
            [1,0,1,0,1,0,0,1,0,1],
            [0,0,0,0,1,0,0,0,0,1],
            [0,0,0,0,1,0,0,0,0,1],
            [0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,1],
            [1,0,1,0,1,0,0,1,0,1],
            [1,0,0,0,1,0,0,0,0,1],
            [1,1,1,1,1,1,1,1,1,1]
        ];
        for y in 0..map_arr.len(){
            for x in 0..map_arr[y].len(){
                if map_arr[y][x] == 1{
                    self.cells.push(Cell{
                        walls: [wall::Wall::default(); 4],
                        pos: (x as f64, y as f64),
                        l: 1.3
                    })
                } 
            }
        }

        self.build_all_cell_walls();
    }
    
    fn build_all_cell_walls(&mut self){
        for i in 0..self.cells.len(){
            self.cells[i].create_walls();
        }
    }
}