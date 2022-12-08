pub fn day8() {
    let grid = include_str!("../../inputs/day8.txt").lines()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i8).collect::<Vec<i8>>())
        .collect::<Vec<Vec<i8>>>();
    let x_len = grid[0].len();
    let y_len = grid.len();

    // part 1
    let mut total: usize = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            let height = grid[y][x];
            if (0..y).all(|i| grid[i][x] < height) || (y + 1..y_len).all(|i| grid[i][x] < height) || (0..x).all(|i| grid[y][i] < height) || (x + 1..y_len).all(|i| grid[y][i] < height)
            {
                total += 1;
            }
        }
    }
    dbg!(total);

    // part 2
    // I started regretting the naming at this point
    let mut resp: usize = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            let height = grid[y][x];

            let north = (0..y).rev().find(|&i| grid[i][x] >= height).unwrap_or(0);
            let south = (y + 1..y_len).find(|&i| grid[i][x] >= height).unwrap_or(x_len - 1);
            let east = (x + 1..x_len).find(|&i| grid[y][i] >= height).unwrap_or(y_len - 1);
            let west = (0..x).rev().find(|&i| grid[y][i] >= height).unwrap_or(0);

            let temp = (y - north) * (south - y) * (x - west) * (east - x);
            if temp > resp {
                resp = temp;
            }
        }
    }
    dbg!(resp);
}
