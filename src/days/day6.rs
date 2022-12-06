
pub fn day6(required_length: usize) -> usize {
    let input = include_str!("../../inputs/day6.txt");
    let mut existing_row: Vec<char> = Vec::new();
    for (i, x) in input.chars().enumerate() {
        if existing_row.len() == required_length {
            return i;
        }
        if let Some(pos) = existing_row.iter().position(|&y| y == x ) {
            existing_row.drain(0..=pos);
        }
        existing_row.push(x);
    }
    return 0;

}