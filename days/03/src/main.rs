use std::io::prelude::*;
use std::io::stdin;

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 96;
    }

    c as u32 - 64 + 26
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut bag_items = vec![];
    let mut bag_overlap = vec![];
    let mut priorities = vec![];
    for line in input.lines() {
        let front = &line[0..line.len() / 2];
        let back = &line[line.len() / 2..];

        let front = front
            .chars()
            .filter(|c| back.contains(*c))
            .collect::<std::collections::HashSet<char>>();

        let priority_total = front.iter().map(|c| priority(*c)).sum::<u32>();
        priorities.push(priority_total);

        bag_items.push(line.chars().collect::<std::collections::HashSet<char>>());
    }

    for group in bag_items.chunks(3) {
        let first = &group[0];
        let second = &group[1];
        let third = &group[2];

        let mut char_count = std::collections::HashMap::new();
        for c in first.iter() {
            char_count
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        for c in second.iter() {
            char_count
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        for c in third.iter() {
            char_count
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let bag_match = char_count
            .into_iter()
            .find(|(_k, v)| *v == 3)
            .map(|(k, _v)| priority(*k))
            .unwrap();

        bag_overlap.push(bag_match);
    }

    println!("Priority Sum - {}", priorities.iter().sum::<u32>());
    println!("Overlap Sum - {}", bag_overlap.iter().sum::<u32>());

    Ok(())
}
