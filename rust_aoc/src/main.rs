use chrono::Datelike;

mod aoc;
fn main() {
    // get EST datetime
    let est = chrono::offset::FixedOffset::east_opt(-5 * 3600).unwrap();
    let now = chrono::Utc::now().with_timezone(&est);
    let date = now.day() as usize;

    // read input from ../input/day{date}.txt
    let input = std::fs::read_to_string(format!("../input/day{}.txt", date)).unwrap();

    // call the function for the day
    // ignore this jank please i dont want to bother learning reflection
    let func: fn(&str) = vec![
        aoc::day1,
        aoc::day2,
        aoc::day3,
        aoc::day4,
        aoc::day5,
        aoc::day6,
        aoc::day7,
        aoc::day8,
        aoc::day9,
        aoc::day10,
        aoc::day11,
        aoc::day12,
        aoc::day13,
        aoc::day14,
        aoc::day15,
        aoc::day16,
        aoc::day17,
        aoc::day18,
        aoc::day19,
        aoc::day20,
        aoc::day21,
        aoc::day22,
        aoc::day23,
        aoc::day24,
        aoc::day25,
    ][date - 1];

    func(&input);
}
