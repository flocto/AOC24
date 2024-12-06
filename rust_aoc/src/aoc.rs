use std::{collections::HashSet, hash::Hash};

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
            if diff
                .iter()
                .skip(1)
                .all(|&x| x.signum() == s && 1 <= x.abs() && x.abs() <= 3)
            {
                valid += 1;
                continue;
            }
        }
        if diff[diff.len() - 1].signum() != s {
            if diff
                .iter()
                .take(diff.len() - 1)
                .all(|&x| x.signum() == s && 1 <= x.abs() && x.abs() <= 3)
            {
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
                    } else {
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
        } else if input[i..].starts_with("don\'t()") {
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
                if i + 3 * dx < 0
                    || i + 3 * dx >= n as i32
                    || j + 3 * dy < 0
                    || j + 3 * dy >= m as i32
                {
                    // oob
                    continue;
                }

                let s = (0..4)
                    .map(|k| grid[(i + k * dx) as usize][(j + k * dy) as usize])
                    .collect::<String>();
                c += (s == "XMAS" || s == "SAMX") as i32;
            }
        }
    }

    println!("Part 1: {}", c);

    c = 0;
    for x in 1..n - 1 {
        for y in 1..m - 1 {
            if grid[x][y] != 'A' {
                continue;
            }

            // abuse that fact that only [XMAS] in grid
            let down = (grid[x - 1][y - 1] as u8) ^ (grid[x + 1][y + 1] as u8);
            let up = (grid[x - 1][y + 1] as u8) ^ (grid[x + 1][y - 1] as u8);
            c += (down == 0b00011110 && up == 0b00011110) as i32;
        }
    }
    println!("Part 2: {}", c);
}

pub fn day5(input: &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules = parts[0]
        .split("\n")
        .map(|line| {
            let mut parts = line.split("|");
            let left = parts.next().unwrap().parse::<usize>().unwrap();
            let right = parts.next().unwrap().parse::<usize>().unwrap();
            (left, right)
        })
        .collect::<HashSet<(usize, usize)>>();
    let updates = parts[1]
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let (mut p1, mut p2) = (0, 0);

    for mut update in updates {
        let update_pairs = update
            .iter()
            .map(|&x| {
                update
                    .iter()
                    .map(|&y| (x, y))
                    .collect::<HashSet<(usize, usize)>>()
            })
            .flatten()
            .collect::<HashSet<(usize, usize)>>();
        let used_rules: HashSet<(usize, usize)> =
            update_pairs.intersection(&rules).cloned().collect();

        let mut seen = Vec::new();
        let mut bad = false;
        update.iter().for_each(|&page| {
            seen.iter().for_each(|&s| {
                if used_rules.contains(&(page, s)) {
                    bad = true;
                }
            });
            seen.push(page);
        });

        if !bad {
            p1 += update[update.len() / 2];
            continue;
        }

        while bad {
            bad = false;
            used_rules.iter().for_each(|&(x, y)| {
                let (i, j) = (
                    update.iter().position(|&z| z == x).unwrap(),
                    update.iter().position(|&z| z == y).unwrap(),
                );
                if i >= j {
                    update.swap(i, j);
                    bad = true;
                }
            });
        }

        p2 += update[update.len() / 2];
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

pub fn day6(input: &str) {
    let grid = input
        .split("\n")
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let (n, m) = (grid.len(), grid[0].len());
    let start: (i32, i32) = (0..n)
        .flat_map(|i| (0..m).map(move |j| (i, j)))
        .find(|&(i, j)| grid[i][j] == b'^')
        .map(|(i, j)| (i as i32, j as i32))
        .unwrap();

    let dxy = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let explore = |start: (i32, i32), blocker: (i32, i32)| {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut path: HashSet<((i32, i32), usize)> = HashSet::new();
        let mut d: usize = 0;

        visited.insert(start);
        path.insert((start, d));
        let mut inf = false;

        let (mut x, mut y) = start;
        loop {
            let (nx, ny) = (x + dxy[d].0, y + dxy[d].1);
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                break;
            }

            if grid[nx as usize][ny as usize] == b'#' || (nx, ny) == blocker {
                d = (d + 1) % 4;
                continue;
            }

            if path.contains(&((nx, ny), d)) {
                inf = true;
                break;
            }

            (x, y) = (nx, ny);
            visited.insert((x, y));
            path.insert(((x, y), d));
        }
        (visited, inf)
    };

    let visited = explore(start, (-1, -1)).0;
    println!("Part 1: {}", visited.len());

    let c = visited.iter().map(|&(x, y)| explore(start, (x, y)).1 as i32).sum::<i32>();
    println!("Part 2: {}", c);
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
