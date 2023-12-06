use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output_1 = part_1(input);
    println!("part 1: {}", output_1);
    let output_2 = part_2(input);
    println!("part 2: {}", output_2);
}

struct Card {
    id: u32,
    winners: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn from_string(line: &str) -> Card {
        let mut meta = line.split(':');
        let id = meta
            .next()
            .expect("input should be guaranteed in this format")
            .split(' ')
            .last()
            .expect("id should be guaranteed by input")
            .parse::<u32>()
            .expect("id should be guaranteed by input");
        let mut card_contents = meta
            .next()
            .expect("input format should be guaranteed")
            .split('|');
        let winners = card_contents
            .next()
            .expect("winners should be guaranteed")
            .trim()
            .split(' ')
            .map(|w| w.parse::<u32>())
            .filter(|w| w.is_ok())
            .map(|w| w.expect("should be filtered to ok parses"))
            .collect();
        let numbers = card_contents
            .next()
            .expect("numbers should be guaranteed")
            .trim()
            .split(' ')
            .map(|w| w.parse::<u32>())
            .filter(|w| w.is_ok())
            .map(|w| w.expect("should be filtered to ok parses"))
            .collect();
        Card {
            id,
            winners,
            numbers,
        }
    }
}

fn count_matches(card: &Card) -> u32 {
    card.numbers
        .iter()
        .filter(|n| card.winners.contains(n))
        .count() as u32
}

fn part_1(input: &str) -> String {
    let mut total_points = 0;
    for line in input.lines() {
        let card = Card::from_string(line);
        let number_matches = count_matches(&card);
        if number_matches <= 1 {
            total_points += number_matches as u32;
        } else {
            total_points += 2_u32.pow(number_matches as u32 - 1);
        }
    }
    total_points.to_string()
}

fn part_2(input: &str) -> String {
    let mut card_instances = HashMap::new();
    for line in input.lines() {
        let card = Card::from_string(line);
        let copies_of_current_card = card_instances
            .entry(card.id)
            .and_modify(|n| *n += 1)
            .or_insert(1)
            .clone();
        let number_matches = count_matches(&card);
        for i in 1..=number_matches {
            card_instances
                .entry(card.id + i)
                .and_modify(|n| *n += copies_of_current_card)
                .or_insert(copies_of_current_card);
        }
    }
    card_instances.values().sum::<u32>().to_string()
}

#[test]
fn test_example_1() {
    let input = include_str!("./example.txt");
    let output_1 = part_1(input);
    assert_eq!(output_1, "13");
}

#[test]
fn test_example_2() {
    let input = include_str!("./example.txt");
    let output_2 = part_2(input);
    assert_eq!(output_2, "30");
}
