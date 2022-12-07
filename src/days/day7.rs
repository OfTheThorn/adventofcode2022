pub fn day7() {
    let input = include_str!("../../inputs/day7.txt");
    let mut current_pos = String::new();
    let mut file_tree: Vec<(String, u64)> = Vec::new();
    file_tree.push((format!("{a}", a = "/"), 0u64));
    for x in input.lines().skip(1) {
        if x.starts_with("$ cd /") {
            current_pos = "/".to_string();
            continue;
        }
        if x.starts_with("$ cd") && !x.ends_with("..") {
            current_pos.push_str(&*format!("/{a}", a = &x[5..]));
            continue;
        }
        if x.starts_with("$ cd ..") {
            if let Some(pos) = current_pos.rfind("/") {
                let _ = current_pos.split_off(pos);
            };
        }
        if x.starts_with("dir") {
            if file_tree.iter().find(|(s, _)| s.eq(&x[4..])).is_none() {
                file_tree.push((format!("{a}/{b}", a = current_pos, b = &x[4..]), 0u64));
                continue;
            }
        }
        if x.as_bytes()[0].is_ascii_alphanumeric() {
            let value = x.split_whitespace().next().unwrap().parse::<u64>().unwrap();
            file_tree[0].1 += value;
            for (s, i) in file_tree.iter_mut() {
                if s == &String::from(&current_pos) {
                    *i += value;
                };
            };
            let mut temp = current_pos.to_string();
            while temp.len() > 0 {
                match temp.rfind("/") {
                    None => { break; }
                    Some(v) => {
                        temp = temp[..v].parse().unwrap();
                        for (s2, i2) in file_tree.iter_mut() {
                            if s2 == &temp {
                                *i2 += value;
                            }
                        }
                    }
                };
            };
        }
    }
    // part 1
    let mut total: u64 = 0;
    for f in &file_tree {
        if f.1.le(&100000u64) {
            total += f.1;
        }
    }
    println!("Part 1:{}", total);
    // part 2
    let to_be_freed_up_amount = 30000000 - (70000000 - &file_tree[0].1);
    let mut part2: Vec<u64> = Vec::new();
    for val in &file_tree {
        if val.1 >= to_be_freed_up_amount {
            part2.push(val.1);
        }
    };
    part2.sort();
    println!("Part 2:{}", part2[0]);


}
