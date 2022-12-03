use std::fs::File;
use std::io::{prelude::*, BufReader, Seek, SeekFrom};

#[derive(Debug)]
struct rpsGame {
    opp: String,
    me: String,
}

impl rpsGame {
    fn init(input: &str) -> rpsGame {
        let mut sstr = input.split(" ");
        rpsGame { opp: (sstr.next().unwrap().to_string()), me: (sstr.next().unwrap().to_string()) }
    }

    fn score_wld(&self) -> i32 {
        if self.me == "X" { //Rock
            match self.opp.as_str() {
                "A" => return 3,
                "B" => return 0,
                "C" => return 6,
                _ => panic!("error")
            }
        } else if self.me == "Y" { //Paper
            match self.opp.as_str() {
                "A" => return 6,
                "B" => return 3,
                "C" => return 0,
                _ => panic!("error")
            }
        } else { //Scissors
            match self.opp.as_str() {
                "A" => return 0,
                "B" => return 6,
                "C" => return 3,
                _ => panic!("error")
            }
        }
    }

    fn score_me(&self) -> i32 {
        match self.me.as_str() {
            "X" => return 1,
            "Y" => return 2,
            "Z" => return 3,
            _ => panic!("error")
        }
    }

    fn change_my_play(&mut self) {
        if self.me == "X" { //Lose
            match self.opp.as_str() {
                "A" => self.me = "Z".to_string(),
                "B" => self.me = "X".to_string(),
                "C" => self.me = "Y".to_string(),
                _ => panic!("error")
            }
        } else if self.me == "Y" { //Draw
            match self.opp.as_str() {
                "A" => self.me = "X".to_string(),
                "B" => self.me = "Y".to_string(),
                "C" => self.me = "Z".to_string(),
                _ => panic!("error")
            }
        } else { //Win
            match self.opp.as_str() {
                "A" => self.me = "Y".to_string(),
                "B" => self.me = "Z".to_string(),
                "C" => self.me = "X".to_string(),
                _ => panic!("error")
            }
        }
    }

}


impl PartialEq for rpsGame {
    fn eq(&self, other: &Self) -> bool {
        self.me == other.me && self.opp == other.opp
    }
}

fn main() {
    let file = File::open("./src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut myscore1 = 0;
    let mut myscore2 = 0;
    for line in reader.lines() {
        let mut hand = rpsGame::init(line.as_ref().unwrap());
        myscore1 = myscore1 + hand.score_wld() + hand.score_me();

        hand.change_my_play();
        myscore2 = myscore2 + hand.score_wld() + hand.score_me();
    }
    println!("my score1: {}", myscore1);
    println!("my score2: {}", myscore2);


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let result = rpsGame::init("A X");
        assert_eq!(result, rpsGame{ opp: ("A".to_string()), me: ("X".to_string()) });

        let result = rpsGame::init("B X");
        assert_eq!(result, rpsGame{ opp: ("B".to_string()), me: ("X".to_string()) });
    }

    #[test]
    fn test_wld() {
        let hand = rpsGame{ opp: ("A".to_string()), me: ("X".to_string()) };
        let result = hand.score_wld();
        assert_eq!(result, 3);
        let hand = rpsGame{ opp: ("A".to_string()), me: ("Y".to_string()) };
        let result = hand.score_wld();
        assert_eq!(result, 6);
    }
}