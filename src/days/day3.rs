use crate::read_lines;

pub fn day3_part1() {
    let mut total: u64 = 0;
    if let Ok(lines) = read_lines("inputs/day3.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let len = ip.len() / 2;
                let temp: [&str; 2] = [&ip[..len], &ip[len..]];
                let char_found = temp[0].as_bytes().iter().find(|&&x| temp[1].as_bytes().contains(&x));
                if let Some(x) = char_found {
                    if x.is_ascii_uppercase() {
                        total += u64::from(*x) - 38
                    } else {
                        total += u64::from(*x) - 96
                    }
                }
            }
        }
    }
    println!("{:?}", total);
}

pub fn day3_part2() {
    let mut total: u64 = 0;
    let mut lines = include_str!("../../inputs/day3.txt").lines().peekable();
    while lines.peek().is_some() {
        let mut input_slice: [&str; 3] = ["";3];
        input_slice[0] = lines.next().unwrap();
        input_slice[1] = lines.next().unwrap();
        input_slice[2] = lines.next().unwrap();
        let char_found = input_slice[0].as_bytes().iter().find(|&&x| input_slice[1].as_bytes().contains(&x) && input_slice[2].as_bytes().contains(&x)).unwrap();
        if char_found.is_ascii_uppercase() {
            total += u64::from(*char_found) - 38
        } else {
            total += u64::from(*char_found) - 96
        }
    }
    println!("{:?}", total);
}