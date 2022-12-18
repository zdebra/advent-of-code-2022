use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const CHARS_LIMIT: usize = 14;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let line = io::BufReader::new(file).lines().next().unwrap().unwrap();

    let mut hm = HashMap::new();
    let chars = line.as_bytes();
    for (i, ch) in chars.iter().enumerate() {
        if i > CHARS_LIMIT - 1 {
            let v = hm.get_mut(&chars[i - CHARS_LIMIT]).unwrap();
            *v -= 1;
            if *v == 0 {
                hm.remove(&chars[i - CHARS_LIMIT]).unwrap();
            }
        }

        let entry = hm.entry(ch).or_insert(0);
        *entry += 1;

        if hm.len() == CHARS_LIMIT {
            println!("{}", i + 1);
            break;
        }
    }
}
