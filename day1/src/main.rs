use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref INPUT: String = fs::read_to_string("input.txt").expect("Can't read file");
}

fn push_value(calibration_values: &mut Vec<u32>, digits: &Vec<&str>) {
    if !digits.is_empty() {
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        let value = format!("{first_digit}{last_digit}").parse::<u32>().unwrap();

        calibration_values.push(value);
    }
}

fn part_1(input: &str) -> u32 {
    let re = Regex::new(r"\d").unwrap();
    let mut calibration_values: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        let digits: Vec<&str> = re.find_iter(line).map(|d| d.as_str()).collect();
        push_value(&mut calibration_values, &digits);
    }

    calibration_values.iter().sum()
}

fn part_2(input: &str) -> u32 {
    let patterns = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", 
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let ac = AhoCorasick::new(patterns).unwrap();
    let mut calibration_values: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        let digits: Vec<&str> = ac
            .find_overlapping_iter(line)
            .map(|mat| match patterns[mat.pattern()].as_ref() {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                digit => digit,
            })
            .collect();

        push_value(&mut calibration_values, &digits);
    }

    calibration_values.iter().sum()
}

fn main() {
    println!("Part 1: {}", part_1(&INPUT));
    println!("Part 2: {}", part_2(&INPUT));
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&INPUT), 54634);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&INPUT), 53855);
}

