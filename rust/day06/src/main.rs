fn main() {
    let input = include_bytes!("../../../day06.txt");
    let mut trailing_cursor = input.iter().copied().map(|ch| 1 << ((ch - b'a') << 2));
    let mut leading_cursor = (1..).zip(trailing_cursor.clone());

    let mut char_counts = 0_u128;

    // The array constants are the increase in length minus one, so the length starts out
    // at 0, then increases by 4 (written as 3), and then increases by 10 (written as 9).
    let [part_1, part_2] = [3, 9].map(|prestretch| {
        for (_, ch_bit) in leading_cursor.by_ref().take(prestretch) {
            char_counts += ch_bit;
        }

        loop {
            let (i, ch_bit) = leading_cursor.next().unwrap();
            char_counts += ch_bit;

            // 0xe is 0b1110 and each character is given four bits, so this allows us to
            // see when any of the characters has been seen more than once.
            if char_counts & 0xee_eeee_eeee_eeee_eeee_eeee_eeee == 0 {
                break i;
            }

            let ch_bit = trailing_cursor.next().unwrap();
            char_counts -= ch_bit;
        }
    });

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
