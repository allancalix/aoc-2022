use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::stdin;

#[derive(Debug)]
struct Assignment(u32, u32);

impl Assignment {
    fn sections(&self) -> HashSet<u32> {
        (self.0..=self.1).collect()
    }
}

impl TryFrom<&str> for Assignment {
    type Error = ();

    fn try_from(value: &str) -> Result<Assignment, ()> {
        let mut split = value.split('-');

        Ok(Assignment(
            split.next().unwrap().parse::<u32>().unwrap(),
            split.next().unwrap().parse::<u32>().unwrap(),
        ))
    }
}

fn parse(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(','))
        .map(|assignment| Assignment::try_from(assignment).unwrap())
        .collect::<Vec<Assignment>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let assignments = parse(&input);
    let mut count: usize = 0;
    let mut any_overlap: usize = 0;
    for pair in assignments.chunks(2) {
        let first = &pair[0].sections();
        let second = &pair[1].sections();

        if first.is_superset(second) || second.is_superset(first) {
            count += 1;
        }

        if !first.is_disjoint(second) {
            any_overlap += 1;
        }
    }

    println!("Overlapped assignments - {}", count);
    println!("Any assignments overlap - {}", any_overlap);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn parse_input() {
        parse(INPUT);
    }

    #[test]
    fn assignment_section_hashset() {
        let mut expected = std::collections::HashSet::new();
        for i in 0..=5 {
            expected.insert(i);
        }

        assert_eq!(Assignment(0, 5).sections(), expected);
    }
}
