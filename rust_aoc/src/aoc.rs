pub fn day1(input: &str) {
    let nums = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    let mut vecs = vec![&mut a, &mut b];
    
    nums.iter().enumerate().for_each(|(i, x)| {
        vecs[i % 2].push(*x);
    });

    a.sort();
    b.sort();

    println!("Part 1: {}", a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum::<i32>());

    println!("Part 2: {}", a.iter().map(|x| x * b.iter().filter(|&y| x == y).count() as i32).sum::<i32>());
}

pub fn day2(input: &str) {
    todo!()
}

pub fn day3(input: &str) {
    todo!()
}

pub fn day4(input: &str) {
    todo!()
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