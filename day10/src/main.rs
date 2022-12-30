use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut device = Device::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        match line {
            ins if ins.starts_with("addx") => {
                let (_, v) = ins.split_once(" ").unwrap();
                device.push_instruction(Instruction::Addx(v.parse().unwrap()))
            }
            ins if ins.starts_with("noop") => device.push_instruction(Instruction::Noop),
            _ => unreachable!(),
        }
    }

    device.run();
}

enum Instruction {
    Addx(isize),
    Noop,
}

struct Device {
    x_before: isize,
    x_after: isize,
    stack: VecDeque<Box<dyn Fn(isize) -> isize>>,
    cur_cycle: usize,
}

impl Device {
    fn new() -> Self {
        Self {
            x_before: 1,
            x_after: 1,
            stack: VecDeque::new(),
            cur_cycle: 0,
        }
    }

    fn push_instruction(&mut self, ins: Instruction) {
        match ins {
            Instruction::Addx(val) => {
                self.stack.push_back(Box::new(|x| x));
                self.stack.push_back(Box::new(move |x| x + val));
            }
            Instruction::Noop => {
                self.stack.push_back(Box::new(|x| x));
            }
        }
    }

    fn advance_cycle(&mut self) -> Result<(), ()> {
        let action = match self.stack.pop_front() {
            None => return Err(()),
            Some(a) => a,
        };
        self.x_before = self.x_after;
        self.x_after = action(self.x_before);
        self.cur_cycle += 1;
        return Ok(());
    }

    fn run(&mut self) {
        loop {
            let sprite = vec![self.x_after - 1, self.x_after, self.x_after + 1];
            let mut to_print = if sprite.contains(&((self.cur_cycle % 40) as isize)) {
                "#".to_string()
            } else {
                ".".to_string()
            };

            if self.cur_cycle % 40 == 39 {
                to_print += "\n";
            }

            if self.advance_cycle().is_err() {
                break;
            }
            print!("{to_print}");
        }
    }
}
