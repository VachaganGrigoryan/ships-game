use crate::coordinate::Coordinate;
use std::io::{BufRead};

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
        for _ in 0..self.rx {
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
            let pol = self.rate();
            self.check(&pol);

            if self.ships.ships_crd.is_empty() {
                break
            }
        }
        println!("|You Win|")
    }

    fn rate(&mut self) -> Coordinate {
        println!("X:");
        let x: i32 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");
        println!("Y:");
        let y: i32 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");

        if x > self.rx || x < 0 || y > self.ry || y < 0 {
            return self.rate();
        }

        Coordinate::new(&(x).try_into().unwrap(), &(y).try_into().unwrap())
    }

    fn check(&mut self, rate: &Coordinate) {
        if self.ships.ships_crd.iter().any(|s| *s == *rate) {
            self.fields[rate.get_x()][rate.get_y()] = '⚓';
            // Remove element
            let index = self.ships.ships_crd.iter().position(|x| *x == *rate).unwrap();
            self.ships.ships_crd.remove(index);
        } else {
            if self.fields[rate.get_x()][rate.get_y()] == '+' {
                self.fields[rate.get_x()][rate.get_y()] = '◽';
            }
            self.compass(*rate)
        }
    }

    fn compass(&mut self, rate: Coordinate) {
        let mut note = vec![vec![" . "; 5]; 5];
        let rate_str = rate.to_string();

        note[2][2] = &*rate_str;
        for ship in self.ships.ships_crd.iter() {
            if ship.get_x() < rate.get_x() && ship.get_y() < rate.get_y() {
                note[0][0] = "N W";
                note[1][1] = " ⇖ ";
            }
            if ship.get_x() == rate.get_x() && ship.get_y() < rate.get_y() {
                note[0][2] = " N ";
                note[1][2] = " ⇑ ";
            }
            if ship.get_x() > rate.get_x() && ship.get_y() < rate.get_y() {
                note[0][4] = "N E";
                note[1][3] = " ⇗ ";
            }
            if ship.get_x() < rate.get_x() && ship.get_y() == rate.get_y() {
                note[2][0] = " W ";
                note[2][1] = " ⇐ ";
            }
            if ship.get_x() > rate.get_x() && ship.get_y() == rate.get_y() {
                note[2][4] = " E ";
                note[2][3] = " ⇒ ";
            }
            if ship.get_x() < rate.get_x() && ship.get_y() > rate.get_y() {
                note[4][0] = "S W";
                note[3][1] = " ⇙ ";
            }
            if ship.get_x() == rate.get_x() && ship.get_y() > rate.get_y() {
                note[4][2] = " S ";
                note[3][2] = " ⇓ ";
            }
            if ship.get_x() > rate.get_x() && ship.get_y() > rate.get_y() {
                note[4][4] = "S E";
                note[3][3] = " ⇘ ";
            }
        }

        println!("\nCompass:");
        for line in note.iter() {
            println!("{}", line.join(""));
        }

    }

    fn show(&mut self) {
        println!("Game Board:");
        for j in 0..(self.ry).try_into().unwrap() {
            for i in 0..(self.rx).try_into().unwrap() {
                print!(" {} ", self.fields[i][j]);
            }
            println!();
        }
    }
}


fn main() {
    let mut bt = BermudaTriangle::new(10, 10, 5);
    bt.run();
}
