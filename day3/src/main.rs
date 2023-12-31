use lazy_static::lazy_static;
use ndarray::{arr2, Array2};
use std::{fs, collections::{HashMap, HashSet}};
use regex::Regex;

lazy_static! {
    static ref INPUT: String = fs::read_to_string("input.txt").expect("Can't open input file.");
}

#[derive(Clone, Copy)]
struct Number {
    start: usize,
    end: usize,
    value: i32
}

fn part_1(input: &str) -> i32 {
    let r = Regex::new(r"\d+").unwrap();
    let operations = vec![-1, 0, 1];
    let mut symbol_ams: HashSet<Array2<i32>> = HashSet::new();
    let mut numbers: HashMap<Array2<i32>, Number> = HashMap::new();
    let mut results: HashMap<Array2<i32>, i32> = HashMap::new();
    let mut line_index: usize = 0;

    for line in input.lines() {
        // Map out all coordinates for all numbers
        for cap in r.captures_iter(line) {
            let cap = cap.get(0).unwrap();
            let number = Number {
                start: cap.start(), 
                end: cap.end(), 
                value: cap.as_str().parse().unwrap()
            };
            
            for i in number.start..number.end {
                let coords = arr2(&[[line_index as i32], [i as i32]]);
                numbers.insert(coords, number);
            }
        }

        // Map out all coordinates for all symbol adjacency matrices
        for (i, c) in line.char_indices() {
            if !c.is_digit(10) && c != '.' {
                let coords = arr2(&[[line_index as i32], [i as i32]]);
                
                for &op1 in &operations {
                    for &op2 in &operations {
                        symbol_ams.insert(arr2(&[[coords[(0, 0)] + op1], [coords[(1, 0)] + op2]]));
                    }
                }
            }
        }

        line_index += 1;
    }

    line_index = 0;
    // Add numbers to results by checking 
    // if any digit of the number is within the symbol adjacency matrix
    for line in input.lines() {
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                let coords = arr2(&[[line_index as i32], [i as i32]]);
                if symbol_ams.get(&coords).is_some() {
                    let number = numbers.get(&coords).unwrap();
                    results.insert(arr2(&[[line_index as i32], [number.start as i32]]), number.value);
                }
            }
        }

        line_index += 1;
    }

    results.values().sum()
}

fn part_2(input: &str) -> i32 {
    let r = Regex::new(r"\d+").unwrap();
    let operations = vec![-1, 0, 1];
    let mut symbol_ams: HashMap<Array2<i32>, Array2<i32>> = HashMap::new();
    let mut numbers: HashMap<Array2<i32>, Number> = HashMap::new();
    let mut gear_matches: HashMap<Array2<i32>, Array2<i32>> = HashMap::new();
    let mut results: HashMap<Array2<i32>, i32> = HashMap::new();
    let mut line_index: usize = 0;

    for line in input.lines() {
        // Map out all coordinates for all numbers
        for cap in r.captures_iter(line) {
            let cap = cap.get(0).unwrap();
            let number = Number {
                start: cap.start(), 
                end: cap.end(), 
                value: cap.as_str().parse().unwrap()
            };
            
            for i in number.start..number.end {
                let coords = arr2(&[[line_index as i32], [i as i32]]);
                numbers.insert(coords, number);
            }
        }

        // Map out all coordinates for all symbol adjacency matrices
        for (i, c) in line.char_indices() {
            if c == '*' {
                let coords = arr2(&[[line_index as i32], [i as i32]]);
                
                for &op1 in &operations {
                    for &op2 in &operations {
                        symbol_ams.insert(arr2(&[[coords[(0, 0)] + op1], [coords[(1, 0)] + op2]]), coords.clone());
                    }
                }
            }
        }

        line_index += 1;
    }

    line_index = 0;
    // Add numbers to results by checking 
    // if any digit of the number is within the symbol adjacency matrix
    for line in input.lines() {
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                let coords = arr2(&[[line_index as i32], [i as i32]]);
                let symbol_coords = symbol_ams.get(&coords);
                
                if symbol_coords.is_some() {
                    let number = numbers.get(&coords).unwrap();
                    let number_coords = arr2(&[[line_index as i32], [number.start as i32]]);
                    gear_matches.insert(number_coords.clone(), symbol_coords.unwrap().clone());
                    results.insert(number_coords.clone(), number.value);
                }
            }
        }

        line_index += 1;
    }

    let mut gear_results: HashMap<Array2<i32>, Vec<i32>> = HashMap::new();
    for (number_coords, value) in results.iter() {
        let gear_coords = gear_matches.get(number_coords).unwrap();
        gear_results.entry(gear_coords.clone()).or_insert(vec![*value]).push(*value);
    }

    let mut result: i32 = 0;
    for grv in gear_results.values() {
        if grv.len() == 3 {
            result += grv.first().unwrap() * grv.last().unwrap();
        }
    }

    result
}

fn main() {
    println!("Part 1: {}", part_1(&INPUT));
    println!("Part 2: {}", part_2(&INPUT));
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&INPUT), 550934);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&INPUT), 81997870);
}
