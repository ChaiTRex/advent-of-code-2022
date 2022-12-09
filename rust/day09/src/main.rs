use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../day09.txt");

    let mut head_position = (0_isize, 0_isize);
    let mut tail_position = (0, 0);

    let mut tail_visited = HashSet::new();
    for line in input.lines() {
        let mut iter = line.split(' ');
        let direction = iter.next().unwrap();
        let distance = iter.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..distance {
            match direction {
                "U" => {
                    let (x, y) = head_position;
                    head_position = (x, y + 1);
                }
                "D" => {
                    let (x, y) = head_position;
                    head_position = (x, y - 1);
                }
                "L" => {
                    let (x, y) = head_position;
                    head_position = (x - 1, y);
                }
                "R" => {
                    let (x, y) = head_position;
                    head_position = (x + 1, y);
                }
                _ => unreachable!(),
            }

            match (
                (head_position.0 - tail_position.0).signum(),
                (head_position.1 - tail_position.1).signum(),
            ) {
                (0, 0) => (),
                (-1, 0) => {
                    if head_position.0.abs_diff(tail_position.0) > 1 {
                        tail_position.0 -= 1;
                    }
                }
                (1, 0) => {
                    if head_position.0.abs_diff(tail_position.0) > 1 {
                        tail_position.0 += 1;
                    }
                }
                (0, -1) => {
                    if head_position.1.abs_diff(tail_position.1) > 1 {
                        tail_position.1 -= 1;
                    }
                }
                (0, 1) => {
                    if head_position.1.abs_diff(tail_position.1) > 1 {
                        tail_position.1 += 1;
                    }
                }
                (a, b) => {
                    if head_position.0.abs_diff(tail_position.0) > 1
                        || head_position.1.abs_diff(tail_position.1) > 1
                    {
                        tail_position = (tail_position.0 + a, tail_position.1 + b);
                    }
                }
            }

            tail_visited.insert(tail_position);
        }
    }

    let part_1 = tail_visited.len();

    let mut positions = [(0_isize, 0_isize); 10];

    let mut tail_visited = HashSet::new();
    for line in input.lines() {
        let mut iter = line.split(' ');
        let direction = iter.next().unwrap();
        let distance = iter.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..distance {
            match direction {
                "U" => {
                    let (x, y) = positions[0];
                    positions[0] = (x, y + 1);
                }
                "D" => {
                    let (x, y) = positions[0];
                    positions[0] = (x, y - 1);
                }
                "L" => {
                    let (x, y) = positions[0];
                    positions[0] = (x - 1, y);
                }
                "R" => {
                    let (x, y) = positions[0];
                    positions[0] = (x + 1, y);
                }
                _ => unreachable!(),
            }

            for i in 0..9 {
                match (
                    (positions[i].0 - positions[i + 1].0).signum(),
                    (positions[i].1 - positions[i + 1].1).signum(),
                ) {
                    (0, 0) => (),
                    (-1, 0) => {
                        if positions[i].0.abs_diff(positions[i + 1].0) > 1 {
                            positions[i + 1].0 -= 1;
                        }
                    }
                    (1, 0) => {
                        if positions[i].0.abs_diff(positions[i + 1].0) > 1 {
                            positions[i + 1].0 += 1;
                        }
                    }
                    (0, -1) => {
                        if positions[i].1.abs_diff(positions[i + 1].1) > 1 {
                            positions[i + 1].1 -= 1;
                        }
                    }
                    (0, 1) => {
                        if positions[i].1.abs_diff(positions[i + 1].1) > 1 {
                            positions[i + 1].1 += 1;
                        }
                    }
                    (a, b) => {
                        if positions[i].0.abs_diff(positions[i + 1].0) > 1
                            || positions[i].1.abs_diff(positions[i + 1].1) > 1
                        {
                            positions[i + 1] = (positions[i + 1].0 + a, positions[i + 1].1 + b);
                        }
                    }
                }
            }
            tail_visited.insert(positions[9]);
        }
    }
    let part_2 = tail_visited.len();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
