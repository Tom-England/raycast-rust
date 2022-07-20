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