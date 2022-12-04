use crate::read_lines;
pub fn day4_part1() {
    let mut count: u16 = 0;
    if let Ok(lines) = read_lines("inputs/day4.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<i32> = ip.split(&[',', '-'][..])
                    .map(|s| s.parse().unwrap())
                    .collect();
                if (split[0] <= split[2] && split[1] >= split[3]) || (split[0] >= split[2] && split[1] <= split[3]) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

pub fn day4_part2() {
    let mut count: u16 = 0;
    if let Ok(lines) = read_lines("inputs/day4.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<i32> = ip.split(&[',', '-'][..])
                    .map(|s| s.parse().unwrap())
                    .collect();
                if (split[0] <= split[3] && split[1] >= split[2]) || (split[0] >= split[3] && split[1] <= split[2]) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}