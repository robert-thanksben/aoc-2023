use std::{collections::HashMap, fs};
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = fs::read_to_string("input.txt").unwrap();
    static ref TEST_INPUT: String = fs::read_to_string("test_input.txt").unwrap(); 
    static ref MAP_NAMES: Vec<String> = vec!["seed-to-soil".to_string(), 
                                             "soil-to-fertilizer".to_string(), 
                                             "fertilizer-to-water".to_string(), 
                                             "water-to-light".to_string(), 
                                             "light-to-temperature".to_string(), 
                                             "temperature-to-humidity".to_string(), 
                                             "humidity-to-location".to_string()];

}

struct Conversion {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64
}

struct Map {
    mappings: Vec<Conversion>,
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    length: i64
}

fn parse_data(input: &str) -> (Vec<i64>, HashMap<String, Map>) {
    let mut current_map_name = String::new();
    let mut maps = HashMap::new();
    let mut seeds: Vec<i64> = Vec::new();

    for line in input.lines() {
        if line.starts_with("seeds:") {
            seeds = line.split(":").nth(1).unwrap().split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
        }
        else if line.ends_with("map:") {
            current_map_name = line.split(":").next().unwrap().to_string();
            current_map_name = current_map_name.replace(" map", "");
        } else if !line.is_empty() {
            let numbers: Vec<i64> = line.split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();

            if numbers.len() == 3 {
                maps.entry(current_map_name.clone())
                    .or_insert_with(|| Map {
                        mappings: Vec::new(),
                    })
                    .mappings.push(Conversion { 
                        destination_range_start: numbers[0], 
                        source_range_start: numbers[1], 
                        range_length: numbers[2] 
                    });
            }
        }
    }

    (seeds, maps)
}

fn part_1(input: &str) -> i64 {
    let (seeds, maps) = parse_data(&input);
    let mut locations: Vec<i64> = Vec::new();

    for seed in seeds {
        let mut source = seed;
        let mut destination = seed;
        
        for map_name in MAP_NAMES.iter() {
            let current_map = maps.get(&map_name as &str).unwrap();

            for mapping in &current_map.mappings {
                if source >= mapping.source_range_start && source < mapping.source_range_start + mapping.range_length {
                    let destination_offset = source - mapping.source_range_start;
                    destination = mapping.destination_range_start + destination_offset;

                    if destination < mapping.destination_range_start + mapping.range_length {
                        break;
                    }
                }
            }

            source = destination;
        }

        locations.push(destination);
    }

    *locations.iter().min().unwrap()
}

fn part_2(input: &str) -> i64 {
    let (seeds, maps) = parse_data(&input);
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    let mut lowest_location: i64 = i64::MAX;

    for seed_chunks in seeds.chunks_exact(2) {
        seed_ranges.push(SeedRange { start: seed_chunks[0], length: seed_chunks[1] });
    }

    for seed_range in seed_ranges.iter() {
        for seed in seed_range.start..seed_range.start+seed_range.length {
            let mut source = seed;
            let mut destination = seed;
        
            for map_name in MAP_NAMES.iter() {
                let current_map = maps.get(&map_name as &str).unwrap();

                for mapping in &current_map.mappings {
                    if source >= mapping.source_range_start && source < mapping.source_range_start + mapping.range_length {
                        let destination_offset = source - mapping.source_range_start;
                        destination = mapping.destination_range_start + destination_offset;

                        if destination < mapping.destination_range_start + mapping.range_length {
                            break;
                        }
                    }
                }

                source = destination;
            }
            
            if destination < lowest_location {
                lowest_location = destination;
            }
        }
    }

    lowest_location
}


fn main() {
    println!("Part 1: {}", part_1(&INPUT));
    println!("Part 2: {}", part_2(&INPUT));
}


#[test]
fn test_part_1() {
    assert_eq!(part_1(&INPUT), 282277027);
}

#[test]
fn test_part_2() {
    // INPUT RESULT = 11554135
    assert_eq!(part_2(&TEST_INPUT), 46);
}
