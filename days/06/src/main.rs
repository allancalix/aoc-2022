use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdin;

fn all_unique(input: &str) -> bool {
    let mut seen = HashSet::new();

    for c in input.chars() {
        if seen.contains(&c) {
            return false;
        }

        seen.insert(c);
    }

    true
}

fn detect_marker(input: &str) -> usize {
    let mut prev_chunk = input.chars().take(13).collect::<String>();

    for (i, c) in input.chars().skip(13).enumerate() {
        prev_chunk.push(c);

        if all_unique(prev_chunk.as_str()) {
            return i + 14;
        }

        prev_chunk.remove(0);
    }

    unreachable!("no marker found")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    println!("Marker start - {}", detect_marker(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_marker() {
        assert_eq!(
            detect_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            5,
        );

        assert_eq!(
            detect_marker("nppdvjthqldpwncqszvftbrmjlhg"),
            6,
        );

        assert_eq!(
            detect_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10,
        );

        assert_eq!(
            detect_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11,
        );
    }
}
