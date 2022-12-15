use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut cnt = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (sec_a, sec_b) = line.split_once(",").unwrap();
        let (sec_a, sec_b) = (section::new(sec_a), section::new(sec_b));
        if sec_a.overlap(&sec_b) || sec_b.overlap(&sec_a) {
            cnt += 1;
        }
    }
    println!("{cnt}");
}

struct section {
    start: u32,
    end: u32,
}

impl section {
    fn new(sec_str: &str) -> Self {
        let (start, end) = sec_str.split_once("-").unwrap();
        Self {
            start: start.parse::<u32>().unwrap(),
            end: end.parse::<u32>().unwrap(),
        }
    }

    fn is_subset_of(&self, b: &section) -> bool {
        return self.start >= b.start && self.end <= b.end;
    }

    fn overlap(&self, b: &section) -> bool {
        // bbbb
        // xaab
        if self.is_subset_of(b) {
            return true;
        }

        // --aaaa
        // bbbb--
        if b.end >= self.start && b.end <= self.end {
            return true;
        }

        // aaaa--
        // --bbbb
        if b.start <= self.end && b.end >= self.end {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn section() {
        let a = section::new("1-3");
        let b = section::new("0-4");
        assert!(a.is_subset_of(&b));
        assert!(!b.is_subset_of(&a));
    }
}
