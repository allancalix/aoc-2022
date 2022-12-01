const INPUT: &'static str = include_str!("../data/a.txt");

fn main() {
    let mut lines = INPUT.lines();
    let mut payloads = vec![];
    let mut elf: Vec<&'static str> = vec![];

    while let Some(line) = lines.next() {
        if line == "" {
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
}
