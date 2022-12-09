use std::collections::HashSet;

fn main() {
    let mut second_visited = HashSet::new();
    let mut tail_visited = HashSet::new();
    
    let mut positions = [(0_isize, 0_isize); 10];
    for line in include_str!("../../../day09.txt").lines() {
        let mut iter = line.split(' ');
        
        let take_step = match iter.next().unwrap() {
            "U" => {
                fn up(head_position: &mut (isize, isize)) {
                    head_position.1 += 1;
                }

                up
            },
            "D" => {
                fn down(head_position: &mut (isize, isize)) {
                    head_position.1 -= 1;
                }

                down
            },
            "L" => {
                fn left(head_position: &mut (isize, isize)) {
                    head_position.0 -= 1;
                }

                left
            },
            "R" => {
                fn right(head_position: &mut (isize, isize)) {
                    head_position.0 += 1;
                }

                right
            },
            _ => unreachable!(),
        };
        
        for _ in 0..iter.next().unwrap().parse::<usize>().unwrap() {
            take_step(&mut positions[0]);
            
            for i in 0..9 {
                match ((positions[i].0 - positions[i + 1].0).signum(), (positions[i].1 - positions[i + 1].1).signum()) {
                    (0, 0) => (),
                    (-1, 0) => if positions[i + 1].0 - positions[i].0 > 1 {
                        positions[i + 1].0 -= 1;
                    },
                    (1, 0) => if positions[i].0 - positions[i + 1].0 > 1 {
                        positions[i + 1].0 += 1;
                    },
                    (0, -1) => if positions[i + 1].1 - positions[i].1 > 1 {
                        positions[i + 1].1 -= 1;
                    },
                    (0, 1) => if positions[i].1 - positions[i + 1].1 > 1 {
                        positions[i + 1].1 += 1;
                    },
                    (a, b) => if positions[i].0.abs_diff(positions[i + 1].0) > 1 || positions[i].1.abs_diff(positions[i + 1].1) > 1 {
                        positions[i + 1] = (positions[i + 1].0 + a, positions[i + 1].1 + b);
                    }
                }
            }
            
            second_visited.insert(positions[1]);
            tail_visited.insert(positions[9]);
        }
    }
    
    let part_1 = second_visited.len();
    let part_2 = tail_visited.len();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
