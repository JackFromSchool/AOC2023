use regex::Regex;

pub fn part_1(str: &str) -> u32 {
    let re = Regex::new("Game [0-9]+: ").unwrap();

    let mut games: u32 = 0;
    
    for (i, line) in str.lines().enumerate() {
        let items = re.replace(line, "");
        let mut possible = true;
        
        for token in items.replace(";", ",").split(",") {
            
            if token.contains("red") {
                let num = token.replace(" red", "").trim().parse::<u32>().unwrap();
                if num > 12 { possible = false; }
            } else if token.contains("green") {
                let num = token.replace(" green", "").trim().parse::<u32>().unwrap();
                if num > 13 { possible = false; }
            } else {
                let num = token.replace(" blue", "").trim().parse::<u32>().unwrap();
                if num > 14 { possible = false; }
            }
        }
        
        if possible {
            games += i as u32 + 1;
        }
    } 
    
    games
}

#[derive(Default)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

impl Colors {

    pub fn power(&self) -> u32 {
        self.red*self.blue*self.green
    }
}

pub fn part_2(str: &str) -> u32 {
    let re = Regex::new("Game [0-9]+: ").unwrap();

    let mut games: u32 = 0;
    
    for (i, line) in str.lines().enumerate() {
        let items = re.replace(line, "");
        
        let mut colors = Colors::default();
        
        for token in items.replace(";", ",").split(",") {
            
            if token.contains("red") {
                let num = token.replace(" red", "").trim().parse::<u32>().unwrap();
                if num > colors.red {
                    colors.red = num;
                }
            } else if token.contains("green") {
                let num = token.replace(" green", "").trim().parse::<u32>().unwrap();
                if num > colors.green {
                    colors.green = num;
                }
            } else {
                let num = token.replace(" blue", "").trim().parse::<u32>().unwrap();
                if num > colors.blue {
                    colors.blue = num;
                }
            }
        }

        games += colors.power();
    } 
    
    games
}

#[cfg(test)]
mod tests {
    use crate::day2::{part_1, part_2};
    
    #[test]
    fn part_1_test() {
        let input =
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_1(input), 8)
    }

    #[test]
    fn part_2_test() {
        let input =
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_2(input), 2286)
    }
}
