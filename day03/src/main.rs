use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut total: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (first_comp, second_comp) = line.split_at(line.len() / 2);

        let first_comp_ct = occurrence(first_comp);
        let second_comp_ct = occurrence(second_comp);

        for item in first_comp_ct.iter() {
            if second_comp_ct.contains(item) {
                total += *item as u32;
                // println!("{} for {}", number_to_char(*item), item);
            }
        }
    }

    println!("total is {}", total);
}

fn occurrence(compartment: &str) -> HashSet<u8> {
    let mut ct = HashSet::new();
    for ch in compartment.chars() {
        let ch_as_int = ch as u8;

        match ch_as_int {
            d if d > 96 && d < 123 => ct.insert(ch_as_int - 96),
            d if d > 64 && d < 91 => ct.insert(ch_as_int - 38),
            _ => panic!("unexpected input"),
        };
    }
    ct
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
