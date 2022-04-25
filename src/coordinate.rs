#[derive(Copy, Clone)]
pub struct Coordinate {
    x: i32,
    y: i32,
}


impl Coordinate {
    pub fn new(in_x: &i32, in_y: &i32) -> Coordinate {
        Coordinate {
            x: *in_x,
            y: *in_y,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn to_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn println(&self) {
        println!("{:?}", self.to_tuple())
    }
}