use crate::read_lines;
pub fn day2() {
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines("inputs/day2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split_result: Vec<&str> = ip.split_whitespace().collect();
                match split_result[0] {
                    "A" => {
                        match split_result[1] {
                            "X" => {
                                total += 4
                            }
                            "Y" => {
                                total += 8
                            }
                            "Z" => {
                                total += 3
                            }
                            _ => {}
                        }
                    }
                    "B" => {
                        match split_result[1] {
                            "X" => {
                                total += 1
                            }
                            "Y" => {
                                total += 5
                            }
                            "Z" => {
                                total += 9
                            }
                            _ => {}
                        }
                    }
                    "C" => {
                        match split_result[1] {
                            "X" => {
                                total += 7
                            }
                            "Y" => {
                                total += 2
                            }
                            "Z" => {
                                total += 6
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{}", total)
}

pub fn day2_part2() {
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines("inputs/day2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split_result: Vec<&str> = ip.split_whitespace().collect();
                match split_result[0] {
                    "A" => {
                        match split_result[1] {
                            "X" => {
                                total += 0 + 3
                            }
                            "Y" => {
                                total += 3 + 1
                            }
                            "Z" => {
                                total += 6 + 2
                            }
                            _ => {}
                        }
                    }
                    "B" => {
                        match split_result[1] {
                            "X" => {
                                total += 0 + 1
                            }
                            "Y" => {
                                total += 3 + 2
                            }
                            "Z" => {
                                total += 6 + 3
                            }
                            _ => {}
                        }
                    }
                    "C" => {
                        match split_result[1] {
                            "X" => {
                                total += 0 + 2
                            }
                            "Y" => {
                                total += 3 + 3
                            }
                            "Z" => {
                                total += 6 + 1
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{}", total)
}
