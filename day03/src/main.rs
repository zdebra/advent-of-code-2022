use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut total = 0;
    let mut lines = io::BufReader::new(file).lines();
    loop {
        let l1 = match lines.next() {
            None => break,
            Some(v) => v.unwrap(),
        };
        let l2 = lines.next().unwrap().unwrap();
        let l3 = lines.next().unwrap().unwrap();

        let (o1, o2, o3) = (occurrence(&l1), occurrence(&l2), occurrence(&l3));
        for ch in o1.iter() {
            if o2.contains(ch) && o3.contains(ch) {
                total += *ch as u32;
            }
        }
    }
    println!("total is {}", total);
}

fn occurrence(compartment: &str) -> HashSet<u8> {
    let mut ct = HashSet::new();
    for ch in compartment.chars() {
        ct.insert(ch_as_u8(ch));
    }
    ct
}

fn ch_as_u8(ch: char) -> u8 {
    let ch = ch as u8;
    match ch {
        d if d > 96 && d < 123 => d - 96,
        d if d > 64 && d < 91 => d - 38,
        _ => panic!("unexpected input"),
    }
}

fn number_to_char(n: u8) -> char {
    match n {
        d if d > 0 && d < 27 => (d + 64) as char,
        d if d > 26 && d < 53 => (d + 70) as char,
        _ => panic!("unexpected input"),
    }
}

#[cfg(test)]
mod tests {
    use crate::occurrence;

    #[test]
    fn occurence_to_number() {
        let ct = occurrence("azAZ");
        assert!(ct.contains(&1));
        assert!(ct.contains(&26));
        assert!(ct.contains(&27));
        assert!(ct.contains(&52));
    }
}
