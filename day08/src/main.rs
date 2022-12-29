use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut map = vec![];
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let row: Vec<Tree> = line
            .chars()
            .map(|ch| Tree::new(ch.to_digit(10).unwrap() as usize))
            .collect();
        map.push(row);
    }

    let y_max = map.len();
    let x_max = map[0].len();
    let mut visible_trees = 0;

    // left -> right
    for y in 0..y_max {
        for x in 0..x_max {
            match x {
                0 => {}
                _ => {
                    map[y][x].left_dist = clone_and_increase(&map[y][x - 1].left_dist);
                    map[y][x].left_dist[map[y][x - 1].size] = 1;
                }
            };
        }
    }

    // left <- right
    for y in (0..y_max).rev() {
        for x in (0..x_max).rev() {
            if x == x_max - 1 {
            } else {
                map[y][x].right_dist = clone_and_increase(&map[y][x + 1].right_dist);
                map[y][x].right_dist[map[y][x + 1].size] = 1;
            }
        }
    }

    // top -> bottom
    for x in 0..x_max {
        for y in 0..y_max {
            match y {
                0 => {}
                _ => {
                    map[y][x].top_dist = clone_and_increase(&map[y - 1][x].top_dist);
                    map[y][x].top_dist[map[y - 1][x].size] = 1;
                }
            };
        }
    }

    // top <- bottom
    for x in (0..x_max).rev() {
        for y in (0..y_max).rev() {
            if y == y_max - 1 {
            } else {
                map[y][x].bottom_dist = clone_and_increase(&map[y + 1][x].bottom_dist);
                map[y][x].bottom_dist[map[y + 1][x].size] = 1;
            }
        }
    }

    let mut highest_scenic_score = 0;
    let (mut xm, mut ym) = (0, 0);
    for y in 0..y_max {
        for x in 0..x_max {
            if map[y][x].is_visible() {
                visible_trees += 1;
                print!("{}", map[y][x].size);
            } else {
                print!("X");
            }

            let scenic_score = map[y][x].scenic_score();
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
                (xm, ym) = (x, y);
            }
        }
        println!("");
    }

    println!("visible trees: {}", visible_trees);
    println!(
        "highest scenic score: {} [{}][{}]={}; t={},r={},b={},l={}",
        highest_scenic_score,
        xm,
        ym,
        map[ym][xm].size,
        map[ym][xm].top_max_dist,
        map[ym][xm].right_max_dist,
        map[ym][xm].bottom_max_dist,
        map[ym][xm].left_max_dist
    );
    println!("");
    println!(
        " [{}][{}]={}; t={},r={},b={},l={}",
        2,
        3,
        map[3][2].size,
        map[3][2].top_max_dist,
        map[3][2].right_max_dist,
        map[3][2].bottom_max_dist,
        map[3][2].left_max_dist
    );
}

fn clone_and_increase(arr: &[usize; 10]) -> [usize; 10] {
    let mut arr = arr.clone();
    for item in &mut arr {
        if *item == usize::MAX {
            continue;
        }
        *item += 1;
    }
    arr
}

struct Tree {
    size: usize,
    top: bool,
    left: bool,
    right: bool,
    bottom: bool,
    top_dist: [usize; 10],
    right_dist: [usize; 10],
    bottom_dist: [usize; 10],
    left_dist: [usize; 10],
}

impl Tree {
    fn new(size: usize) -> Self {
        Tree {
            size,
            top: false,
            left: false,
            right: false,
            bottom: false,
            top_dist: [usize::MAX; 10],
            right_dist: [usize::MAX; 10],
            bottom_dist: [usize::MAX; 10],
            left_dist: [usize::MAX; 10],
        }
    }

    fn is_visible(&self) -> bool {
        return self.top || self.left || self.right || self.bottom;
    }

    fn scenic_score(&self) -> usize {
        (self.top_max_dist) * (self.right_max_dist) * (self.bottom_max_dist) * (self.left_max_dist)
    }
}
