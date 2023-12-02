use regex::Regex;

pub fn part_1(str: &str) -> i32 {
    let re = Regex::new("[0-9]").unwrap();
    let mut all_nums: Vec<i32> = Vec::new();
    
    for line in str.lines() {
        let nums: Vec<_> = 
            re.find_iter(line).map(|m| str::parse::<i32>(m.as_str()).unwrap()).collect();

        all_nums.push(nums[0]*10 + nums[nums.len()-1]);
    }
    
    return all_nums.iter().sum();
}

pub fn part_2(str: &str) -> i32 {
    let re = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut all_nums: Vec<i32> = Vec::new();

    for line in str.lines() {
        let mut nums = Vec::new();
        
        // Stupid trick to get overlapping regex matches
        for i in 0..line.len() {
            if let Some(m) = re.find_at(line, i) {
                if let Ok(val) = str::parse::<i32>(m.as_str()) {
                    nums.push(val);
                } else {
                    nums.push(match m.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => unreachable!()
                    });
                }
            }
        }

        all_nums.push(nums[0]*10 + nums[nums.len()-1]);
    }

    return all_nums.iter().sum();
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn part_1() {
        let input = 
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

       assert_eq!(crate::day1::part_1(input), 142)
    }

    #[test]
    fn part_2() {
        let input =
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        
        assert_eq!(crate::day1::part_2(input), 281)
    }
}
