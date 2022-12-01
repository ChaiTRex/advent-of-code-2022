fn main() {
    macro_rules! handle_new_elf {
        ($top_three_elves: ident, $new_elf: ident) => {
            if $new_elf > $top_three_elves[1] {
                $top_three_elves[0] = $top_three_elves[1];
                if $new_elf > $top_three_elves[2] {
                    $top_three_elves[1] = $top_three_elves[2];
                    $top_three_elves[2] = $new_elf;
                } else {
                    $top_three_elves[1] = $new_elf;
                }
            } else if $new_elf > $top_three_elves[0] {
                $top_three_elves[0] = $new_elf;
            }
        };
    }

    let mut top_three_elves = [0, 0, 0];
    let mut new_elf = 0;

    for line in include_str!("../../../day01.txt").lines() {
        if line.is_empty() {
            handle_new_elf!(top_three_elves, new_elf);
            new_elf = 0;
        } else {
            new_elf += line.parse::<u32>().unwrap();
        }
    }
    handle_new_elf!(top_three_elves, new_elf);

    let part_1 = top_three_elves[2];
    let part_2 = part_1 + top_three_elves[1] + top_three_elves[0];

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
