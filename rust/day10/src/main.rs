fn main() {
    let mut part_1 = 0;

    let mut row = 0_i16;
    let mut column = 0_i16;

    let mut cycle = 1;
    let mut x = 1_i16;

    let part_2 = include_str!("../../../day10.txt")
        .lines()
        .flat_map(|line| match &line[0..4] {
            "addx" => [Some(0), Some(line[5..].parse::<i16>().unwrap())],
            "noop" => [Some(0), None],
            _ => unreachable!(),
        })
        .flatten()
        .flat_map(|x_increment| {
            cycle += 1;
            if cycle % 40 == 20 {
                part_1 += cycle * x;
            }

            let mut result = if x.abs_diff(column) <= 1 {
                [Some('█'), None]
            } else {
                [Some(' '), None]
            };

            column += 1;
            if column == 40 {
                row += 1;
                column = 0;
                result[1] = Some('\n');
            }

            x += x_increment;

            result
        })
        .flatten()
        .collect::<String>();

    println!("Part 1: {part_1}");
    println!("Part 2:\n{part_2}");
}
