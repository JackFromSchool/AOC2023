use regex::Regex;

struct NumPos {
    num: u32,
    y_pos: usize,
    x_range: std::ops::Range<usize>,
}

impl NumPos {
    
    fn new(num: u32, y_pos: usize, x_range: std::ops::Range<usize>) -> Self {
        Self {
            num,
            y_pos,
            x_range,
        }
    }
    
}

trait NextTo {
    fn next_to(&self, x: usize) -> bool;
}

impl NextTo for std::ops::Range<usize> {
    fn next_to(&self, x: usize) -> bool {
        self.contains(&(x + 1)) || 
        self.contains(&x) ||
        self.contains(&((x as isize - 1) as usize))
    }
}

pub fn part_1(input: &str) -> u32 {
    let num = Regex::new("[0-9]+").unwrap();
    let symbol = Regex::new(r"[^0-9.]").unwrap();

    let mut nums = Vec::new();
    let mut sum = 0;
    
    for (y, line) in input.lines().enumerate() {
        for m in num.find_iter(line) {
            nums.push(NumPos::new(
                m.as_str().parse().unwrap(),
                y,
                m.range()
            ));
        }
    }

    for (y, line) in input.lines().enumerate() {
        for m in symbol.find_iter(line) {
            sum += nums.iter()
                .filter(|x| x.y_pos == y || x.y_pos == y+1 || x.y_pos == y-1)
                .filter(|x| x.x_range.next_to(m.start()))
                .map(|x| x.num)
                .sum::<u32>();
        }
    }

    sum
}

pub fn part_2(input: &str) -> u32 {
    let num = Regex::new("[0-9]+").unwrap();
    let symbol = Regex::new(r"\*").unwrap();

    let mut nums = Vec::new();
    let mut sum = 0;
    
    for (y, line) in input.lines().enumerate() {
        for m in num.find_iter(line) {
            nums.push(NumPos::new(
                m.as_str().parse().unwrap(),
                y,
                m.range()
            ));
        }
    }

    for (y, line) in input.lines().enumerate() {
        for m in symbol.find_iter(line) {
            let gears = nums.iter()
                .filter(|x| x.y_pos == y || x.y_pos == y+1 || x.y_pos == y-1)
                .filter(|x| x.x_range.next_to(m.start()))
                .map(|x| x.num)
                .collect::<Vec<u32>>();

            if gears.len() == 2 {
                sum += gears[0]*gears[1];
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    
    use crate::day3::{part_1, part_2};
    
    #[test]
    fn part_1_test() {
        let input =
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_1(input), 4361)
    }
    
    #[test]
    fn part_2_test() {
        let input =
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_2(input), 467835)
    }
}
