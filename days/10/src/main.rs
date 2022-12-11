use std::io::prelude::*;
use std::io::stdin;

enum Command {
    Add(i32),
    Busy,
    Noop,
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(cmd: &str) -> Result<Self, Self::Error> {
        let mut parts = cmd.split(" ");

        match (parts.next(), parts.next()) {
            (Some("addx"), Some(val)) => {
                let val = val.parse::<i32>().unwrap();
                Ok(Command::Add(val))
            }
            (Some("noop"), None) => Ok(Command::Noop),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct VM {
    register: i32,
    next: Option<i32>,
    pending: Option<i32>,
    cycle: usize,
    canvas: Vec<Vec<bool>>,
}

impl VM {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 1,
            next: None,
            pending: None,
            canvas: vec![
                vec![false; 40],
                vec![false; 40],
                vec![false; 40],
                vec![false; 40],
                vec![false; 40],
                vec![false; 40],
            ],
        }
    }

    fn execute(&mut self, cmd: Command) {
        self.next = self.pending.take();
        let sprite_span = std::ops::Range {
            start: self.register - 1,
            end: self.register + 2,
        };

        match cmd {
            Command::Noop => {}
            Command::Busy => {}
            Command::Add(x) => {
                self.pending = Some(x);
                println!(
                    "Cycle - {} {}",
                    self.cycle,
                    self.register * (self.cycle as i32)
                );

                let x = (self.cycle - 1) % 40;
                let y = (self.cycle - 1) / 40;
                println!("{:?}", sprite_span);
                println!("{x}, {y}");
                if sprite_span.contains(&(x as i32)) {
                    self.canvas[y][x] = true;
                }

                self.cycle += 1;

                return self.execute(Command::Busy);
            }
        }

        println!(
            "Cycle - {} {}",
            self.cycle,
            self.register * (self.cycle as i32)
        );

        let x = (self.cycle - 1) % 40;
        let y = (self.cycle - 1) / 40;
        println!("{:?}", sprite_span);
        println!("{x}, {y}");
        if sprite_span.contains(&(x as i32)) {
            self.canvas[y][x] = true;
        }

        self.cycle += 1;
        if let Some(x) = self.next.take() {
            self.register += x;
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Command::try_from(line).unwrap())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut vm = VM::new();
    let cmds = parse(&input);
    for cmd in cmds {
        vm.execute(cmd);
    }

    for y in vm.canvas {
        for x in y {
            if x {
                // print!("#");
                print!(" ");
            } else {
                // print!(".");
                print!("░");
            }
        }
        print!("\n");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn computes_program() {
        let mut vm = VM::new();
        let cmds = parse(&INPUT);
        for cmd in cmds {
            vm.execute(cmd);
        }

        for y in vm.canvas {
            for x in y {
                if x {
                    // print!("#");
                    print!(" ");
                } else {
                    // print!(".");
                    print!("░");
                }
            }
            print!("\n");
        }
        println!("Final value: {:?}", vm.register);
    }
}
