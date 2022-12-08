use std::ops::Add;

pub fn day5_part1() {
    let input = include_str!("../../inputs/day5.txt");
    // Find crates
    let pos = input.lines().position(|x| x.as_bytes().len() == 0).unwrap();
    let mut row1: Vec<char> = Vec::new();
    let mut row2: Vec<char> = Vec::new();
    let mut row3: Vec<char> = Vec::new();
    let mut row4: Vec<char> = Vec::new();
    let mut row5: Vec<char> = Vec::new();
    let mut row6: Vec<char> = Vec::new();
    let mut row7: Vec<char> = Vec::new();
    let mut row8: Vec<char> = Vec::new();
    let mut row9: Vec<char> = Vec::new();
    input.lines()
        .take(pos - 1)
        .map(|x| x.chars())
        .map(|x| x.enumerate().filter(|(i, y)| y.is_ascii_uppercase()).collect::<Vec<(usize, char)>>())
        .map(|x| x.iter().map(|y| (y.0 / 4 + 1, y.1)).collect::<Vec<(usize, char)>>())
        .for_each(|x| x.iter().for_each(
            |y: &(usize, char)| match y.0 {
                1 => row1.push(y.1),
                2 => row2.push(y.1),
                3 => row3.push(y.1),
                4 => row4.push(y.1),
                5 => row5.push(y.1),
                6 => row6.push(y.1),
                7 => row7.push(y.1),
                8 => row8.push(y.1),
                9 => row9.push(y.1),
                _ => ()
            }
        ));
    let crates_commands = input
        .lines()
        .skip(pos + 1)
        .map(|x| x.bytes())
        .map(|x| x.into_iter().filter(|y| y.is_ascii_digit()).collect::<Vec<u8>>())
        .map(|x| x.iter().map(|y| (y - 48).to_string()).collect::<String>())
        .collect::<Vec<String>>();
    // Again, fed up...
    /*
    For part 1
    row1.reverse();
    row2.reverse();
    row3.reverse();
    row4.reverse();
    row5.reverse();
    row6.reverse();
    row7.reverse();
    row8.reverse();
    row9.reverse();


     */
    let mut grouped: Vec<&mut Vec<char>> = Vec::new();

    grouped.push(&mut row1);
    grouped.push(&mut row2);
    grouped.push(&mut row3);
    grouped.push(&mut row4);
    grouped.push(&mut row5);
    grouped.push(&mut row6);
    grouped.push(&mut row7);
    grouped.push(&mut row8);
    grouped.push(&mut row9);
    for crates_command in crates_commands {
        let mut move_number: usize = 0;
        let mut from_number: usize = 0;
        let mut to_number: usize = 0;
        if crates_command.len() > 3 {
            move_number = crates_command.as_str()[..2].parse::<usize>().unwrap();
            from_number = crates_command.chars().nth(2).unwrap().to_digit(10).unwrap() as usize;
            to_number = crates_command.chars().nth(3).unwrap().to_digit(10).unwrap() as usize;
        } else {
            move_number = crates_command.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
            from_number = crates_command.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
            to_number = crates_command.chars().nth(2).unwrap().to_digit(10).unwrap() as usize;
        }
        // part 1
        /*
        for i in 0..move_number {
            if let Some(&ok) = grouped[from_number - 1].last() {
                grouped[to_number-1].push(ok);
                grouped[from_number - 1].pop();
            }
        }
        */
        // part 2
        if move_number < grouped[from_number - 1].len() {
            let mut temp = grouped[from_number - 1][..move_number].to_vec();
            grouped[to_number - 1].splice(..0, temp);
            let _ = grouped[from_number - 1].drain(..move_number);
        } else {
            let mut temp = grouped[from_number - 1].to_vec();
            grouped[to_number - 1].splice(..0,temp);
            grouped[from_number - 1].clear();
        }

    }
    grouped.iter().for_each(|x| if let Some(ok) = x.first() { print!("{:?}", ok) });
}