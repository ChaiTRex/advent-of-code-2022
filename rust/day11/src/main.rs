use core::cell::RefCell;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}

#[derive(Debug)]
enum Operation {
    Plus(u64),
    Times(u64),
    Square,
}

impl Operation {
    fn with(&self, old: u64) -> u64 {
        match self {
            Self::Plus(x) => old + x,
            Self::Times(x) => old * x,
            Self::Square => old * old,
        }
    }
}

impl core::fmt::Display for Operation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Plus(x) => write!(f, "(+ {x})"),
            Self::Times(x) => write!(f, "(* {x})"),
            Self::Square => write!(f, "(^ 2)"),
        }
    }    
}

fn main() {
    let monkeys = include_str!("../../../day11.txt")
        .split("\n\n")
        .map(|monkey_info| {
            let mut lines = monkey_info.lines();
            let items = lines.nth(1).unwrap()[18..]
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let operation = match lines.next().unwrap()[23..].split_once(' ').unwrap() {
                ("+", "old") => Operation::Times(2),
                ("*", "old") => Operation::Square,
                ("+", number) => {
                    let number = number.parse::<u64>().unwrap();
                    Operation::Plus(number)
                }
                ("*", number) => {
                    let number = number.parse::<u64>().unwrap();
                    Operation::Times(number)
                }
                _ => unreachable!(),
            };
            let divisor = lines.next().unwrap()[21..].parse::<u64>().unwrap();
            let true_monkey = lines.next().unwrap()[29..].parse::<usize>().unwrap();
            let false_monkey = lines.next().unwrap()[30..].parse::<usize>().unwrap();

            RefCell::new(Monkey {
                items,
                operation,
                divisor,
                true_monkey,
                false_monkey,
                inspection_count: 0,
            })
        })
        .collect::<Vec<_>>();

    for _round in 0..20 {
        for monkey in monkeys.iter() {
            let items = {
                let mut monkey = monkey.borrow_mut();
                let items = core::mem::take(&mut monkey.items);
                monkey.inspection_count += items.len();
                items
            };

            let monkey = monkey.borrow();
            for mut worry_level in items.into_iter() {
                print!("Starting worry level is {worry_level}. ");
                worry_level = monkey.operation.with(worry_level) / 3;
                print!("(flip div 3) . {} produces {worry_level}, which is ", monkey.operation);
                let destination_monkey = if worry_level % monkey.divisor == 0 {
                    println!(
                        "divisible by {}, so it is passed to monkey {}.",
                        monkey.divisor,
                        monkey.true_monkey,
                    );
                    monkey.true_monkey
                } else {
                    println!(
                        "not divisible by {}, so it is passed to monkey {}.",
                        monkey.divisor,
                        monkey.false_monkey,
                    );
                    monkey.false_monkey
                };
                monkeys[destination_monkey]
                    .borrow_mut()
                    .items
                    .push(worry_level);
            }
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspection_count)
        .collect::<Vec<_>>();
    inspection_counts.sort_unstable();

    let part_1 = inspection_counts[inspection_counts.len() - 2..]
        .iter()
        .product::<usize>();
    
    let monkeys = include_str!("../../../day11.txt")
        .split("\n\n")
        .map(|monkey_info| {
            let mut lines = monkey_info.lines();
            let items = lines.nth(1).unwrap()[18..]
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let operation = match lines.next().unwrap()[23..].split_once(' ').unwrap() {
                ("+", "old") => Operation::Times(2),
                ("*", "old") => Operation::Square,
                ("+", number) => {
                    let number = number.parse::<u64>().unwrap();
                    Operation::Plus(number)
                }
                ("*", number) => {
                    let number = number.parse::<u64>().unwrap();
                    Operation::Times(number)
                }
                _ => unreachable!(),
            };
            let divisor = lines.next().unwrap()[21..].parse::<u64>().unwrap();
            let true_monkey = lines.next().unwrap()[29..].parse::<usize>().unwrap();
            let false_monkey = lines.next().unwrap()[30..].parse::<usize>().unwrap();

            RefCell::new(Monkey {
                items,
                operation,
                divisor,
                true_monkey,
                false_monkey,
                inspection_count: 0,
            })
        })
        .collect::<Vec<_>>();
    
    let product = monkeys
        .iter()
        .map(|monkey| monkey.borrow().divisor)
        .product::<u64>();
    
    let gcd = monkeys
        .iter()
        .map(|monkey| monkey.borrow().divisor)
        .fold(0, |mut a, mut b| {
            loop {
                if a == 0 {
                    break b;
                }
                b %= a;
                
                if b == 0 {
                    break a;
                }
                a %= b;
            }
        });
    
    let lcm = product/gcd;
    
    for _round in 0..10_000 {
        for monkey in monkeys.iter() {
            let items = {
                let mut monkey = monkey.borrow_mut();
                let items = core::mem::take(&mut monkey.items);
                monkey.inspection_count += items.len();
                items
            };

            let monkey = monkey.borrow();
            for mut worry_level in items.into_iter() {
                print!("Starting worry level is {worry_level}. ");
                worry_level = monkey.operation.with(worry_level) % lcm;
                print!("(flip mod lcm) . {} produces {worry_level}, which is ", monkey.operation);
                let destination_monkey = if worry_level % monkey.divisor == 0 {
                    println!(
                        "divisible by {}, so it is passed to monkey {}.",
                        monkey.divisor,
                        monkey.true_monkey,
                    );
                    monkey.true_monkey
                } else {
                    println!(
                        "not divisible by {}, so it is passed to monkey {}.",
                        monkey.divisor,
                        monkey.false_monkey,
                    );
                    monkey.false_monkey
                };
                monkeys[destination_monkey]
                    .borrow_mut()
                    .items
                    .push(worry_level);
            }
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspection_count)
        .collect::<Vec<_>>();
    inspection_counts.sort_unstable();

    let part_2 = inspection_counts[inspection_counts.len() - 2..]
        .iter()
        .product::<usize>();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
