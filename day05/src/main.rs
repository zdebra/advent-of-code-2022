use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    // let read_stacks = true;
    // for line in io::BufReader::new(file).lines() {
    //     let line = line.unwrap();
    //     println!("{}", line.as_bytes()[1] as char);
    // }

    let mut stacks = vec![Vec::<char>::new(); 9];
    let mut lines = io::BufReader::new(file).lines();

    // parsing starting scheme
    loop {
        let line = match lines.next() {
            None => break,
            Some(v) => {
                let v = v.unwrap();
                if v.is_empty() {
                    break;
                }
                v
            }
        };

        let mut index = 0;
        for (_, ch) in line
            .chars()
            .enumerate()
            .filter(|(i, _)| i == &1 || i % 4 == 1)
        {
            if !ch.is_whitespace() && !ch.is_numeric() {
                stacks[index].insert(0, ch);
            }
            index += 1;
        }
    }

    let re = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();

    // parsing instructions
    loop {
        let line = match lines.next() {
            None => break,
            Some(v) => v.unwrap(),
        };

        let caps = re.captures(&line).unwrap();
        let cnt = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        for _ in 0..cnt {
            let tmp_crate = &stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(*tmp_crate);
        }
    }

    for stack in &stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}
