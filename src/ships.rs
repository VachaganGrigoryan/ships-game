use rand::Rng;
use crate::coordinate::Coordinate;

pub struct Ships {
    rx: i32,
    ry: i32,
    count: u16,
    pub(crate) ships_crd: Vec<Coordinate>,
}

impl Ships {
    pub fn new(in_rx: i32, in_ry: i32, in_count: u16) -> Ships {
        let mut s = Ships {
            rx: in_rx,
            ry: in_ry,
            count: in_count,
            ships_crd: vec![]
        };

        s.populate();
        return s;
    }

    fn populate(&mut self) {
        for _ in 0..self.count {
            let mut rng = rand::thread_rng();
            let mut  x;
            let mut  y;
            loop {
                x = rng.gen_range(0..self.rx);
                y = rng.gen_range(0..self.ry);

                if self.free_coordinate(x, y) {
                    break;
                }
            }

            let new_crd = Coordinate::new(&x, &y);
            self.ships_crd.push(new_crd)
        }

    }

    fn free_coordinate(&self, x: i32, y: i32) -> bool {
        for ship in self.ships_crd.iter() {
            if ship.get_x() == x && ship.get_y() == y {
                return false;
            }
        }
        return true;
    }

    pub fn show(self) {
        for ship in self.ships_crd.iter() {
            ship.println()
        }
    }
}