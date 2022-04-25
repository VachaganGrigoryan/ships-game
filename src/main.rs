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

    fn show(self) {
        for line in self.fields.iter() {
            println!("{:?}", line)
        }
    }
}


fn main() {

    let bt = BermudaTriangle::new(10, 10, 5);
    bt.show();

}
