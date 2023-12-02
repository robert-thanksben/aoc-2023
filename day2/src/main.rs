use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Color {
    Red,
    Green,
    Blue
}

lazy_static! {
    static ref INPUT: String = fs::read_to_string("input.txt").expect("Can't read input file");
    static ref GAME_RE: Regex = Regex::new(r"Game (\d+): (.*)").unwrap();
    static ref COLOR_RE: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    static ref CUBE_RX: HashMap<Color, u32> = HashMap::from([(Color::Red, 12), 
                                                             (Color::Green, 13), 
                                                             (Color::Blue, 14)]);
}

fn part_1(input: &str) -> u32 {
    let mut game_count = 0;

    for cap in GAME_RE.captures_iter(input) {
        let game_number: u32 = cap[1].parse().unwrap();
        let mut color_counts: HashMap<Color, u32> = HashMap::new();
        let mut update = true;

        for color_cap in COLOR_RE.captures_iter(&cap[2]) {
            let count: u32 = color_cap[1].parse().unwrap();
            let color = Color::from_str(&color_cap[2]).unwrap();
            let entry = color_counts.entry(color).or_insert(count);

            if *entry < count {
                *entry = count;
            }
        }
        
        for (cc_key, cc_value) in color_counts.iter() {
            if CUBE_RX.get(&cc_key).unwrap() < cc_value {
                update = false;
                break;
            }
        }

        if update {
            game_count += game_number;
        }
    }
    
    game_count
}

fn part_2(input: &str) -> u32 {
    let mut sum_power_set: u32 = 0;

    for cap in GAME_RE.captures_iter(input) {
        let mut color_counts: HashMap<Color, u32> = HashMap::new();

        for color_cap in COLOR_RE.captures_iter(&cap[2]) {
            let count: u32 = color_cap[1].parse().unwrap();
            let color = Color::from_str(&color_cap[2]).unwrap();
            let entry = color_counts.entry(color).or_insert(count);

            if *entry < count {
                *entry = count;
            }
        }

        sum_power_set += color_counts.values().product::<u32>();
    }
    
    sum_power_set
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
