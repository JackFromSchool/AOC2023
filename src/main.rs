use aoc_2023::day1;
use aoc_2023::day2;
use aoc_2023::day3;
use aoc_2023::day4;
use aoc_2023::day5;

fn main() {
    let input1 = include_str!("inputs/day1.txt");
    println!("Day 1.1 solution: {}", day1::part_1(input1));
    println!("Day 1.2 solution: {}", day1::part_2(input1));
    println!("==========================================");
    let input2 = include_str!("inputs/day2.txt");
    println!("Day 2.1 solution: {}", day2::part_1(input2));
    println!("Day 2.2 solution: {}", day2::part_2(input2));
    println!("=========================================");
    let input3 = include_str!("inputs/day3.txt");
    println!("Day 3.1 solution {}", day3::part_1(input3));
    println!("Day 3.2 solution {}", day3::part_2(input3));
    println!("=========================================");
    let input4 = include_str!("inputs/day4.txt");
    println!("Day 4.1 solution {}", day4::part_1(input4));
    println!("Day 4.2 solution {}", day4::part_2(input4));
    println!("=========================================");
    let input5 = include_str!("inputs/day5.txt");
    println!("Day 5.1 solution {}", day5::part_1(input5));
    println!("Day 5.2 solution {}", day5::part_2(input5));
}
