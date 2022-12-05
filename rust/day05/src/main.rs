fn main() {
    let (crates, instructions) = include_str!("../../../day05.txt")
        .split_once("\n\n")
        .unwrap();

    let mut part_1_columns = vec![Vec::new(); 9];
    for line in crates.lines().map(str::as_bytes) {
        for i in (0..line.len()).step_by(4).filter(|&i| line[i] == b'[') {
            part_1_columns[i >> 2].push(line[i + 1] as char);
        }
    }
    for column in part_1_columns.iter_mut() {
        column.reverse();
    }
    let mut part_2_columns = part_1_columns.clone();

    for line in instructions.lines() {
        let (count, rest) = line.split_once(" from ").unwrap();
        let count = count[5..].parse::<usize>().unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = to.parse::<usize>().unwrap() - 1;

        let start = part_1_columns[from].len() - count;

        let mut moved_elements = part_1_columns[from].drain(start..).collect::<Vec<_>>();
        moved_elements.reverse();
        part_1_columns[to].extend(moved_elements);

        let moved_elements = part_2_columns[from].drain(start..).collect::<Vec<_>>();
        part_2_columns[to].extend(moved_elements);
    }

    let part_1 = part_1_columns
        .iter()
        .map(|column| column.last().unwrap())
        .collect::<String>();
    let part_2 = part_2_columns
        .iter()
        .map(|column| column.last().unwrap())
        .collect::<String>();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
