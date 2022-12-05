use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut calories = vec![];
    let mut calories_tmp = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            calories.push(calories_tmp);
            calories_tmp = 0;
            continue;
        }
        let num = line.parse::<usize>().unwrap();
        calories_tmp += num;
    }

    calories.sort();
    let len = calories.len();

    println!(
        "top 3 Elfs are carrying {} calories",
        calories[len - 1] + calories[len - 2] + calories[len - 3]
    );
}
