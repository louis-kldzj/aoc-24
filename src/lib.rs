mod day1;
mod day2;
mod day3;
mod day4;

pub fn go() {
    /*println!("Day 1 - Part 1");
    println!("Answer is: {}", day1::solve_part1());
    println!("Day 1 - Part 2");
    println!("Answer is: {}", day1::solve_part2());

    println!("Day 2 - Part 1");
    println!("Answer is: {}", day2::solve_part1());
    println!("Day 2 - Part 2");
    println!("Answer is: {}", day2::solve_part2());

    println!("Day 3 - Part 1");
    println!("Answer is: {}", day3::solve_part1());
    println!("Day 3 - Part 2");
    println!("Answer is: {}", day3::solve_part2());*/

    let now = std::time::Instant::now();
    println!("Day 4 - Part 1");
    println!("Answer is: {}", day4::solve_part1());
    println!("Done in {:?}", now.elapsed());
    println!("Day 4 - Part 2");
    //   println!("Answer is: {}", day4::solve_part2());
}