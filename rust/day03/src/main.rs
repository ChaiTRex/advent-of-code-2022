fn main() {
    static PRIORITIES: [u8; 58] = {
        let mut result = [0; 58];

        let mut value = 1;
        let mut ch = b'a' - b'A';
        while ch <= b'z' - b'A' {
            result[ch as usize] = value;

            value += 1;
            ch += 1;
        }

        ch = 0; // b'A' - b'A'
        while ch <= b'Z' - b'A' {
            result[ch as usize] = value;

            value += 1;
            ch += 1;
        }

        result
    };

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut lines = include_str!("../../../day03.txt")
        .lines()
        .map(str::as_bytes);
    'outer: loop {
        let mut part_2_set = !0_u64;

        for _ in 0..3 {
            let line = match lines.next() {
                Some(line) => line,
                None => break 'outer,
            };

            let (left_half, right_half) = line.split_at(line.len() >> 1);
            let mut left_set = 0_u64;
            for byte in left_half.iter().copied() {
                left_set |= 1 << (byte - b'A');
            }
            let mut right_set = 0_u64;
            for byte in right_half.iter().copied() {
                right_set |= 1 << (byte - b'A');
            }

            part_1 += PRIORITIES[(left_set & right_set).trailing_zeros() as usize] as u16;
            part_2_set &= left_set | right_set;
        }

        part_2 += PRIORITIES[part_2_set.trailing_zeros() as usize] as u16;
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
