fn main() {
    let mut elevations = include_str!("../../../day12.txt")
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut current_position = None;
    let mut destination = None;
    for (y, row) in elevations.iter().enumerate() {
        for (x, elevation) in row.iter().enumerate() {
            match elevation {
                b'S' => current_position = Some((x, y)),
                b'E' => destination = Some((x, y)),
                _ => (),
            }
        }
    }
    let current_position = current_position.unwrap();
    let destination = destination.unwrap();
    let mut step_counts = vec![vec![i32::MAX; elevations[0].len()]; elevations.len()];
    
    {
        let (x, y) = current_position;
        elevations[y][x] = b'a';
        step_counts[y][x] = 0;
        let (x, y) = destination;
        elevations[y][x] = b'z';
    }
    
    fn find_shortest_paths((x, y): (usize, usize), elevations: &mut Vec<Vec<u8>>, step_counts: &mut Vec<Vec<i32>>) {
        let current_elevation = elevations[y][x];
        let current_step_count = step_counts[y][x];
        let next_step_count = current_step_count + 1;
        
        //println!("({x}, {y}) with elevation {current_elevation} and step count {current_step_count}");
        
        if y > 0 {
            if let Some(row) = elevations.get(y - 1) {
                let next_elevation = row[x];
                if current_elevation + 1 >= next_elevation && next_step_count < step_counts[y - 1][x] {
                    step_counts[y - 1][x] = next_step_count;
                    find_shortest_paths((x, y - 1), elevations, step_counts);
                }
            }
        }
        if let Some(row) = elevations.get(y + 1) {
            let next_elevation = row[x];
            if current_elevation + 1 >= next_elevation && next_step_count < step_counts[y + 1][x] {
                step_counts[y + 1][x] = next_step_count;
                find_shortest_paths((x, y + 1), elevations, step_counts);
            }
        }
        if x > 0 {
            let row = &elevations[y];
            if let Some(next_elevation) = row.get(x - 1) {
                if current_elevation + 1 >= *next_elevation && next_step_count < step_counts[y][x - 1] {
                    step_counts[y][x - 1] = next_step_count;
                    find_shortest_paths((x - 1, y), elevations, step_counts);
                }
            }
        }
        let row = &elevations[y];
        if let Some(next_elevation) = row.get(x + 1) {
            if current_elevation + 1 >= *next_elevation && next_step_count < step_counts[y][x + 1] {
                step_counts[y][x + 1] = next_step_count;
                find_shortest_paths((x + 1, y), elevations, step_counts);
            }
        }
    }
    find_shortest_paths(current_position, &mut elevations, &mut step_counts);
    
    let part_1 = step_counts[destination.1][destination.0];
    
    let mut step_counts = vec![vec![i32::MAX; elevations[0].len()]; elevations.len()];
    step_counts[destination.1][destination.0] = 0;
    fn find_shortest_paths_2((x, y): (usize, usize), elevations: &mut Vec<Vec<u8>>, step_counts: &mut Vec<Vec<i32>>) {
        let current_elevation = elevations[y][x];
        let current_step_count = step_counts[y][x];
        let next_step_count = current_step_count + 1;
        
        //println!("({x}, {y}) with elevation {current_elevation} and step count {current_step_count}");
        
        if y > 0 {
            if let Some(row) = elevations.get(y - 1) {
                let next_elevation = row[x];
                if next_elevation + 1 >= current_elevation && next_step_count < step_counts[y - 1][x] {
                    step_counts[y - 1][x] = next_step_count;
                    find_shortest_paths_2((x, y - 1), elevations, step_counts);
                }
            }
        }
        if let Some(row) = elevations.get(y + 1) {
            let next_elevation = row[x];
            if next_elevation + 1 >= current_elevation && next_step_count < step_counts[y + 1][x] {
                step_counts[y + 1][x] = next_step_count;
                find_shortest_paths_2((x, y + 1), elevations, step_counts);
            }
        }
        if x > 0 {
            let row = &elevations[y];
            if let Some(next_elevation) = row.get(x - 1) {
                if next_elevation + 1 >= current_elevation && next_step_count < step_counts[y][x - 1] {
                    step_counts[y][x - 1] = next_step_count;
                    find_shortest_paths_2((x - 1, y), elevations, step_counts);
                }
            }
        }
        let row = &elevations[y];
        if let Some(next_elevation) = row.get(x + 1) {
            if next_elevation + 1 >= current_elevation && next_step_count < step_counts[y][x + 1] {
                step_counts[y][x + 1] = next_step_count;
                find_shortest_paths_2((x + 1, y), elevations, step_counts);
            }
        }
    }
    find_shortest_paths_2(destination, &mut elevations, &mut step_counts);
    
    let mut min_elevation_a_step_count = i32::MAX;
    for (y, row) in elevations.iter().enumerate() {
        for (x, elevation) in row.iter().copied().enumerate() {
            if elevation == b'a' {
                println!("{} at ({x}, {y})", step_counts[y][x]);
                min_elevation_a_step_count = core::cmp::min(min_elevation_a_step_count, step_counts[y][x]);
            }
        }
    }
    
    let part_2 = min_elevation_a_step_count;
    
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
