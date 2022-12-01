fn main() {
    let top_three_elves = include_str!("../../../day01.txt")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .fold([0, 0, 0], |mut top_three_elves, new_elf| {
            if new_elf > top_three_elves[1] {
                top_three_elves[0] = top_three_elves[1];
                if new_elf > top_three_elves[2] {
                    top_three_elves[1] = top_three_elves[2];
                    top_three_elves[2] = new_elf;
                } else {
                    top_three_elves[1] = new_elf;
                }
            } else if new_elf > top_three_elves[0] {
                top_three_elves[0] = new_elf;
            }

            top_three_elves
        });

    let part_1 = top_three_elves[2];
    let part_2 = part_1 + top_three_elves[1] + top_three_elves[0];

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
