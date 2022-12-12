use std::collections::VecDeque;

pub fn day9() -> usize {
    let mut position_head: (i16, i16) = (0, 0);
    let mut position_tail: (i16, i16) = (0, 0);
    let mut tail_history: Vec<(i16, i16)> = Vec::new();
    let mut current_rope: VecDeque<(i16, i16)> = VecDeque::new();
    let mut counter: i32 = 0;
    let _ = include_str!("../../inputs/day9.txt").lines().for_each(|x| move_head_and_tail(x, &mut position_head, &mut position_tail, &mut tail_history, &mut current_rope));
    dbg!(tail_history.len());
    return tail_history.len() + 1;
}

fn move_head_and_tail(input: &str, mut position_head: &mut (i16, i16), mut position_tail: &mut (i16, i16), mut tail_history: &mut Vec<(i16, i16)>, mut current_rope: &mut VecDeque<(i16, i16)>) {
    let t = input.split_whitespace().collect::<Vec<&str>>();
    let amount = t[1].parse::<i16>().unwrap();

    for _ in 0..amount {
        match t[0] {
            "U" => {
                position_head.1 += 1;
            }
            "D" => {
                position_head.1 += -1;
            }
            "R" => {
                position_head.0 += 1;
            }
            "L" => {
                position_head.0 += -1;
            }
            _ => ()
        };

        let diff_x = position_head.0.abs_diff(position_tail.0);
        let diff_y = position_head.1.abs_diff(position_tail.1);

        if diff_x <= 1 && diff_y <= 1 {
            continue;
        }

        if position_head.0 < position_tail.0 {
            position_tail.0 -= 1;
        } else if position_head.0 > position_tail.0 {
            position_tail.0 += 1;
        }

        if position_head.1 < position_tail.1 {
            position_tail.1 -= 1;
        } else if position_head.1 > position_tail.1 {
            position_tail.1 += 1;
        }


        if let None = tail_history.iter().find(|&&x| x == *position_tail) {
            tail_history.push(*position_tail);
        }
    }
}


//(-2  2)  (-1  2)   (0  2)   (1  2)   (2  2)
//(-2  1)  (-1  1)   (0  1)   (1  1)   (2  1)
//(-2  0)  (-1  0)   (0  0)   (1  0)   (2  0)
//(-2 -1)  (-1 -1)   (0 -1)   (1 -1)   (2 -1)
//(-2 -2)  (-1 -2)   (0 -2)   (1 -2)   (2 -2)

#[test]
fn test_part1() {
    assert_eq!(36, day9());
}

