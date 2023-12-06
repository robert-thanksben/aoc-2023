use std::{fs, collections::HashMap};
use regex::Regex;
use lazy_static::lazy_static;
use array_tool::vec::Intersect;

lazy_static! {
    static ref INPUT: String = fs::read_to_string("input.txt").expect("Can't read input file");
    static ref NR: Regex = Regex::new(r"\d+").unwrap();
}

fn part_1(input: &str) -> u32 {
    let mut result: u32 = 0;

    for line in input.lines() {
        let card = line.split(":").nth(1).unwrap();
        let winning_num_extracted = card.split("|").nth(0).unwrap();
        let player_num_extracted = card.split("|").nth(1).unwrap();

        let winning_nums: Vec<u16> = NR.find_iter(winning_num_extracted).filter_map(|n| n.as_str().parse().ok()).collect();
        let player_nums: Vec<u16> = NR.find_iter(player_num_extracted).filter_map(|n| n.as_str().parse().ok()).collect();

        let nums_intersect = winning_nums.intersect(player_nums.clone());
        let n = nums_intersect.len();

        if n > 0 {
            result += 2_u32.pow(n as u32 - 1);
        }
    }

    result
}

#[derive(Debug, Clone)]
struct Card {
    id: String,
    matches: usize
}

fn part_2(input: &str) -> usize {
    let mut scratch_cards: Vec<Card> = Vec::new();
    let mut complete_scratch_cards: HashMap<String, Vec<Card>> = HashMap::new();

    for line in input.lines() {
        let card_id = line.split(":").nth(0).unwrap();
        let card_contet = line.split(":").nth(1).unwrap();
        let winning_num_extracted = card_contet.split("|").nth(0).unwrap();
        let player_num_extracted = card_contet.split("|").nth(1).unwrap();

        let winning_nums: Vec<u16> = NR.find_iter(winning_num_extracted).filter_map(|n| n.as_str().parse().ok()).collect();
        let player_nums: Vec<u16> = NR.find_iter(player_num_extracted).filter_map(|n| n.as_str().parse().ok()).collect();

        let nums_intersect = winning_nums.intersect(player_nums.clone());
        let n = nums_intersect.len();

        let card = Card { id: card_id.to_string(), matches: n };
        scratch_cards.push(card);
    }

    let mut i = 1;

    for sc in &scratch_cards {
        let cloned_cards = complete_scratch_cards.entry(sc.id.to_string()).and_modify(|c| c.push(sc.clone())).or_insert(vec![sc.clone()]);
        let ccl = cloned_cards.len();

        if sc.matches > 0 {
            for _n in 0..ccl {
                for scc in &scratch_cards[i..(i+sc.matches)] {
                    complete_scratch_cards.entry(scc.id.to_string()).and_modify(|c| c.push(scc.clone())).or_insert(vec![scc.clone()]);
                }
            }
        }
        i = i+1;
    }

    let mut result: usize = 0;
    for stacks in complete_scratch_cards.values() {
        result += stacks.len();
    }

    result
}

fn main() {
    println!("Part 1: {}", part_1(&INPUT));
    println!("Part 2: {}", part_2(&INPUT));
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&INPUT), 25183);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&INPUT), 5667240);
}

