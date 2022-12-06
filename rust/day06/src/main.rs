fn main() {
    let input = include_bytes!("../../../day06.txt");
    let mut trailing_cursor = input.iter().copied().map(|ch| (ch - b'a') as usize);
    let mut leading_cursor = (1..).zip(trailing_cursor.clone());

    let mut seen = [0_u8; 26];

    // The array constants are the increase in length minus one, so the length starts out
    // at 0, then increases by 4 (written as 3), and then increases by 10 (written as 9).
    let [part_1, part_2] = [3, 9].map(|prestretch| {
        for (_, ch) in leading_cursor.by_ref().take(prestretch) {
            seen[ch] += 1;
        }

        loop {
            let (i, ch) = leading_cursor.next().unwrap();
            seen[ch] += 1;

            if seen.iter().copied().all(|count| count <= 1) {
                break i;
            }

            let ch = trailing_cursor.next().unwrap();
            seen[ch] -= 1;
        }
    });

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
