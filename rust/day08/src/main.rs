fn main() {
    let heights = include_str!("../../../day08.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut visible = vec![vec![false; heights[0].len()]; heights.len()];
    let mut column_maxes_from_top = vec![0; heights[0].len()];
    for (row_of_heights, row_of_visible) in heights.iter().zip(visible.iter_mut()) {
        let mut max_so_far = 0;
        for (height, visible) in row_of_heights
            .iter()
            .copied()
            .zip(row_of_visible.iter_mut())
        {
            if height > max_so_far {
                *visible = true;
                max_so_far = height;
            }
        }

        let mut max_so_far = 0;
        for (height, visible) in row_of_heights
            .iter()
            .rev()
            .copied()
            .zip(row_of_visible.iter_mut().rev())
        {
            if height > max_so_far {
                *visible = true;
                max_so_far = height;
            }
        }

        for ((height, visible), column_max) in row_of_heights
            .iter()
            .zip(row_of_visible.iter_mut())
            .zip(column_maxes_from_top.iter_mut())
        {
            if height > column_max {
                *visible = true;
                *column_max = *height;
            }
        }
    }

    let mut column_maxes_from_bottom = vec![0; heights[0].len()];
    for (row_of_heights, row_of_visible) in heights.iter().rev().zip(visible.iter_mut().rev()) {
        for ((height, visible), column_max) in row_of_heights
            .iter()
            .zip(row_of_visible.iter_mut())
            .zip(column_maxes_from_bottom.iter_mut())
        {
            if height > column_max {
                *visible = true;
                *column_max = *height;
            }
        }
    }

    let part_1 = visible
        .iter()
        .map(|row| {
            row.iter()
                .map(|is_visible| if *is_visible { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();

    let mut part_2 = 0;
    for row_index in 0..heights.len() {
        for column_index in 0..heights[row_index].len() {
            fn solve(heights: &Vec<&[u8]>, row_index: usize, column_index: usize) -> usize {
                let my_height = heights[row_index][column_index];

                let mut result = core::cmp::min(
                    row_index,
                    1 + (0..row_index)
                        .rev()
                        .take_while(|&new_row_index| {
                            heights[new_row_index][column_index] < my_height
                        })
                        .count(),
                );
                result *= core::cmp::min(
                    heights.len() - row_index - 1,
                    1 + (row_index + 1..heights.len())
                        .take_while(|&new_row_index| {
                            heights[new_row_index][column_index] < my_height
                        })
                        .count(),
                );
                result *= core::cmp::min(
                    column_index,
                    1 + (0..column_index)
                        .rev()
                        .take_while(|&new_column_index| {
                            heights[row_index][new_column_index] < my_height
                        })
                        .count(),
                );
                result *= core::cmp::min(
                    heights[row_index].len() - column_index - 1,
                    1 + (column_index + 1..heights[row_index].len())
                        .take_while(|&new_column_index| {
                            heights[row_index][new_column_index] < my_height
                        })
                        .count(),
                );

                result
            }

            part_2 = core::cmp::max(part_2, solve(&heights, row_index, column_index));
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 1: {part_2}");
}
