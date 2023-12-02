use aoc_2023::day1;
use aoc_2023::day2;

fn main() {
    let input1 = include_str!("inputs/day1.txt");
    println!("Day 1.1 solution: {}", day1::part_1(input1));
    println!("Day 1.2 solution: {}", day1::part_2(input1));
    println!("==========================================");
    let input2 = include_str!("inputs/day2.txt");
    println!("Day 2.1 solution: {}", day2::part_1(input2));
    println!("Day 2.2 solution: {}", day2::part_2(input2));
}
