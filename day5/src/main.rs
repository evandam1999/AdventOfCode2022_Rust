use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line);
    }
}
