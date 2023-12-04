use regex::Regex;
use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    let nums = Regex::new("[0-9]+").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let mut wins: HashSet<u32> = HashSet::new();
        let split: Vec<&str> = line.split('|').collect();
        let mut match_iter = nums.find_iter(split[0]);
        match_iter.next();
        
        for m in match_iter {
            wins.insert(m.as_str().parse().unwrap());
        }
        
        let mut matches: i32 = -1;
        
        for m in nums.find_iter(split[1]) {
            if wins.contains(&m.as_str().parse().unwrap()) {
                matches += 1;
            }
        }

        sum += if matches == -1 { 0 } else { (2 as u32).pow(matches as u32) };
    }
    
    sum
}

#[derive(Debug)]
struct Card {
    pub matches: usize,
    pub copies: u128,
}

pub fn part_2(input: &str) -> u128 {
    let nums = Regex::new("[0-9]+").unwrap();
    let mut card_vec: Vec<Card> = Vec::new();

    for line in input.lines() {
        let mut wins: HashSet<u32> = HashSet::new();
        let split: Vec<&str> = line.split('|').collect();
        let match_iter = nums.find_iter(split[0]);
        
        for m in match_iter {
            wins.insert(m.as_str().parse().unwrap());
        }
        
        let mut matches = 0;
        
        for m in nums.find_iter(split[1]) {
            println!("Matched with: {}", m.as_str());
            if wins.contains(&m.as_str().parse().unwrap()) {
                matches += 1;
            }
        }
        
        card_vec.push(
            Card {
                matches,
                copies: 1,
            }
        );
    }
    
    for i in 0..card_vec.len() {
        for j in 1..=card_vec[i].matches {
            let copies = card_vec[i].copies;
            if let Some(card) = card_vec.get_mut(i+j) {
                card.copies += copies;
            }
        }
    }

    println!("{:?}", card_vec);
    
    card_vec.iter().map(|card| card.copies).sum()
}

#[cfg(test)]
mod tests {
    use crate::day4::{part_1, part_2};
    
    #[test]
    fn part_1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_1(input), 13)
    }

    #[test]
    fn part_2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_2(input), 30)
    }
    
    #[test]
    fn part_2_test_2() {
        let input = "Card 1: 1 2 3 4 | 1 2 3 4
Card 2: 0 0 1 0 | 1 2 3 4
Card 2: 0 0 0 0 | 1 1 1 1
Card 2: 0 0 0 0 | 1 1 1 1
Card 2: 0 0 0 0 | 1 1 1 1";

        assert_eq!(part_2(input), 10)
    }
}
