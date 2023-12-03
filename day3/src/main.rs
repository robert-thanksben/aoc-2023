use lazy_static::lazy_static;
use ndarray::{arr2, Array2};
use std::fs;

lazy_static! {
    static ref INPUT: String = fs::read_to_string("input.txt").expect("Can't open input file.");
    static ref TEST_INPUT: String =
        fs::read_to_string("test_input_1.txt").expect("Can't open test input file.");
}

fn part_1(input: &str) -> i32 {
    let operations = vec![-1, 0, 1];
    let mut variations: Vec<Array2<i32>> = Vec::new();
    let mut line_index: usize = 0;
    // let mut numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        for (i, c) in line.char_indices() {
            if !c.is_digit(10) && c != '.' {
                let coordinates = arr2(&[[line_index as i32], [i as i32]]);
                
                for &op1 in &operations {
                    for &op2 in &operations {
                        let variation = arr2(&[[coordinates[(0, 0)] + op1], [coordinates[(1, 0)] + op2]]);
                        variations.push(variation);
                    }
                }
            }
        }

        line_index += 1;
    }

    line_index = 0;
    for line in input.lines() {
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                let coordinates = arr2(&[[line_index as i32], [i as i32]]);
                if variations.contains(&coordinates) {
                    println!("[{line_index}, {i}] = {c}");
                } 
            }
        }

        line_index += 1;
    }

    7
}

fn main() {
    println!("Part 1: {}", part_1(&INPUT));
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&TEST_INPUT), 4361);
}
