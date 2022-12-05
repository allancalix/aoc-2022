use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::stdin;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::{many0, many_m_n},
    sequence::{delimited, terminated},
    IResult,
};

fn container(input: &str) -> IResult<&str, Option<char>> {
    alt((
        map(delimited(tag("["), anychar, tag("]")), Some),
        map(many_m_n(3, 3, tag(" ")), |_| None),
    ))(input)
}

fn stack_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    many0(terminated(container, many_m_n(0, 1, tag(" "))))(input)
}

type Stack = Vec<char>;

#[derive(Debug)]
struct Layout {
    stacks: Vec<Stack>,
}

impl Layout {
    fn new(num: usize) -> Self {
        let mut stacks = vec![];

        for _ in 0..num {
            stacks.push(vec![]);
        }

        Self { stacks }
    }

    fn add(&mut self, to: usize, value: char) {
        self.stacks[to].push(value);
    }

    #[allow(dead_code)]
    fn cmd_move(&mut self, cmd: Command) {
        for _ in 0..cmd.count {
            let swap = self.stacks[cmd.from].pop();

            if let Some(swap) = swap {
                self.stacks[cmd.to].push(swap);
            }
        }
    }

    #[allow(dead_code)]
    fn cmd_bulk_move(&mut self, cmd: Command) {
        let available_count = self.stacks[cmd.from].len();
        let qty = if available_count < cmd.count {
            available_count
        } else {
            cmd.count
        };

        let stack = self.stacks[cmd.from].split_off(available_count - qty);
        self.stacks[cmd.to].extend_from_slice(&stack);
    }

    fn tops(&self) -> String {
        let mut tops = String::new();

        for stack in &self.stacks {
            if let Some(c) = stack.last() {
                tops.push(*c);
            }
        }

        tops
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Command {
    from: usize,
    to: usize,
    count: usize,
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(value: &str) -> Result<Command, ()> {
        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures_iter(value).next();
        let count = caps
            .as_ref()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let from = caps
            .as_ref()
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let to = caps
            .as_ref()
            .unwrap()
            .get(3)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        Ok(Command {
            from: from - 1,
            to: to - 1,
            count,
        })
    }
}

fn parse_stacks(input: &str) -> Layout {
    let mut lines = input.lines();
    let stack_setup = lines.next_back().unwrap();
    let mut stacks = Layout::new(stack_setup.trim().split("   ").count());

    while let Some(line) = lines.next_back() {
        for (i, v) in stack_row(line).unwrap().1.iter().enumerate() {
            if let Some(c) = v {
                stacks.add(i, *c);
            }
        }
    }

    stacks
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| Command::try_from(line).unwrap())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut sections = input.split("\n\n");
    let mut stacks = parse_stacks(sections.next().unwrap());
    let commands = parse_commands(sections.next().unwrap());

    for cmd in commands {
        // Part 1
        // stacks.cmd_move(cmd);
        // Part 2
        stacks.cmd_bulk_move(cmd);
    }

    println!("Tops - {}", stacks.tops());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const STACK: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
"#;
    #[test]
    fn parse_initial_stack() {
        parse_stacks(STACK);
    }

    fn parse_row() {
        assert_eq!(
            stack_row("[N] [C]    ").unwrap().1,
            vec![Some('N'), Some('C'), None],
        );

        assert_eq!(
            stack_row("    [D]    ").unwrap().1,
            vec![None, Some('D'), None],
        );
    }

    fn parse_commands() {
        assert_eq!(
            Command::try_from("move 1 from 2 to 1").unwrap(),
            Command {
                from: 2,
                to: 1,
                count: 1,
            },
        );
    }
}
