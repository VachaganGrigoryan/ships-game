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
            let mut x: usize;
            let mut y: usize;
            loop {
                x = (rng.gen_range(0..self.rx)).try_into().unwrap();
                y = (rng.gen_range(0..self.ry)).try_into().unwrap();

                let new_crd = Coordinate::new(&x, &y);
                if !self.ships_crd.iter().any(|s| *s == new_crd) {
                    break;
                }
            }

            let new_crd = Coordinate::new(&x, &y);
            self.ships_crd.push(new_crd)
        }

    }
}