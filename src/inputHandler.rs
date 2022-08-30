use piston::Button;

pub struct InputHandler{
    pub turn: i8,
    pub adv: i8
}

impl InputHandler{
    pub fn new() -> Self{
        InputHandler{
            turn: 0,
            adv: 0
        }
    }
    
    pub fn key_press(&mut self, args: &Button) {
		use piston::Button::Keyboard;
		use piston::Key;		
		
        match *args {
            Keyboard(Key::Left) => self.turn = -1,
            Keyboard(Key::Right) => self.turn = 1,
            Keyboard(Key::Up) => self.adv = 1,
            Keyboard(Key::Down) => self.adv = -1,
            _ => (),
        }
    }

    pub fn key_up(&mut self, args: &Button) {
		use piston::Button::Keyboard;
		use piston::Key;		
		
        match *args {
            Keyboard(Key::Left) => self.turn = 0,
            Keyboard(Key::Right) => self.turn = 0,
            Keyboard(Key::Up) => self.adv = 0,
            Keyboard(Key::Down) => self.adv = 0,
            _ => (),
        }
    }
}