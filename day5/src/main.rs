use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;
use std::iter;

#[derive(Debug)]
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
            from: data[3].parse::<i32>().unwrap() - 1,
            to: data[5].parse::<i32>().unwrap() - 1}
    }
}

#[derive(Debug)]
struct stack {
    s: Vec<VecDeque<char>>,
}


impl stack {
    fn add_from_top(&mut self, input: &str) {
        let v:Vec<char> = input.chars().collect();
        // println!("{:?}", input.len());
        let len = input.len();
        if len >= 1 {if v[1] != ' ' { self.s[0].push_front(v[1])}};
        if len >= 5 {if v[5] != ' ' { self.s[1].push_front(v[5])}};
        if len >= 9 {if v[9] != ' ' { self.s[2].push_front(v[9])}};
        if len >= 13 {if v[13] != ' ' { self.s[3].push_front(v[13])}};
        if len >= 17 {if v[17] != ' ' { self.s[4].push_front(v[17])}};
        if len >= 21 {if v[21] != ' ' { self.s[5].push_front(v[21])}};
        if len >= 25 {if v[25] != ' ' { self.s[6].push_front(v[25])}};
        if len >= 29 {if v[29] != ' ' { self.s[7].push_front(v[29])}};
        if len >= 33 {if v[33] != ' ' { self.s[8].push_front(v[33])}};
    }

    //TODO:  Issue is here in that moving 2 should move them as a group not one at a time??
    fn process_move(&mut self, input: &amove) {
        let from: usize = input.from as usize;
        let to: usize = input.to as usize;
        for i in 0..input.count {
            let moving = self.s[from].pop_back();
            self.s[to].push_back(moving.unwrap());
        }
    }
}

impl Default for stack {
    fn default() -> Self {
        // Has to be a better way
        stack { s: vec![VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            ]
        }
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut s: stack = stack::default();
    let mut moves: VecDeque<amove> = VecDeque::new();

    for line in reader.lines() {
        if line.as_ref().unwrap().contains("[") {
            println!("{:?}", line);
            s.add_from_top(&line.unwrap())
        }
        else if line.as_ref().unwrap().contains("move") {
            let a = amove::init(&line.unwrap());
            moves.push_back(a);
        }
    }
    // println!("{:?}", s);
    for m in moves {
        s.process_move(&m);
    }
    println!("{:?}", s);
    for c in &s.s {
        println!("{:?}", c.back().unwrap());
    }
    println!("{}", "I NEED TO CLEAN THIS OUTPUT UP to just give the answer");

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_amove_init() {
        let x = amove::init("move 1 from 3 to 9");
        assert_eq!(x.count, 1);
        assert_eq!(x.from, 2);
        assert_eq!(x.to, 8);
    }

    #[test]
    fn test_add_from_top() {
        let mut a = stack::default();
        a.add_from_top("[D]                     [N] [F]    ");
        let c: Vec<&char> = a.s[0].iter().collect();
        // println!("{:?}", a);
        assert_eq!(*c[0], 'D');
        assert_eq!(a.s[1].is_empty(), true);
    }

    #[test]
    fn test_proc_move() {
        let mut s = stack::default();
        s.add_from_top("[D]                     [N] [F]    ");
        s.add_from_top("[X]                     [N] [F]    ");
        let a = amove::init("move 2 from 1 to 2");
        s.process_move(&a);
        println!("{:?}", s)
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