use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;

struct amove {
    count: i32,
    from: i32,
    to: i32,
}

impl amove {
    fn init(input: &str) -> amove {
        let data:Vec<&str> = input.split(' ').collect();
        assert_eq!(data.len(), 6, "Error in move input, not 6 items");
        amove { count: data[1].parse::<i32>().unwrap(),
            from: data[3].parse::<i32>().unwrap(),
            to: data[5].parse::<i32>().unwrap()}
    }
}

#[derive(Debug)]
struct stack {
    _1: VecDeque<char>,
    _2: VecDeque<char>,
    _3: VecDeque<char>,
    _4: VecDeque<char>,
    _5: VecDeque<char>,
    _6: VecDeque<char>,
    _7: VecDeque<char>,
    _8: VecDeque<char>,
    _9: VecDeque<char>,
}

impl stack {
    fn add_from_top(&mut self, input: &str) {
        let v:Vec<char> = input.chars().collect();
        if v[1] != ' ' { self._1.push_front(v[1])};
        if v[5] != ' ' { self._2.push_front(v[2])};
        if v[9] != ' ' { self._3.push_front(v[3])};
        if v[13] != ' ' { self._4.push_front(v[4])};
        if v[17] != ' ' { self._5.push_front(v[5])};
        if v[21] != ' ' { self._6.push_front(v[6])};
        if v[25] != ' ' { self._7.push_front(v[7])};
        if v[29] != ' ' { self._8.push_front(v[8])};
        if v[33] != ' ' { self._9.push_front(v[9])};
    }
}

impl Default for stack {
    fn default() -> Self {
        stack { _1: VecDeque::new(),
            _2: VecDeque::new(),
            _3: VecDeque::new(),
            _4: VecDeque::new(),
            _5: VecDeque::new(),
            _6: VecDeque::new(),
            _7: VecDeque::new(),
            _8: VecDeque::new(),
            _9: VecDeque::new(),
        }
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line);
    }
    //Start here have amove and stack implemented at least started.
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_amove_init() {
        let x = amove::init("move 1 from 3 to 9");
        assert_eq!(x.count, 1);
        assert_eq!(x.from, 3);
        assert_eq!(x.to, 9);
    }

    #[test]
    fn test_add_from_top() {
        let mut a = stack::default();
        a.add_from_top("[D]                     [N] [F]    ");
        let c: Vec<&char> = a._1.iter().collect();
        // println!("{:?}", a);
        assert_eq!(*c[0], 'D');
        assert_eq!(a._2.is_empty(), true);
    }

    #[test]
    fn test_input_temp() {
        let input:&str = "[D]                     [N] [F]";
        let v:Vec<char> = input.chars().collect();
        assert_eq!(v[1], 'D');
        assert_eq!(v[5], ' ');
        assert_eq!(v[25], 'N');
        assert_eq!(v[29], 'F');
    }

}