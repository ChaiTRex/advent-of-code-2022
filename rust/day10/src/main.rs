fn main() {
    let mut part_1 = 0;
    let mut part_2 = vec![vec![' '; 40]; 6];
    
    let mut row = 0_i16;
    let mut column = 0_i16;
    
    let mut cycle = 1;
    let mut x = 1_i16;
    for line in include_str!("../../../day10.txt").lines() {
        match &line[0..4] {
            "addx" => {
                cycle += 1;
                if cycle % 40 == 20 && cycle <= 220 {
                    part_1 += cycle * x;
                }
                if x.abs_diff(column) <= 1 {
                    part_2[row as usize][column as usize] = '█';
                }
                column += 1;
                if column == 40 {
                    row += 1;
                    column = 0;
                }

                cycle += 1;
                if cycle % 40 == 20 && cycle <= 220 {
                    part_1 += cycle * x;
                }
                if x.abs_diff(column) <= 1 {
                    part_2[row as usize][column as usize] = '█';
                }
                column += 1;
                if column == 40 {
                    row += 1;
                    column = 0;
                }
                
                x += &line[5..].parse::<i16>().unwrap();
            },
            "noop" => {
                cycle += 1;
                if cycle % 40 == 20 && cycle <= 220 {
                    part_1 += cycle * x;
                }
                if x.abs_diff(column) <= 1 {
                    part_2[row as usize][column as usize] = '█';
                }
                column += 1;
                if column == 40 {
                    row += 1;
                    column = 0;
                }
            }
            _ => unreachable!(),
        }
    }
    let part_2 = part_2
        .iter()
        .map(|row| row.iter().copied().map(|ch| ch as char).chain(core::iter::once('\n')).collect::<String>())
        .collect::<String>();
    
    println!("Part 1: {part_1}");
    println!("Part 2:\n{part_2}");
}
