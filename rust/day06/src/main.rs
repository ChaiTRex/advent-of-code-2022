fn main() {
    let input = include_bytes!("../../../day06.txt");
    let mut trailing_cursor = input.iter().copied().map(|ch| (ch - b'a') as usize);
    let mut leading_cursor = (1..).zip(trailing_cursor.clone());

    let mut seen = [0_u8; 26];

    for (_, ch) in leading_cursor.by_ref().take(3) {
        seen[ch] += 1;
    }

    let part_1 = loop {
        let (i, ch) = leading_cursor.next().unwrap();
        seen[ch] += 1;

        if seen.iter().copied().all(|count| count <= 1) {
            for (_, ch) in leading_cursor.by_ref().take(9) {
                seen[ch] += 1;
            }

            break i;
        }

        let ch = trailing_cursor.next().unwrap();
        seen[ch] -= 1;
    };

    let part_2 = loop {
        let (i, ch) = leading_cursor.next().unwrap();
        seen[ch] += 1;

        if seen.iter().copied().all(|count| count <= 1) {
            break i;
        }

        let ch = trailing_cursor.next().unwrap();
        seen[ch] -= 1;
    };

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
