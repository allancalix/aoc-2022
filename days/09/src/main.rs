use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::stdin;

#[derive(Debug, Eq, PartialEq)]
struct Command {
    direction: Move,
    steps: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Move {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(match input {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("invalid input direction"),
        })
    }
}

#[derive(Debug)]
struct Map {
    head: (i32, i32),
    tail: (i32, i32),
    next: Option<Box<Map>>,
    map: HashSet<(i32, i32)>,
}

impl Map {
    fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            map: HashSet::new(),
            next: None,
        }
    }

    fn n_tails(n: usize) -> Self {
        let mut map = Self {
            head: (0, 0),
            tail: (0, 0),
            map: HashSet::new(),
            next: None,
        };

        if n > 1 {
            map.next = Some(Box::new(Map::n_tails(n - 1)));
        }

        map
    }

    fn knots(&self) -> usize {
        if let Some(next) = &self.next {
            return 1 + next.knots();
        }

        2
    }

    fn tail(&self) -> &Map {
        if let Some(next) = &self.next {
            return next.tail();
        }

        self
    }

    fn move_head(&mut self, cmd: &Command) {
        match cmd.direction {
            Move::Up => {
                let next_y = self.head.1 + cmd.steps;

                for _ in 0..(self.head.1 - next_y).abs() {
                    self.head.1 += 1;
                    self.move_tail();
                }
            }
            Move::Down => {
                let next_y = self.head.1 - cmd.steps;

                for _ in 0..(self.head.1 - next_y).abs() {
                    self.head.1 -= 1;
                    self.move_tail();
                }
            }
            Move::Right => {
                let next_x = self.head.0 + cmd.steps;

                for _ in 0..(self.head.0 - next_x).abs() {
                    self.head.0 += 1;
                    self.move_tail();
                }
            }
            Move::Left => {
                let next_x = self.head.0 - cmd.steps;

                for _ in 0..(self.head.0 - next_x).abs() {
                    self.head.0 -= 1;
                    self.move_tail();
                }
            }
        };
    }

    fn move_tail(&mut self) {
        if self.head.0 != self.tail.0 && self.head.1 != self.tail.1 {
            return self.move_diagonally();
        }

        let x_delta = self.head.0 - self.tail.0;
        let y_delta = self.head.1 - self.tail.1;

        if y_delta > 1 {
            self.tail.1 += 1;
        } else if y_delta < -1 {
            self.tail.1 -= 1;
        } else if x_delta > 1 {
            self.tail.0 += 1;
        } else if x_delta < -1 {
            self.tail.0 -= 1;
        }

        self.map.insert(self.tail);

        if let Some(next) = &mut self.next {
            next.head = self.tail;
            next.move_tail()
        }
    }

    fn move_diagonally(&mut self) {
        let x_delta = self.head.0 - self.tail.0;
        let y_delta = self.head.1 - self.tail.1;

        if y_delta > 1 {
            if x_delta > 0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }

            self.tail.1 += 1;
        } else if y_delta < -1 {
            if x_delta > 0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }
            self.tail.1 -= 1;
        } else if x_delta > 1 {
            if y_delta > 0 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
            self.tail.0 += 1;
        } else if x_delta < -1 {
            if y_delta > 0 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
            self.tail.0 -= 1;
        }

        self.map.insert(self.tail);

        if let Some(next) = &mut self.next {
            next.head = self.tail;
            next.move_tail()
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    let mut cmds = vec![];
    for line in input.lines().filter(|line| !line.is_empty()) {
        let mut parts = line.split(" ");

        cmds.push(Command {
            direction: Move::try_from(parts.next().unwrap()).unwrap(),
            steps: parts.next().unwrap().parse::<i32>().unwrap(),
        });
    }

    cmds
}

// Some ideas here:
// Build a vector of ranges and find intersections
// Build 2 hashsets one for the head and another for the tail
// Build out a 2-D vector space
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let cmds = parse(&input);
    let mut map = Map::n_tails(9);

    for cmd in cmds {
        map.move_head(&cmd);
    }

    println!("Visited locations - {}", &map.map.iter().count());
    println!("Last tail locations - {}", &map.tail().map.iter().count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    #[test]
    fn parses_input() {
        assert_eq!(
            parse(&INPUT),
            vec![
                Command {
                    direction: Move::Right,
                    steps: 4
                },
                Command {
                    direction: Move::Up,
                    steps: 4
                },
                Command {
                    direction: Move::Left,
                    steps: 3
                },
                Command {
                    direction: Move::Down,
                    steps: 1
                },
                Command {
                    direction: Move::Right,
                    steps: 4
                },
                Command {
                    direction: Move::Down,
                    steps: 1
                },
                Command {
                    direction: Move::Left,
                    steps: 5
                },
                Command {
                    direction: Move::Right,
                    steps: 2
                },
            ]
        );
    }

    #[test]
    fn test_input() {
        let cmds = parse(&INPUT);
        let mut map = Map::new();

        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 13);
    }

    #[test]
    fn move_back_and_forth() {
        let cmds = vec![
            Command {
                direction: Move::Down,
                steps: 2,
            },
            Command {
                direction: Move::Down,
                steps: 1,
            },
            Command {
                direction: Move::Up,
                steps: 1,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 3);
    }

    #[test]
    fn moves_laterally() {
        let cmds = vec![Command {
            direction: Move::Down,
            steps: 6,
        }];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 6);
    }

    #[test]
    fn moves_diagonally() {
        let cmds = vec![
            Command {
                direction: Move::Right,
                steps: 1,
            },
            Command {
                direction: Move::Up,
                steps: 2,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 2);
    }

    #[test]
    fn moves_diagonally_left() {
        let cmds = vec![
            Command {
                direction: Move::Up,
                steps: 4,
            },
            Command {
                direction: Move::Left,
                steps: 3,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 6);
    }

    #[test]
    fn moves_diagonally_negative() {
        let cmds = vec![
            Command {
                direction: Move::Left,
                steps: 1,
            },
            Command {
                direction: Move::Down,
                steps: 2,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 2);
    }

    #[test]
    fn diagonal_move_not_far_enough() {
        let cmds = vec![
            Command {
                direction: Move::Left,
                steps: 1,
            },
            Command {
                direction: Move::Down,
                steps: 1,
            },
            Command {
                direction: Move::Up,
                steps: 2,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 1);
    }

    #[test]
    fn reverse_course() {
        let cmds = vec![
            Command {
                direction: Move::Left,
                steps: 1,
            },
            Command {
                direction: Move::Right,
                steps: 4,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 3);
    }

    #[test]
    fn circle() {
        let cmds = vec![
            Command {
                direction: Move::Right,
                steps: 5,
            },
            Command {
                direction: Move::Up,
                steps: 5,
            },
            Command {
                direction: Move::Left,
                steps: 5,
            },
            Command {
                direction: Move::Down,
                steps: 5,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 17);
    }

    #[test]
    fn counter_circle() {
        let cmds = vec![
            Command {
                direction: Move::Left,
                steps: 5,
            },
            Command {
                direction: Move::Down,
                steps: 5,
            },
            Command {
                direction: Move::Right,
                steps: 5,
            },
            Command {
                direction: Move::Up,
                steps: 5,
            },
        ];

        let mut map = Map::new();
        for cmd in cmds {
            map.move_head(&cmd);
        }

        assert_eq!(map.map.iter().count(), 17);
    }

    // 2468 - 1
    // 2472 - 2
    // 2293 - 3 
    #[test]
    fn n_tails() {
        let cmds = parse(
            r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
        );
        let mut map = Map::n_tails(9);

        for cmd in cmds {
            map.move_head(&cmd);
        }

        println!("Total knots - {}", map.knots());
        assert_eq!(map.tail().map.iter().count(), 36);
    }
}
