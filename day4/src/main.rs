use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Orders {
    g1_start: i32,
    g1_end: i32,
    g2_start: i32,
    g2_end: i32,
}

impl Orders{
    fn init(input: &str) -> Orders {
        let mut values: Vec<&str> = vec!();
        let data: Vec<&str> = input.split(',').collect();

        //I wanted to do something clever here
        for x in data {
            let val = x.split('-');
            for y in val {
                values.push(y);
            }
        }
        Orders { g1_start: values[0].parse::<i32>().unwrap(),
            g1_end: values[1].parse::<i32>().unwrap(),
            g2_start: values[2].parse::<i32>().unwrap(),
            g2_end: values[3].parse::<i32>().unwrap() }
    }

    fn full_overlap(&self) -> bool {
        if self.g1_start >= self.g2_start && self.g1_end <= self.g2_end { true }
        else if self.g2_start >= self.g1_start && self.g2_end <= self.g1_end{ true }
        else { false }
    }

    fn any_overlap(&self) -> bool {
        if self.full_overlap() { true }
        else {
            if self.g1_start <= self.g2_end && self.g2_start <= self.g1_end { true }
            else { false }
        }
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut full_count = 0;
    let mut any_count = 0;

    for line in reader.lines() {
        let order = Orders::init(&line.unwrap());
        if order.full_overlap() {
            full_count += 1;
        }
        if order.any_overlap() {
            any_count += 1;
        }
    }
    println!("full overlap count: {full_count}");
    println!(" any overlap count: {any_count}");
    assert_eq!(full_count, 453); // Part 1
    assert_eq!(any_count, 919); // Part 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_full_overlap() {
        let mut o = Orders::init("2-4,1-5");
        let mut result = o.full_overlap();
        assert_eq!(result, true);
        o = Orders::init("3-87,6-17");
        result = o.full_overlap();
        assert_eq!(result, true);
        o = Orders::init("3-5,5-8");
        result = o.full_overlap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_part_overlap() {
        let mut o = Orders::init("2-4,1-5");
        let mut result = o.any_overlap();
        assert_eq!(result, true);
        o = Orders::init("3-87,6-17");
        result = o.any_overlap();
        assert_eq!(result, true);
        o = Orders::init("3-4,5-7");
        result = o.any_overlap();
        assert_eq!(result, false);
        o = Orders::init("3-5,5-8");
        result = o.any_overlap();
        assert_eq!(result, true);
    }
}
