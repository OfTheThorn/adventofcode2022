pub mod days;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let result = days::day6::day6(14);
    dbg!(result);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

