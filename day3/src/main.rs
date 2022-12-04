use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

struct RuckSack {
    all: String,
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
        RuckSack { all: input.to_string(), comp1set: c1set, comp2set: c2set }
    }
}

fn score_item(c: char) -> u32 {
    let score:u32;
    if c as u32 >= 97 {
        score = c as u32 - 96;
    }
    else {
        score = c as u32 - 38;
    }
    if score < 1 || score > 52 {
        panic!("{}", format!("{}:{} score {}", c, c as u32, score))
    }
    score
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut groupscore = 0;
    let mut count = 0;
    let mut groupset1 = HashSet::<char>::new();
    let mut groupset2 = HashSet::<char>::new();
    let mut groupset3 = HashSet::<char>::new();
    let mut maingroupset = HashSet::<char>::new();

    for line in reader.lines() {
        // Part 1
        count += 1;
        let sack = RuckSack::init(line.as_ref().unwrap());
        let mut common = sack.comp1set.intersection(&sack.comp2set);
        score = score + score_item(*common.next().unwrap());
        assert_eq!(common.next(), None);

        // Part 2
        match count % 3 {
            1 => {
                for c in sack.all.chars() {
                    groupset1.insert(c);
                }
            }
            2 => {
                for c in sack.all.chars() {
                    groupset2.insert(c);
                }
            }
            0 => {
                for c in sack.all.chars() {
                    groupset3.insert(c);
                }
                // Tricky part
                let mut group_common = groupset1.intersection(&groupset2);

                for item in group_common {
                    maingroupset.insert(*item);
                }
                group_common = groupset3.intersection(&maingroupset);
                groupscore += score_item(*group_common.next().unwrap());
                assert_eq!(common.next(), None);

                groupset1.clear();
                groupset2.clear();
                groupset3.clear();
                maingroupset.clear();
            }
            _ => { panic!("error"); }
        }
    }
    // Part 1
    println!("Score: {}", score);
    // Part 2
    println!("Group Score: {}", groupscore);
    // Correct Answers
    assert_eq!(score, 8493);
    assert_eq!(groupscore, 2552);
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