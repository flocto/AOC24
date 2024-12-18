use std::collections::{HashMap, HashSet, VecDeque};

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
    let evals = input
        .split("\n")
        .map(
            |line| -> (usize, Vec<usize>) {
                let parts = line.split(": ").collect::<Vec<&str>>();
                let res = parts[0].parse::<usize>().unwrap();
                let nums = parts.iter().skip(1).next().unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                (res, nums)
            }
        );
    
    let mut found_p1 = HashSet::new();
    let p1 = evals.clone().map(|(res, nums)| {
        let mut possible = HashSet::new();
        possible.insert(nums[0]);

        nums.iter().skip(1).for_each(|&x| {
            let mut new = HashSet::new();
            possible.iter().for_each(|&p| {
                new.insert(p + x);
                new.insert(p * x);
            });
            possible = new.iter().filter(|&x| *x <= res).cloned().collect();
        });

        if !possible.contains(&res) {
            return 0;
        }
        
        found_p1.insert(res);
        res
    }).sum::<usize>();

    println!("Part 1: {}", p1);

    let p2 = evals.clone().map(|(res, nums)| {
        if found_p1.contains(&res) {
            return res;
        }

        let mut possible = HashSet::new();
        possible.insert(nums[0]);

        nums.iter().skip(1).for_each(|&x| {
            let mut new = HashSet::new();
            possible.iter().for_each(|&p| {
                new.insert(p + x);
                new.insert(p * x);
                // concat
                new.insert(format!("{}{}", p, x).parse::<usize>().unwrap());
            });
            possible = new.iter().filter(|&x| *x <= res).cloned().collect();
        });

        if !possible.contains(&res) {
            return 0;
        }
        
        res
    }).sum::<usize>();

    println!("Part 2: {}", p2);
}

pub fn day8(input: &str) {
    let grid = input
        .split("\n")
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    // map of c -> [(x1, y1), (x2, y2), ...]
    let signals = grid.iter().enumerate().fold(HashMap::new(), |mut acc, (i, line)| {
        line.iter().enumerate().for_each(|(j, &c)| {
            if c != b'.' {
                acc.entry(c).or_insert(Vec::new()).push((i, j));
            }
        });
        acc
    });

    let antinodes = signals.iter().fold(HashSet::new(), |mut acc, (_, v)| {
        v.iter().for_each(|&(x1, y1)| {
            v.iter().for_each(|&(x2, y2)| {
                if x1 == x2 && y1 == y2 {
                    return;
                }

                let [dx, dy] = [x2 as i32 - x1 as i32, y2 as i32 - y1 as i32];
                let [nx, ny] = [x2 as i32 + dx, y2 as i32 + dy];
                if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                    acc.insert((nx, ny));
                }
            });
        });
        acc
    });

    println!("Part 1: {}", antinodes.len());

    let full_antinodes = signals.iter().filter(|(_, v)| v.len() > 1).fold(HashSet::new(), |mut acc, (_, v)| {
        v.iter().for_each(|&(x1, y1)| {
            v.iter().for_each(|&(x2, y2)| {
                if x1 == x2 && y1 == y2 {
                    return;
                }

                let [dx, dy] = [x2 as i32 - x1 as i32, y2 as i32 - y1 as i32];
                let [mut nx, mut ny] = [x2 as i32, y2 as i32];
                while nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                    acc.insert((nx, ny));
                    nx += dx;
                    ny += dy;
                }
            });
        });
        acc
    });

    println!("Part 2: {}", full_antinodes.len());
}

pub fn day9(input: &str) {
    // okay i actually dont want to reimplement this one its really really annoying just see my 2pointers solution in day9.py
    todo!()
}

pub fn day10(input: &str) {
    let grid = input
        .split("\n")
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let (n, m) = (grid.len() as i32, grid[0].len() as i32);
    let dxy = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let explore = |grid: &Vec<Vec<u8>>, (x, y)| {
        let mut c = (0, 0);
        let mut visited = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((x, y));
        visited.insert((x, y), 1);

        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            if grid[x as usize][y as usize] == b'9' {
                c.0 += 1;
                c.1 += visited[&(x, y)];
            }

            for (dx, dy) in &dxy {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || nx >= n || ny < 0 || ny >= m {
                    continue;
                }

                if grid[nx as usize][ny as usize] == grid[x as usize][y as usize] + 1 {
                    if !visited.contains_key(&(nx, ny)) {
                        q.push_back((nx, ny));

                    }
                    visited.insert((nx, ny), visited.get(&(nx, ny)).unwrap_or(&0) + visited[&(x, y)]);
                }
            }
        }
        c
    };

    let (p1, p2) = (0..n).flat_map(|x| (0..m).map(move |y| (x, y))).fold((0, 0), |(p1, p2), (x, y)| {
        if grid[x as usize][y as usize] != b'0' {
            return (p1, p2);
        }

        let (c1, c2) = explore(&grid, (x, y));
        (p1 + c1, p2 + c2)
    });

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

pub fn day11(input: &str) {
    let nums = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let apply_rule = |counter: HashMap<usize, usize>| {
        let mut new_counter = HashMap::new();
        for (&n, &c) in counter.iter() {
            if n == 0 {
                new_counter.insert(1, new_counter.get(&1usize).unwrap_or(&0) + c);
            }
            else if n.ilog10() % 2 == 1 {
                let half = (n.ilog10() + 1) / 2;
                let [a, b] = [n / 10usize.pow(half), n % 10usize.pow(half)];
                new_counter.insert(a, new_counter.get(&a).unwrap_or(&0) + c);
                new_counter.insert(b, new_counter.get(&b).unwrap_or(&0) + c);
            }
            else {
                new_counter.insert(n * 2024, new_counter.get(&(n * 2024)).unwrap_or(&0) + c);
            }
        }
        new_counter
    };

    let mut counter = nums.iter().fold(HashMap::new(), |mut acc, &x| {
        acc.insert(x, acc.get(&x).unwrap_or(&0) + 1);
        acc
    });

    for _ in 0..25 {
        counter = apply_rule(counter);
    }
    println!("Part 1: {}", counter.values().sum::<usize>());

    for _ in 0..50 {
        counter = apply_rule(counter);
    }
    println!("Part 2: {}", counter.values().sum::<usize>());
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
