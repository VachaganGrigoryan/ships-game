use std::fmt;

#[derive(Copy, Clone)]
pub struct Coordinate {
    x: usize,
    y: usize,
}


impl Coordinate {
    pub fn new(in_x: &usize, in_y: &usize) -> Coordinate {
        Coordinate {
            x: *in_x,
            y: *in_y,
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coordinate {}


impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}