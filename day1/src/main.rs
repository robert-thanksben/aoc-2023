use std::fs;
use regex::Regex;
use aho_corasick::AhoCorasick;

fn part_1(input: &str) -> u32 {
    let re = Regex::new(r"\d").unwrap();
    let mut calibration_values: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        let digits: Vec<&str> = re.find_iter(line).map(|d| d.as_str()).collect();
        
        if !digits.is_empty() {
            let calibration_value = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            calibration_values.push(calibration_value.parse::<u32>().unwrap());
        }
    }

    calibration_values.iter().sum()
}

fn part_2(input: &str) -> u32 {
    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
                     "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let ac = AhoCorasick::new(patterns).unwrap();
    let mut calibration_values: Vec<u32> = Vec::new();
    
    for line in input.split("\n") {
        let mut digits: Vec<&str> = Vec::new();

        for m in ac.find_overlapping_iter(line) {
            let matched_digit: &str = patterns.get(m.pattern().as_usize()).unwrap();

            let converted_digit = match matched_digit {
                "one" => Some("1"),
                "two" => Some("2"),
                "three" => Some("3"),
                "four" => Some("4"),
                "five" => Some("5"),
                "six" => Some("6"),
                "seven" => Some("7"),
                "eight" => Some("8"),
                "nine" => Some("9"),
                _ => Some(matched_digit),
            }.unwrap();

            digits.push(converted_digit);
        }

        if !digits.is_empty() {
            let calibration_value = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            calibration_values.push(calibration_value.parse::<u32>().unwrap());
        }
    }

    calibration_values.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file");
    
    let result_1 = part_1(&input);
    println!("Part 1: {}", result_1);

    let result_2 = part_2(&input);
    println!("Part 2: {}", result_2);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("input.txt").expect("Can't read file");
        assert_eq!(part_1(&input), 54634);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("input.txt").expect("Can't read file");
        assert_eq!(part_2(&input), 53855);
    }
}
