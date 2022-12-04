fn main() {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in include_str!("../../../day04.txt")
        .lines()
        .map(str::as_bytes)
    {
        // Get the sections as two-byte u16s, which have identical relative orderingsa as
        // the numbers they represent.
        let mut i = 0;
        let [a, b, c, d] = [(); 4].map(|_| {
            let byte_0 = line[i];
            match line.get(i + 1) {
                Some(&byte_1) if byte_1 >= b'0' => {
                    i += 3;
                    u16::from_be_bytes([byte_0, byte_1])
                }
                _ => {
                    i += 2;
                    byte_0 as u16
                }
            }
        });

        if (a <= c && b >= d) || (c <= a && d >= b) {
            part_1 += 1;
        }
        if a <= d && b >= c {
            part_2 += 1;
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
