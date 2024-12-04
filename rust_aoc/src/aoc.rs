pub fn day1(input: &str) {
    let nums = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    let mut vecs = vec![&mut a, &mut b];

    nums.iter().enumerate().for_each(|(i, x)| {
        vecs[i % 2].push(*x);
    });

    a.sort();
    b.sort();

    println!(
        "Part 1: {}",
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<i32>()
    );

    println!(
        "Part 2: {}",
        a.iter()
            .map(|x| x * b.iter().filter(|&y| x == y).count() as i32)
            .sum::<i32>()
    );
}

pub fn day2(input: &str) {
    let records = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut valid = 0;
    for record in &records {
        let diff = record
            .iter()
            .zip(record.iter().skip(1))
            .map(|(&x, &y)| x - y)
            .collect::<Vec<i32>>();

        if diff.iter().all(|&x| x > 0) || diff.iter().all(|&x| x < 0) {
            if diff.iter().all(|&x| 1 <= x.abs() && x.abs() <= 3) {
                valid += 1;
            }
        }
    }

    println!("Part 1: {}", valid);

    valid = 0;
    
    for record in &records { 
        let mut fail = 0;
        let diff = record
            .iter()
            .zip(record.iter().skip(1))
            .map(|(&x, &y)| x - y)
            .collect::<Vec<i32>>();

        let s = diff.iter().map(|&x| x.signum()).sum::<i32>().signum();
        // hardcoded xdd
        if diff[0].signum() != s {
            if diff.iter().skip(1).all(|&x| x.signum() == s && 1 <= x.abs() && x.abs() <= 3) {
                valid += 1;
                continue;
            }
        }
        if diff[diff.len() - 1].signum() != s {
            if diff.iter().take(diff.len() - 1).all(|&x| x.signum() == s && 1 <= x.abs() && x.abs() <= 3) {
                valid += 1;
                continue;
            }
        }

        let mut i: usize = 0;
        while i < diff.len() {
            if diff[i].signum() != s || !(1 <= diff[i].abs() && diff[i].abs() <= 3) {
                fail += 1;
                if fail > 1 {
                    break;
                }

                // remove the current element
                let mut failed_cur = false;
                let mut failed_prev = false;
                if i > 0 {
                    let new_diff = diff[i] + diff[i - 1];
                    if new_diff.signum() != s || !(1 <= new_diff.abs() && new_diff.abs() <= 3) {
                        failed_cur = true;
                    }
                }
                if i < diff.len() - 1 {
                    let new_diff = diff[i] + diff[i + 1];
                    if new_diff.signum() != s || !(1 <= new_diff.abs() && new_diff.abs() <= 3) {
                        failed_prev = true;
                    }
                    else {
                        i += 1;
                    }
                }
                if failed_cur && failed_prev {
                    fail += 1;
                    break;
                }
            } 
            i += 1;
        }
        
        if fail < 2 {
            valid += 1;
        }
    }

    println!("Part 2: {}", valid);
}

pub fn day3(input: &str) {
    let (mut p1, mut p2): (u64, u64) = (0, 0);
    let mut i: usize = 0;
    let mut active = true;

    while i < input.len() {
        if input[i..].starts_with("do()") {
            active = true;
        }
        else if input[i..].starts_with("don\'t()") {
            active = false;
        }
        // mul(\d+,\d+)
        else if input[i..].starts_with("mul(") {
            i += 4;
            let input_bytes = input[i..].as_bytes();
            let mut j = 0;
            let mut a = 0;
            let mut b = 0;
            while '0' <= input_bytes[j] as char && input_bytes[j] as char <= '9' {
                a = a * 10 + (input_bytes[j] - '0' as u8) as u64;
                j += 1;
            }
            if input_bytes[j] != ',' as u8 {
                i += j;
                continue;
            }
            j += 1;
            while '0' <= input_bytes[j] as char && input_bytes[j] as char <= '9' {
                b = b * 10 + (input_bytes[j] - '0' as u8) as u64;
                j += 1;
            }
            if input_bytes[j] != ')' as u8 {
                i += j;
                continue;
            }
            p1 += a * b;
            p2 += a * b * active as u64;
            i += j;
        }
        i += 1
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

pub fn day4(input: &str) {
    let grid = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dxy = vec![(0, 1), (1, 0), (1, 1), (1, -1)];
    let (n, m) = (grid.len(), grid[0].len());
    let mut c = 0;
    for x in 0..n {
        for y in 0..m {
            for (dx, dy) in &dxy {
                let i = x as i32;
                let j = y as i32;
                if i + 3 * dx < 0 || i + 3 * dx >= n as i32 || j + 3 * dy < 0 || j + 3 * dy >= m as i32 { // oob
                    continue;
                }

                let s = (0..4).map(|k| grid[(i + k * dx) as usize][(j + k * dy) as usize]).collect::<String>();
                c += (s == "XMAS" || s == "SAMX") as i32;
            }
        }
    }

    println!("Part 1: {}", c);

    c = 0;
    for x in 1..n-1 {
        for y in 1..m-1 {
            if grid[x][y] != 'A' {
                continue;
            }

            // abuse that fact that only [XMAS] in grid
            let down = (grid[x-1][y-1] as u8) ^ (grid[x+1][y+1] as u8);
            let up = (grid[x-1][y+1] as u8) ^ (grid[x+1][y-1] as u8);
            c += (down == 0b00011110 && up == 0b00011110) as i32;
        }
    }
    println!("Part 2: {}", c);
}

pub fn day5(input: &str) {
    todo!()
}

pub fn day6(input: &str) {
    todo!()
}

pub fn day7(input: &str) {
    todo!()
}

pub fn day8(input: &str) {
    todo!()
}

pub fn day9(input: &str) {
    todo!()
}

pub fn day10(input: &str) {
    todo!()
}

pub fn day11(input: &str) {
    todo!()
}

pub fn day12(input: &str) {
    todo!()
}

pub fn day13(input: &str) {
    todo!()
}

pub fn day14(input: &str) {
    todo!()
}

pub fn day15(input: &str) {
    todo!()
}

pub fn day16(input: &str) {
    todo!()
}

pub fn day17(input: &str) {
    todo!()
}

pub fn day18(input: &str) {
    todo!()
}

pub fn day19(input: &str) {
    todo!()
}

pub fn day20(input: &str) {
    todo!()
}

pub fn day21(input: &str) {
    todo!()
}

pub fn day22(input: &str) {
    todo!()
}

pub fn day23(input: &str) {
    todo!()
}

pub fn day24(input: &str) {
    todo!()
}

pub fn day25(input: &str) {
    todo!()
}
