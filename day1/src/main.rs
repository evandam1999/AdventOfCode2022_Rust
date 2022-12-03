use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    calc_calories("./src/input.txt").expect("error");
    calc_top_three("./src/input.txt").expect("error");
}

fn calc_calories(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut largest = 0;
    let mut current = 0;
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.as_ref().unwrap() == "" {
            // println!("total is: {}", current);
            if current > largest {
                largest = current;
            }
            current = 0;
        }
        else {
            current += line.as_ref().unwrap().parse::<i32>().unwrap();
        }
    }
    println!("largest {}", largest);

    Ok(())
}

fn calc_top_three(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut largest: [i32; 3] = [0, 0, 0];
    let mut current = 0;
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.as_ref().unwrap() == "" {
            // println!("total is: {}", current);
            largest.sort();
            let min = largest.iter().min();

            if current > *min.unwrap() {
                largest[0] = current;
            }
            current = 0;
        }
        else {
            current += line.as_ref().unwrap().parse::<i32>().unwrap();
        }
    }
    println!("top three total {}", largest.iter().sum::<i32>());

    Ok(())
}
