use crate::coordinate::Coordinate;
use crate::ships::Ships;

mod coordinate;
mod ships;


struct BermudaTriangle {
    rx: i32,
    ry: i32,
    fields: Vec<Vec<char>>,
    ships: ships::Ships,
}

impl BermudaTriangle {
    fn new(in_rx: i32, in_ry: i32, ships_count: u16) -> BermudaTriangle {
        let mut bt = BermudaTriangle {
            rx: in_rx,
            ry: in_ry,
            fields: vec![],
            ships: ships::Ships::new(in_rx, in_ry, ships_count),
        };
        bt.populate();
        return bt;
    }

    fn populate(&mut self) {
        for i in 0..self.rx {
            let mut line: Vec<char> = vec![];
            for _ in 0..self.ry {
                line.push('+');
            }
            self.fields.push(line);
        }
    }

    fn run(&mut self) {
        loop {
            self.show();
            self.check(self.rate());

            if self.ships.ships_crd.is_empty() {
                break
            }
        }
    }

    fn rate(&mut self) -> (&i32, &i32) {
        (&5, &3)
    }

    fn check(&mut self, rate: (&i32, &i32)) {
    //     todo need to complete
    }

    fn kill_ship(&self, rate: &Coordinate) -> (bool, Coordinate) {
        for ship in self.ships.ships_crd.iter() {
            if ship.get_x() == rate.get_x() && ship.get_y() == rate.get_y() {
                return (true, *ship);
            }
        }
        return (false, *rate);
    }

    fn show(&mut self) {
        for line in self.fields.iter() {
            println!("{:?}", line)
        }
    }
}


fn main() {

    let mut bt = BermudaTriangle::new(10, 10, 5);
    bt.run();

}
