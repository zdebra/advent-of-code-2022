use core::panic;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let (mut tail, mut head) = (Point::new(), Point::new());
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (direction, cnt) = line
            .split_once(" ")
            .expect("expected 2 strings separated by a space");
        let cnt: usize = cnt.parse().expect("cnt supposed to be a number");
        for _ in 0..cnt {
            match direction {
                "U" => head.up(),
                "R" => head.right(),
                "D" => head.down(),
                "L" => head.left(),
                _ => panic!("unexpected direction {}", direction),
            }

            if head_tail_positions_check(&head, &tail) {
                continue;
            }

            adjust_tail(&head, &mut tail);
            visited.insert((tail.x, tail.y));
        }
    }

    println!("visited: {}", visited.len());
}

fn print_visited(visited: &HashSet<(isize, isize)>, limit: isize) {
    for y in (0..limit).rev() {
        for x in 0..limit {
            if (x, y) == (0, 0) {
                print!("s")
            } else if visited.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }
}

fn print_field(head: &Point, tail: &Point, size: isize) {
    for y in (0..size).rev() {
        for x in 0..size {
            if head.x == x && head.y == y {
                print!("H");
            } else if tail.x == x && tail.y == y {
                print!("T");
            } else if (x, y) == (0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!();
}

fn adjust_tail(head: &Point, tail: &mut Point) {
    // points are on the same col
    if head.x == tail.x {
        if head.y > tail.y {
            tail.up();
        } else {
            tail.down();
        }
        return;
    }

    // points are on the same row
    if head.y == tail.y {
        if head.x > tail.x {
            tail.right();
        } else {
            tail.left();
        }
        return;
    }

    if head.x > tail.x && head.y > tail.y {
        tail.up_right();
        return;
    }
    if head.x > tail.x && head.y < tail.y {
        tail.down_right();
        return;
    }
    if head.x < tail.x && head.y < tail.y {
        tail.down_left();
        return;
    }
    if head.x < tail.x && head.y > tail.y {
        tail.up_left();
        return;
    }

    unreachable!()
}

fn head_tail_positions_check(head: &Point, tail: &Point) -> bool {
    if isize::abs(head.x - tail.x) > 1 {
        return false;
    }
    if isize::abs(head.y - tail.y) > 1 {
        return false;
    }
    true
}

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    fn from(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }

    fn up_right(&mut self) {
        self.up();
        self.right();
    }

    fn down_right(&mut self) {
        self.down();
        self.right();
    }

    fn down_left(&mut self) {
        self.down();
        self.left();
    }

    fn up_left(&mut self) {
        self.up();
        self.left();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjust_tail_test() {
        let table = vec![
            ((7, 5), (6, 5)),
            ((7, 4), (6, 4)),
            ((6, 3), (6, 4)),
            ((5, 3), (5, 4)),
            ((4, 3), (4, 4)),
            ((3, 4), (4, 4)),
            ((3, 5), (4, 5)),
            ((3, 6), (4, 6)),
            ((4, 7), (4, 6)),
            ((5, 7), (5, 6)),
            ((6, 7), (6, 6)),
            ((7, 6), (6, 6)),
        ];

        for (head_pos, tail_exp) in table {
            let mut tail = Point::from(5, 5);
            adjust_tail(&Point::from(head_pos.0, head_pos.1), &mut tail);
            assert_eq!(Point::from(tail_exp.0, tail_exp.1), tail);
        }
    }

    #[test]
    fn head_tail_positions_check_test() {
        let t = Point::from(-10, -10);
        let h = Point::from(-10, -9);
        assert_eq!(true, head_tail_positions_check(&h, &t));

        let t = Point::from(-10, 5);
        let h = Point::from(-9, 4);
        assert_eq!(true, head_tail_positions_check(&h, &t));
    }
}
