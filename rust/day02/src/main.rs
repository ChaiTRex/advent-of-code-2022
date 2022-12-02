fn main() {
    static SCORES: [(u16, u16); 9] = [
        (4, 3),
        (8, 4),
        (3, 8),
        (1, 1),
        (5, 5),
        (9, 9),
        (7, 2),
        (2, 6),
        (6, 7),
    ];

    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in include_str!("../../../day02.txt").lines() {
        let line = line.as_bytes();
        let (part_1_score, part_2_score) = SCORES[3 * line[0] as usize + line[2] as usize - 283];

        part_1 += part_1_score;
        part_2 += part_2_score;
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
