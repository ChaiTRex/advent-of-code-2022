fn main() {
    let input = include_str!("../../../day01.txt");

    let part1 = input
        .split("\n\n")
        .map(|elf| {
            elf
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    let mut part2 = input
        .split("\n\n")
        .map(|elf| {
            elf
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    part2.sort();
    part2.reverse();
    let part2 = part2[0..3].iter().sum::<u32>();
    
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
