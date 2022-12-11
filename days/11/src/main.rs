use std::collections::{HashMap, VecDeque};
use std::io::prelude::*;
use std::io::stdin;

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    initial_items: VecDeque<i32>,
    op: Expr,
    test: Test,
}

#[derive(Debug, PartialEq, Eq)]
struct Test {
    divisor: i32,
    branch: Branch,
}

impl Test {
    fn check(&self, n: i32) -> usize {
        if n % self.divisor == 0 {
            return self.branch.pass;
        }

        return self.branch.fail;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Branch {
    pass: usize,
    fail: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Multiply(Value, Value),
    Add(Value, Value),
}

impl Expr {
    fn apply(&self, old: i32) -> i32 {
        match self {
            Expr::Multiply(Value::Old, Value::Static(x))
            | Expr::Multiply(Value::Static(x), Value::Old) => old * x,
            Expr::Multiply(Value::Old, Value::Old) => old * old,
            Expr::Add(Value::Old, Value::Static(x)) | Expr::Add(Value::Static(x), Value::Old) => {
                old + x
            }
            Expr::Add(Value::Old, Value::Old) => old + old,
            _ => panic!("muliplying two statics is not supported"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Old,
    Static(i32),
}

impl std::convert::TryFrom<&str> for Value {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(match input {
            "old" => Value::Old,
            v => Value::Static(v.parse::<i32>().unwrap()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Simulator {
    monkies: VecDeque<Monkey>,
    monkey_inspections: HashMap<usize, usize>,
    round: usize,
}

impl Simulator {
    fn new(monkies: VecDeque<Monkey>) -> Self {
        Self {
            monkies,
            round: 0,
            monkey_inspections: HashMap::new(),
        }
    }

    fn round(&mut self) {
        for i in 0..self.monkies.len() {
            while let Some(item) = self.monkies[i].initial_items.pop_front() {
                *self.monkey_inspections.entry(i).or_insert(0) += 1;

                // Part 1
                // let item = self.monkies[i].op.apply(item) / 3;
                let item = self.monkies[i].op.apply(item);
                let next = self.monkies[i].test.check(item);
                self.monkies[next].initial_items.push_back(item);
            }
        }
    }
}

fn parse_op(def: &str) -> Expr {
    let re = regex::Regex::new(r"Operation: new = (old|\d+) ([+*]) (old|\d+)").unwrap();
    let mut cap = re.captures_iter(def).next();

    let first = Value::try_from(cap.as_ref().unwrap().get(1).unwrap().as_str()).unwrap();

    let op = cap.as_ref().unwrap().get(2).unwrap().as_str();

    let second = Value::try_from(cap.as_ref().unwrap().get(3).unwrap().as_str()).unwrap();

    match op {
        "+" => Expr::Add(first, second),
        "*" => Expr::Multiply(first, second),
        v => panic!("unknown operation {}", v),
    }
}

fn parse_test(input: &mut std::str::Lines) -> Test {
    let re = regex::Regex::new(r"Test: divisible by (\d+)").unwrap();
    let mut cap = re.captures_iter(input.next().unwrap()).next();
    let divisor = cap
        .as_ref()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();

    let re = regex::Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap();
    let mut cap = re.captures_iter(input.next().unwrap()).next();
    let pass = cap
        .as_ref()
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let mut cap = re.captures_iter(input.next().unwrap()).next();
    let fail = cap
        .as_ref()
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    Test {
        divisor,
        branch: Branch { pass, fail },
    }
}

fn parse_monkey(input: &mut std::str::Lines) -> Option<Monkey> {
    if input.next().is_none() {
        return None;
    }

    let re = regex::Regex::new(r"Starting items: (.+)").unwrap();
    let initial_items = re
        .captures_iter(&input.next().unwrap().trim())
        .next()
        .as_ref()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let op = parse_op(&input.next().unwrap().trim());
    let test = parse_test(input);

    Some(Monkey {
        initial_items,
        op,
        test,
    })
}

fn parse(input: &str) -> VecDeque<Monkey> {
    let mut input = input.trim().lines();

    let mut monkies = VecDeque::new();
    while let Some(monkey) = parse_monkey(&mut input) {
        monkies.push_back(monkey);
        // Consume newline.
        input.next();
    }

    monkies
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut sim = Simulator::new(parse(&input));
    for _ in 0..10_000 {
        sim.round();
    }

    let mut inspections: Vec<usize> = sim.monkey_inspections.iter().map(|(_k, v)| *v).collect();
    inspections.sort();
    println!(
        "{:?}",
        inspections.iter().rev().take(2).fold(1, |acc, x| acc * x)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    // fn parses_input() {
    //     assert_eq!(
    //         parse(&INPUT),
    //         vec![
    //             Monkey {
    //                 initial_items: vec![79, 98],
    //                 op: Expr::Multiply(Value::Old, Value::Static(19)),
    //                 test: Test {
    //                     divisor: 23,
    //                     branch: Branch { pass: 2, fail: 3 }
    //                 },
    //             },
    //             Monkey {
    //                 initial_items: vec![54, 65, 75, 74],
    //                 op: Expr::Add(Value::Old, Value::Static(6)),
    //                 test: Test {
    //                     divisor: 19,
    //                     branch: Branch { pass: 2, fail: 0 }
    //                 },
    //             },
    //             Monkey {
    //                 initial_items: vec![79, 60, 97],
    //                 op: Expr::Multiply(Value::Old, Value::Old),
    //                 test: Test {
    //                     divisor: 13,
    //                     branch: Branch { pass: 1, fail: 3 }
    //                 },
    //             },
    //             Monkey {
    //                 initial_items: vec![74],
    //                 op: Expr::Add(Value::Old, Value::Static(3)),
    //                 test: Test {
    //                     divisor: 17,
    //                     branch: Branch { pass: 0, fail: 1 }
    //                 },
    //             },
    //         ]
    //     );
    // }
    #[test]
    fn test_input() {
        let mut sim = Simulator::new(parse(&INPUT));
        for _ in 0..5 {
            sim.round();
        }

        let mut inspections: Vec<usize> = sim.monkey_inspections.iter().map(|(_k, v)| *v).collect();
        inspections.sort();
        println!(
            "{:?}",
            inspections.iter().rev().take(2).fold(1, |acc, x| acc * x)
        );
    }
}
