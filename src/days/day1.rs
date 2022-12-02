use crate::read_lines;

pub fn day1() {
    let mut maxes: [u32; 3] = [0; 3];
    let mut current: u32 = 0;

    if let Ok(lines) = read_lines("inputs/day1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                match ip.trim().parse::<u32>() {
                    Ok(val) => {
                        current = current + val;
                    }
                    Err(..) => {
                        // Error means newline
                        if let Some(index_of_min) = maxes
                            .iter()
                            .enumerate()
                            .min_by(|(_, a), (_, b)| a.cmp(b))
                            .map(|(index, _)| index) {
                            if maxes[index_of_min] < current {
                                maxes[index_of_min] = current;
                            }
                        }
                        current = 0;
                    }
                }
            }
        }
    }
    println!("{}", maxes.iter().sum::<u32>());
}