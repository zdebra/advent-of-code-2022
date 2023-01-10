fn main() {
    let mut monkeys = vec![
        Monkey {
            items: vec![75, 75, 98, 97, 79, 97, 64],
            operation: Box::new(|old| old * 13),
            test: Box::new(|worry_level| if worry_level % 19 == 0 { 2 } else { 7 }),
            test_val: 19,
        },
        Monkey {
            items: vec![50, 99, 80, 84, 65, 95],
            operation: Box::new(|old| old + 2),
            test: Box::new(|worry_level| if worry_level % 3 == 0 { 4 } else { 5 }),
            test_val: 3,
        },
        Monkey {
            items: vec![96, 74, 68, 96, 56, 71, 75, 53],
            operation: Box::new(|old| old + 1),
            test: Box::new(|worry_level| if worry_level % 11 == 0 { 7 } else { 3 }),
            test_val: 11,
        },
        Monkey {
            items: vec![83, 96, 86, 58, 92],
            operation: Box::new(|old| old + 8),
            test: Box::new(|worry_level| if worry_level % 17 == 0 { 6 } else { 1 }),
            test_val: 17,
        },
        Monkey {
            items: vec![99],
            operation: Box::new(|old| old * old),
            test: Box::new(|worry_level| if worry_level % 5 == 0 { 0 } else { 5 }),
            test_val: 5,
        },
        Monkey {
            items: vec![60, 54, 83],
            operation: Box::new(|old| old + 4),
            test: Box::new(|worry_level| if worry_level % 2 == 0 { 2 } else { 0 }),
            test_val: 2,
        },
        Monkey {
            items: vec![77, 67],
            operation: Box::new(|old| old * 17),
            test: Box::new(|worry_level| if worry_level % 13 == 0 { 4 } else { 1 }),
            test_val: 13,
        },
        Monkey {
            items: vec![95, 65, 58, 76],
            operation: Box::new(|old| old + 5),
            test: Box::new(|worry_level| if worry_level % 7 == 0 { 3 } else { 6 }),
            test_val: 7,
        },
    ];

    // let mut monkeys = vec![
    //     Monkey {
    //         items: vec![79, 98],
    //         operation: Box::new(|old| return old * 19),
    //         test: Box::new(|worry_level| if worry_level % 23 == 0 { 2 } else { 3 }),
    //         test_val: 23,
    //     },
    //     Monkey {
    //         items: vec![54, 65, 75, 74],
    //         operation: Box::new(|old| old + 6),
    //         test: Box::new(|worry_level| if worry_level % 19 == 0 { 2 } else { 0 }),
    //         test_val: 19,
    //     },
    //     Monkey {
    //         items: vec![79, 60, 97],
    //         operation: Box::new(|old| old * old),
    //         test: Box::new(|worry_level| if worry_level % 13 == 0 { 1 } else { 3 }),
    //         test_val: 13,
    //     },
    //     Monkey {
    //         items: vec![74],
    //         operation: Box::new(|old| old + 3),
    //         test: Box::new(|worry_level| if worry_level % 17 == 0 { 0 } else { 1 }),
    //         test_val: 17,
    //     },
    // ];

    let mut inspection_counter = vec![0; monkeys.len()];

    let factor = monkeys.iter().map(|x| x.test_val).product::<u64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let mut push_updates = vec![vec![]; monkeys.len()];
            for item in &monkey.items {
                inspection_counter[i] += 1;
                let op = &monkey.operation;
                let test = &monkey.test;
                let item_worry_level = {
                    let wl = op(*item);
                    wl % factor
                };
                let next_monkey = test(item_worry_level);
                push_updates[next_monkey as usize].push(item_worry_level);
            }
            monkeys[i].items.clear();
            for j in 0..push_updates.len() {
                for update in &push_updates[j] {
                    monkeys[j].items.push(*update);
                }
            }
        }
    }

    for (i, item) in inspection_counter.iter().enumerate() {
        println!("{i}: {item}");
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> u64>,
    test_val: u64,
}
