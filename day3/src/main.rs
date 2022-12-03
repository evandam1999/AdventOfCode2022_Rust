use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

struct RuckSack {
    comp1: String,
    comp2: String,
    comp1set: HashSet<char>,
    comp2set: HashSet<char>,
}

impl RuckSack {
    fn init(input: &str) -> RuckSack {
        let split_index = input.chars().count() / 2;
        let c1 = &input[split_index..];
        let c2 = &input[..split_index];
        let mut c1set = HashSet::new();
        for c in c1.chars() {
            c1set.insert(c);
        }
        let mut c2set = HashSet::new();
        for c in c2.chars() {
            c2set.insert(c);
        }
        RuckSack { comp1: (c1.to_string()), comp2: (c2.to_string()), comp1set: c1set, comp2set: c2set }
    }
}

fn score_item(c: char) -> u32 {
    let mut score:u32 = 0;
    if c as u32 >= 97 {
        score = c as u32 - 96;
    }
    else {
        score = c as u32 - 38;
    }
    if score < 1 || score > 52 {
        let error = format!("{}:{} score {}", c, c as u32, score);
        panic!("{}", error)
    }
    score
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    for line in reader.lines() {
        let sack = RuckSack::init(line.as_ref().unwrap());
        let mut common = sack.comp1set.intersection(&sack.comp2set);
        score = score + score_item(*common.next().unwrap());
        assert_eq!(common.next(), None)
    }
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score() {
        let result =  score_item('a');
        assert_eq!(result, 1);
        let result =  score_item('b');
        assert_eq!(result, 2);
        let result =  score_item('z');
        assert_eq!(result, 26);
        let result =  score_item('A');
        assert_eq!(result, 27);
        let result =  score_item('Y');
        assert_eq!(result, 51);
        let result =  score_item('Z');
        assert_eq!(result, 52);
    }
}