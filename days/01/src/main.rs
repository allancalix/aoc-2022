use std::io::prelude::*;
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut payloads = vec![];
    let mut elf: Vec<&str> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            let total: u32 = elf.iter().map(|t| t.parse::<u32>().unwrap()).sum();
            payloads.push(total);
            elf.clear();

            continue;
        }

        elf.push(line);
    }

    // Handle last entry without trailing empty line.
    let total: u32 = elf.iter().map(|t| t.parse::<u32>().unwrap()).sum();
    payloads.push(total);

    println!("Max - {}", payloads.iter().max().unwrap());
    payloads.sort();
    payloads.reverse();
    println!("Top 3 - {}", payloads.iter().take(3).sum::<u32>());

    Ok(())
}
