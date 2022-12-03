fn main() {
    static VALUES: [u8; 123] = {
        let mut result = [0; 123];

        let mut value = 1;
        let mut ch = b'a';
        while ch <= b'z' {
            result[ch as usize] = value;

            value += 1;
            ch += 1;
        }

        ch = b'A';
        while ch <= b'Z' {
            result[ch as usize] = value;

            value += 1;
            ch += 1;
        }

        result
    };

    let mut part_1 = 0;
    let mut part_2 = 0;

    let mut part_2_set = 0_u64;
    for (three_index, line) in (0..3).cycle().zip(
        include_str!("../../../day03.txt")
            .lines()
            .map(str::as_bytes),
    ) {
        {
            let (left_half, right_half) = line.split_at(line.len() >> 1);
            let mut left_set = 0_u64;
            for byte in left_half.iter().copied() {
                left_set |= 1 << VALUES[byte as usize];
            }
            let mut right_set = 0_u64;
            for byte in right_half.iter().copied() {
                right_set |= 1 << VALUES[byte as usize];
            }
            part_1 += (left_set & right_set).trailing_zeros() as u16;
        }

        {
            match three_index {
                0 => {
                    for byte in line.iter().copied() {
                        part_2_set |= 1 << VALUES[byte as usize];
                    }
                }
                1 => {
                    let mut part_2_other_set = 0_u64;
                    for byte in line.iter().copied() {
                        part_2_other_set |= 1 << VALUES[byte as usize];
                    }
                    part_2_set &= part_2_other_set;
                }
                _ => {
                    let mut part_2_other_set = 0_u64;
                    for byte in line.iter().copied() {
                        part_2_other_set |= 1 << VALUES[byte as usize];
                    }
                    part_2 += (part_2_set & part_2_other_set).trailing_zeros() as u16;
                    part_2_set = 0;
                }
            }
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
