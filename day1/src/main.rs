use std::fs;
use regex::Regex;
use aho_corasick::AhoCorasick;
// Part 1:
// On each line, the calibration value can be found by combining 
// the first digit and the last digit (in that order) 
// to form a single two-digit number.
// What is the sum of all of the calibration values?
//
fn part_1(input: &str) {
    let re = Regex::new(r"\d").unwrap();
    let mut calibration_values: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        let digits: Vec<&str> = re.find_iter(line).map(|d| d.as_str()).collect();
        
        if !digits.is_empty() {
            let calibration_value = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            calibration_values.push(calibration_value.parse::<u32>().unwrap());
        }
    }

    let sum: u32 = calibration_values.iter().sum();
    println!("Part 1: SUM(Calibration Values): {}", sum);
}

// Part 2:
// Your calculation isn't quite right. It looks like some of the 
// digits are actually spelled out with letters: one, two, three, 
// four, five, six, seven, eight, and nine also count as valid "digits".
//
fn parse_chunk(chunk: &str) -> Option<&str> {
    match chunk.to_lowercase().as_str() {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => Some(chunk),
    }
}

fn part_2(input: &str) {
    let re = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let ac = AhoCorasick::new(re).unwrap();
    let mut calibration_values: Vec<usize> = Vec::new();
    
    for line in input.split("\n") {
        let mut digits: Vec<&str> = Vec::new(); 
        for m in ac.find_overlapping_iter(line) {
            let digit: &str = re.get(m.pattern().as_usize()).unwrap();
            digits.push(parse_chunk(digit).unwrap());
        }

        if !digits.is_empty() {
            let calibration_value = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            calibration_values.push(calibration_value.parse::<usize>().unwrap());
        }
    }

    let sum: usize = calibration_values.iter().sum();
    println!("Part 2: SUM(Calibration Values): {}", sum);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file");
    
    part_1(&input);
    part_2(&input);
}
