use std::ops::Range;
use regex::Regex;

#[derive(Debug)]
struct RangeAdjust {
    adjust: u64,
    range: Range<u64>,
}

impl RangeAdjust {
    
    fn new(adjust: u64, range: Range<u64>) -> Self {
        Self {
            adjust,
            range,
        }
    }

    fn adjust_num(&self, num: u64) -> Option<u64> {
        if self.range.contains(&num) {
            Some((num - self.range.start) + self.adjust)
        } else {
            None
        }
    }
    
}

#[derive(Debug)]
struct RangeMap {
    label: String,
    ranges: Vec<RangeAdjust>
}

impl RangeMap {

    fn new(label: String) -> Self {
        Self {
            label,
            ranges: Vec::new(),
        }
    }

    fn add(&mut self, adjust: RangeAdjust) {
        self.ranges.push(adjust);
    }

    fn map(&self, num: u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(val) = range.adjust_num(num) {
                return val;
            } else {
                continue;
            }
        }
        
        return num;
    }
    
}

fn parse_map(input: std::str::Lines) -> Vec<RangeMap> {
    let nums = Regex::new("[0-9]+").unwrap();
    let mut range_maps: Vec<RangeMap> = Vec::new();
    
    for line in input {
        if line.is_empty() {
            continue;
        } else if line.contains(':') {
            range_maps.push(RangeMap::new(line.replace(" map:", "")));
            continue;
        }

        let mut m = nums.find_iter(line);
        let adjust: u64 = m.next().unwrap().as_str().parse().unwrap();
        let start: u64 = m.next().unwrap().as_str().parse().unwrap();
        let go: u64 = m.next().unwrap().as_str().parse().unwrap();

        range_maps.last_mut().unwrap()
            .add(RangeAdjust::new(adjust, start..start+go));
    }

    range_maps
}

pub fn part_1(input: &str) -> u64 {
    let nums = Regex::new("[0-9]+").unwrap();
    let mut seeds: Vec<u64> = Vec::new();
    let mut lines = input.lines();
    
    for m in nums.find_iter(lines.next().unwrap()) {
        seeds.push(m.as_str().parse().unwrap());
    }
    
    let range_maps = parse_map(lines);

    range_maps.iter()
        .for_each(|rm| {
            seeds = seeds.iter()
                .map(|x| rm.map(*x))
                .collect();
        });
    
    
    seeds.into_iter().min().unwrap()
}

pub fn part_2(input: &str) -> u64 {
    let nums = Regex::new("[0-9]+").unwrap();
    let mut seeds: Vec<u64> = Vec::new();
    let mut lines = input.lines();

    // Funny solution that gets the answer after letting it run for an hour or so
    // I know how to do it properly but I'll fix it later 
    
    let mut  m_iter = nums.find_iter(lines.next().unwrap());
    loop {
        if let Some(m) = m_iter.next() {
            let start: u64 = m.as_str().parse().unwrap();
            let step: u64 = m_iter.next().unwrap().as_str().parse().unwrap();

            for i in start..(start+step) {
                seeds.push(i);
            }
        } else {
            break;
        }
    }
    
    let range_maps = parse_map(lines);

    range_maps.iter()
        .for_each(|rm| {
            seeds = seeds.iter()
                .map(|x| rm.map(*x))
                .collect();
        });
    
    
    seeds.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day5::{part_1, part_2};
    
    #[test]
    fn part_1_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        
        assert_eq!(part_1(input), 35)
    }

    #[test]
    fn part_2_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(part_2(input), 46)
    }
}
