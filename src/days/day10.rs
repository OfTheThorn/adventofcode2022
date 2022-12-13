const ADD_X: &str = "addx ";
const NOOP: &str = "noop";

pub fn day10() {
    let mut cycle_and_cpu_register: Vec<(i16, i16)> = Vec::new();
    let mut cpu_register: i16 = 1;
    let mut cycle: i16 = 1;
    let mut sum: i16 = 0;
    let mut crt: [char; 240] = ['.'; 240];
    let mut sprite_from_to: (i16, i16) = (0, 2);

    for x in include_str!("../../inputs/day10.txt").lines() {
        if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
            sum += cycle_and_cpu_register.last().unwrap().0 * cycle_and_cpu_register.last().unwrap().1;
        }
        if x.starts_with(&NOOP) {
            cycle += 1;
            cycle_and_cpu_register.push((cycle, cpu_register));
        } else if x.starts_with(&ADD_X) {
            let amount = x.split_whitespace().nth(1).unwrap().parse::<i8>().unwrap();
            if (cycle + 1) == 20 || (cycle + 1) == 60 || (cycle + 1) == 100 || (cycle + 1) == 140 || (cycle + 1) == 180 || (cycle + 1) == 220 {
                sum += (cycle_and_cpu_register.last().unwrap().0 + 1) * cycle_and_cpu_register.last().unwrap().1;
            }
            cycle += 2;
            cpu_register += amount as i16;
            cycle_and_cpu_register.push((cycle, cpu_register));
        }
    };

    for i in 0..240 {
        let t = i % 40;
        for x in &cycle_and_cpu_register {
            if x.0 - 1 == i {
                sprite_from_to = (x.1 - 1, x.1 + 1);
            }
        }

        if t >= sprite_from_to.0 && t <= sprite_from_to.1 {
            crt[i as usize] = '#';
        }

    };

    let mut counter = 0;
    let mut i = 0;
    while i <= 40 {
        if i % 40 == 0 && i > 0 {
            counter += 1;
            println!();
            i = 0;
        }
        if i + (counter * 40) < 240 {
            print!("{}", crt[i + (counter * 40)]);
        }
        if counter == 6 {
            break;
        }
        i += 1;
    }
}

