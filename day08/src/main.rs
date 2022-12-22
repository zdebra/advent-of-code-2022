use std::cmp;
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
                0 => map[y][x].left = true,
                _ => {
                    map[y][x].left_max = cmp::max(map[y][x - 1].left_max, map[y][x - 1].size);
                    if map[y][x].left_max < map[y][x].size {
                        map[y][x].left = true;
                    }
                }
            };
        }
    }

    // left <- right
    for y in (0..y_max).rev() {
        for x in (0..x_max).rev() {
            if x == x_max - 1 {
                map[y][x].right = true;
            } else {
                map[y][x].right_max = cmp::max(map[y][x + 1].right_max, map[y][x + 1].size);
                if map[y][x].right_max < map[y][x].size {
                    map[y][x].right = true;
                }
            }
        }
    }

    // top -> bottom
    for x in 0..x_max {
        for y in 0..y_max {
            match y {
                0 => {
                    map[y][x].top = true;
                }
                _ => {
                    map[y][x].top_max = cmp::max(map[y - 1][x].top_max, map[y - 1][x].size);
                    if map[y][x].top_max < map[y][x].size {
                        map[y][x].top = true;
                    }
                }
            };
        }
    }

    // top <- bottom
    for x in (0..x_max).rev() {
        for y in (0..y_max).rev() {
            if y == y_max - 1 {
                map[y][x].bottom = true;
            } else {
                map[y][x].bottom_max = cmp::max(map[y + 1][x].bottom_max, map[y + 1][x].size);
                if map[y][x].bottom_max < map[y][x].size {
                    map[y][x].bottom = true;
                }
            }
        }
    }

    for y in 0..y_max {
        for x in 0..x_max {
            if map[y][x].is_visible() {
                visible_trees += 1;
                print!("{}", map[y][x].size);
            } else {
                print!("X");
            }
        }
        println!("");
    }

    // println!("");
    // println!("top_max");
    // for y in 0..y_max {
    //     for x in 0..x_max {
    //         if map[y][x].top {
    //             print!("T");
    //         } else {
    //             print!("F");
    //         }
    //     }
    //     println!("");
    // }

    // println!("");
    // println!("right_max");
    // for y in 0..y_max {
    //     for x in 0..x_max {
    //         if map[y][x].right {
    //             print!("T");
    //         } else {
    //             print!("F");
    //         }
    //     }
    //     println!("");
    // }

    // println!("");
    // println!("left_max");
    // for y in 0..y_max {
    //     for x in 0..x_max {
    //         if map[y][x].left {
    //             print!("T");
    //         } else {
    //             print!("F");
    //         }
    //     }
    //     println!("");
    // }

    // println!("");
    // println!("bottom_max");
    // for y in 0..y_max {
    //     for x in 0..x_max {
    //         if map[y][x].bottom {
    //             print!("T");
    //         } else {
    //             print!("F");
    //         }
    //     }
    //     println!("");
    // }

    println!("visible trees: {}", visible_trees);
}

struct Tree {
    size: usize,
    top: bool,
    left: bool,
    right: bool,
    bottom: bool,
    top_max: usize,
    left_max: usize,
    right_max: usize,
    bottom_max: usize,
}

impl Tree {
    fn new(size: usize) -> Self {
        Tree {
            size,
            top: false,
            left: false,
            right: false,
            bottom: false,
            top_max: 0,
            left_max: 0,
            right_max: 0,
            bottom_max: 0,
        }
    }

    fn is_visible(&self) -> bool {
        return self.top || self.left || self.right || self.bottom;
    }
}
