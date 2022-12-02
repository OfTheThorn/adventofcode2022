pub mod days;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    days::day2::day2_part2()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

