use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref INPUT: String = fs::read_to_string("input.txt").expect("Can't read input file");
}

fn part_1(input: &str) -> u32 {
    let max_cube_counts: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let game_re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let color_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut game_count = 0;

    for cap in game_re.captures_iter(input) {
        let game_number: u32 = cap[1].parse().unwrap();
        let content = &cap[2];

        let mut color_counts: HashMap<String, u32> = HashMap::new();

        for color_cap in color_re.captures_iter(content) {
            let count: u32 = color_cap[1].parse().unwrap();
            let color = color_cap[2].to_string();
            let entry = color_counts.entry(color).or_insert(count);

            if *entry < count {
                *entry = count;
            }
        }
        
        let mut update = true;

        for (cc_key, cc_value) in color_counts.iter() {
            if max_cube_counts.get(cc_key as &str).unwrap() < cc_value {
                update = false;
            }

        }

        if update {
            game_count += game_number;
        }
    }
    
    game_count
}

fn part_2(input: &str) -> u32 {
    let game_re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let color_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut game_count: u32 = 0;

    for cap in game_re.captures_iter(input) {
        let content = &cap[2];

        let mut color_counts: HashMap<String, u32> = HashMap::new();

        for color_cap in color_re.captures_iter(content) {
            let count: u32 = color_cap[1].parse().unwrap();
            let color = color_cap[2].to_string();
            let entry = color_counts.entry(color).or_insert(count);

            if *entry < count {
                *entry = count;
            }
        }

        game_count += color_counts.values().product::<u32>();
    }
    
    game_count
}

fn main() {
    println!("Part 1: {}", part_1(&INPUT));
    println!("Part 2: {}", part_2(&INPUT));
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&INPUT), 2439);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&INPUT), 63711);
}
