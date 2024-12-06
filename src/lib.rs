mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn go() {
    let now = std::time::Instant::now();
    println!("Day 1 - Part 1");
    println!("Answer is: {}", day1::solve_part1());
    println!("Done in {:?}", now.elapsed());
    let now = std::time::Instant::now();
    println!("Day 1 - Part 2");
    println!("Answer is: {}", day1::solve_part2());
    println!("Done in {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!("Day 2 - Part 1");
    println!("Answer is: {}", day2::solve_part1());
    println!("Done in {:?}", now.elapsed());
    let now = std::time::Instant::now();
    println!("Day 2 - Part 2");
    println!("Answer is: {}", day2::solve_part2());
    println!("Done in {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!("Day 3 - Part 1");
    println!("Answer is: {}", day3::solve_part1());
    println!("Done in {:?}", now.elapsed());
    let now = std::time::Instant::now();
    println!("Day 3 - Part 2");
    println!("Answer is: {}", day3::solve_part2());
    println!("Done in {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!("Day 4 - Part 1");
    println!("Answer is: {}", day4::solve_part1());
    println!("Done in {:?}", now.elapsed());
    let now = std::time::Instant::now();
    println!("Day 4 - Part 2");
    println!("Answer is: {}", day4::solve_part2());
    println!("Done in {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!("Day 5 - Part 1 and 2");
    println!("Answer is: {:?}", day5::solve_part1());
    println!("Done in {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!("Day 6 - Part 1");
    println!("Answer is: {}", day6::solve_part1());
    println!("Done in {:?}", now.elapsed());
    let now = std::time::Instant::now();
    println!("Day 6 - Part 2");
    println!("Answer is: {}", day6::solve_part2());
    println!("Done in {:?}", now.elapsed());
}
