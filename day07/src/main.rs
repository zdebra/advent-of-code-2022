use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut s = Status::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        println!("{}", line);
        s.parse_line(&line);
    }

    s.items.sort_by(|a, b| {
        let (a, b) = (a.name(), b.name());
        if a < b {
            Ordering::Less
        } else if a == b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    println!("");
    for item in &s.items {
        println!("{}", item.name());
    }
    println!("");

    let mut dirs = HashMap::new();
    for item in &s.items {
        match item {
            Item::Directory { name: n } => {
                dirs.insert(n, 0);
            }
            Item::File {
                name: file_name,
                size: file_size,
            } => {
                for (dir_name, dir_size) in &mut dirs {
                    if file_name.starts_with(*dir_name) {
                        *dir_size += file_size;
                    }
                }
            }
        };
    }

    let mut total = 0;
    for (_dir, size) in dirs {
        // println!("{} {}", dir, size);
        if size <= 100000 {
            total += size;
        }
    }
    println!("{total}");
}

struct Status {
    cur_dir: String,
    mode: Mode,
    items: Vec<Item>,
}

impl Status {
    fn new() -> Self {
        Self {
            cur_dir: "/".to_string(),
            mode: Mode::ReadClose,
            items: vec![],
        }
    }

    fn parse_line(&mut self, line: &String) {
        match line {
            l if l.starts_with("$") => self.command(line),
            _ => self.non_command(line),
        }
    }

    fn command(&mut self, cmd_str: &String) {
        self.mode = Mode::ReadClose;
        match cmd_str {
            x if x.starts_with("$ cd") => self.cmd_cd(cmd_str),
            x if x.starts_with("$ ls") => self.cmd_ls(cmd_str),
            _ => panic!("unexpected cmd input {}", cmd_str),
        }
    }

    fn non_command(&mut self, line: &String) {
        if self.mode != Mode::ReadOpen {
            panic!("unexpected mode")
        }
        match line {
            dir if dir.starts_with("dir ") => {
                let dir = line.strip_prefix("dir ").unwrap();
                self.items.push(Item::Directory {
                    name: self.cur_dir.to_owned() + dir + "/",
                });
            }
            _ => {
                let (file_size, file_name) = {
                    let split: Vec<&str> = line.split_whitespace().collect();
                    assert_eq!(2, split.len());
                    (split[0], split[1])
                };
                self.items.push(Item::File {
                    name: self.cur_dir.to_owned() + file_name,
                    size: file_size.parse().unwrap(),
                });
            }
        }
    }

    // cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
    //      cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
    //      cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
    //      cd / switches the current directory to the outermost directory, /.
    fn cmd_cd(&mut self, cmd_str: &String) {
        let arg = cmd_str.strip_prefix("$ cd ").expect("expect `$ cd `");
        match arg {
            "/" => self.cur_dir = "/".to_string(),
            ".." => {
                let splits: Vec<&str> = self.cur_dir.split("/").collect();
                self.cur_dir = splits[0..splits.len() - 2].join("/") + "/";
            }
            _ => self.cur_dir = format!("{}{}/", self.cur_dir, arg),
        }
        println!("changed cur dir to {}", self.cur_dir);
    }

    fn cmd_ls(&mut self, _: &String) {
        self.mode = Mode::ReadOpen;
    }
}

enum Item {
    File { name: String, size: usize },
    Directory { name: String },
}

impl Item {
    fn name(&self) -> &String {
        match self {
            Item::File { name: n, size: _s } => n,
            Item::Directory { name: n } => n,
        }
    }
}

#[derive(PartialEq)]
enum Mode {
    ReadClose,
    ReadOpen,
}
