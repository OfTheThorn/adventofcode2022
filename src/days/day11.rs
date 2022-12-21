use std::borrow::Borrow;
use std::num::ParseIntError;


pub fn day11() {
    let input: Vec<Vec<&str>> = include_str!("../../inputs/day11.txt").lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>()).collect();
    let mut n: usize = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();
    while n <= input.len() {
        monkeys.push(Monkey {
            name: (n / 7) as i8,
            items: input[n + 1][2..].iter().map(|x| x.replace(",", "").parse::<i64>().unwrap()).collect(),
            operation: input[n + 2][3..].iter().map(|x| x.to_string()).collect(),
            test_value: input[n + 3].last().unwrap().parse::<i64>().unwrap(),
            actions: [
                input[n + 4].last().unwrap().parse::<i32>().unwrap() as i8,
                input[n + 5].last().unwrap().parse::<i32>().unwrap() as i8,
            ],
            counter: 0
        });

        n += 7;
    }
    let mut z = 0;
    let monkeys_len = monkeys.len();
    while z < 20 {
        for x in 0..monkeys_len {
            monkeys[x].perform_operation();
            let monkey_items = monkeys[x].items.clone();
            monkeys[x].counter += monkey_items.len();

            for y in monkey_items.iter().rev().copied() {
                if y % monkeys[x].test_value == 0 {
                    let temp = monkeys[x].actions[0] as usize;
                    monkeys[temp].items.push(y);
                } else {
                    let temp = monkeys[x].actions[1] as usize;
                    &monkeys[temp].items.push(y);
                };
                &monkeys[x].items.pop();
            };
        }
        z+=1;
    }
    monkeys.sort_by_key(|x| x.counter);
    println!("{}", monkeys[monkeys_len-1].counter * monkeys[monkeys_len-2].counter);
    dbg!(monkeys[0].counter);
    dbg!(monkeys[1].counter);
    dbg!(monkeys[2].counter);
    dbg!(monkeys[3].counter);
}

#[derive(Debug)]
struct Monkey {
    name: i8,
    items: Vec<i64>,
    test_value: i64,
    operation: Vec<String>,
    actions: [i8; 2],
    counter: usize,
}

impl Monkey {
    fn perform_operation(&mut self) {
        if self.items.len() == 0 {
            return;
        }
        let mut temp:i64 = 0;
        match self.operation.last().unwrap().parse::<i64>() {
            Ok(val) => {
                temp = val
            }
            Err(_) => {
                temp = -1;
            }
        };
        match self.operation[1].as_str() {
            "*" => {
                for mut item in &mut self.items {
                    if temp == -1 {
                        *item = *item * *item;
                    } else {
                        *item = *item * temp;
                    }
                    *item = *item / 3;
                }
            }
            "/" => {
                for mut item in &mut self.items {
                    if temp == -1 {
                        *item = *item / *item;
                    } else {
                        *item = *item / temp;
                    }
                    *item = *item / 3;
                }
            }
            "+" => {
                for mut item in &mut self.items {
                    if temp == -1 {
                        *item = *item + *item;
                    } else {
                        *item = *item + temp;
                    }
                    *item = *item / 3;
                }
            }
            "-" => {
                for mut item in &mut self.items {
                    if temp  == -1 {
                        *item = *item - *item;
                    } else {
                        *item = *item - temp;
                    }
                    *item = *item / 3;
                }
            }
            _ => {}
        }
    }
}