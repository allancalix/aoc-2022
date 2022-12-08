use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::stdin;
use std::path::{Component, Components, Path, PathBuf};

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Box<Directory>>,
    children: Vec<Node>,
}

impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
        }
    }

    fn tree_size(&self) -> usize {
        self.children
            .iter()
            .map(|node| match node {
                Node::Directory(d) => d.tree_size(),
                Node::File(f) => f.size,
            })
            .sum()
    }

    fn size(&self) -> usize {
        self.children
            .iter()
            .map(|node| match node {
                Node::File(f) => f.size,
                _ => 0,
            })
            .sum()
    }

    fn print(&self, level: usize, required_space: usize, sum: &mut usize, acc: &mut Vec<(usize, String)>) {
        let prefix = (0..level).map(|_| '\t').collect::<String>();
        println!("{}Dir Start - {} {}", prefix, self.name, self.size());

        let mut dir_size = 0;
        for child in self.children.iter() {
            match child {
                Node::File(f) => {
                    println!("{}File - {}", prefix, f.name);
                    dir_size += f.size;
                }
                Node::Directory(d) => {
                    dir_size += d.tree_size();
                    d.print(level + 1, required_space, sum, acc)
                },
            }
        }

        if dir_size <= 100_000 {
            *sum += dir_size;
        }

        if dir_size >= required_space {
            acc.push((dir_size, self.name.clone()));
        }
    }

    fn add_node(&mut self, mut components: Components, node: Node) {
        // println!("components - {:?}, dir - {:?}", &components, self);
        if let Some(component) = components.next() {
            let dir = self
                .children
                .iter_mut()
                .find(|node| {
                    match (node.name(), component) {
                        ("/", Component::RootDir) => true,
                        (name, Component::Normal(os)) => os.to_str().unwrap() == name,
                        // Path not found error.
                        _ => unreachable!(),
                    }
                })
                .unwrap();

            return match dir {
                Node::Directory(dir) => dir.add_node(components, node),
                Node::File(_) => panic!("cannot add node to file"),
            };
        }

        self.children.push(node);
    }
}

#[derive(Debug)]
enum Node {
    File(File),
    Directory(Box<Directory>),
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::File(f) => f.name.as_str(),
            Node::Directory(d) => d.name.as_str(),
        }
    }
}

#[derive(Debug)]
struct File {
    size: usize,
    name: String,
}

impl TryFrom<&str> for File {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

struct DirectoryBuilder {
    /// Current shell directory context.
    context: PathBuf,
    directory: Directory,
}

impl DirectoryBuilder {
    fn new() -> Self {
        Self {
            context: "/".into(),
            directory: Directory::new("/"),
        }
    }

    fn set_context(&mut self, dir: &str) {
        match dir {
            "." => (),
            ".." => {
                self.context.pop();
            }
            "/" => self.context = "/".into(),
            p => self.context.push(p),
        }
    }

    fn add_node(&mut self, node: Node) {
        let mut components = self.context.as_path().components();
        // dump root directory.
        components.next().unwrap();
        self.directory.add_node(components, node);
    }

    fn into_directory(self) -> Directory {
        self.directory
    }
}

#[derive(Debug)]
struct Command {
    /// The command input from the user.
    command: String,
    /// The resulting output of the command.
    output: Vec<String>,
}

impl Command {
    fn input(&self) -> (&str, Option<&str>) {
        let mut parts = self.command.split(" ");
        let _marker = parts.next().unwrap();
        let command = parts.next().unwrap();
        let output = parts.next();

        (command, output)
    }

    fn output(&self) -> Vec<String> {
        self.output.clone()
    }
}

fn parse_directory(input: &str) -> Directory {
    unimplemented!()
}

fn parse_output(input: &str) -> Vec<Command> {
    let mut commands = vec![];

    let mut lines = input.trim().lines();

    let mut command = lines.next().unwrap().to_string();
    let mut output: Vec<String> = vec![];
    for line in lines {
        if line.starts_with("$ ") {
            let cmd_out = std::mem::replace(&mut output, vec![]);
            commands.push(Command {
                command,
                output: cmd_out,
            });

            command = line.into();

            continue;
        }

        output.push(line.into());
    }

    commands.push(Command { command, output });

    commands
}

fn parse_shellout(input: &str) -> Directory {
    let commands = parse_output(&input);
    let mut filesystem = DirectoryBuilder::new();

    for cmd in commands {
        match cmd.input() {
            ("cd", Some(input)) => {
                filesystem.set_context(input);
            }
            ("ls", None) => {
                for output in cmd.output.iter() {
                    if output.starts_with("dir ") {
                        let mut output = output.split(" ");
                        output.next().unwrap();
                        filesystem.add_node(Node::Directory(Box::new(Directory::new(
                            output.next().unwrap(),
                        ))));

                        continue;
                    }

                    let mut output = output.split(" ");
                    filesystem.add_node(Node::File(File {
                        size: output.next().unwrap().parse::<usize>().unwrap(),
                        name: output.next().unwrap().into(),
                    }));
                }
            }
            _ => (),
        }
    }

    let fs = filesystem.into_directory();

    let unused_space = 70000000 - fs.tree_size();
    let required_space = 30000000 - unused_space;
    let mut size = 0;
    let mut big_enough_dirs = vec![];
    fs.print(0, required_space, &mut size, &mut big_enough_dirs);
    println!("Unused space - {}", unused_space);
    println!("Required space - {}", required_space);
    println!("Candidates - {:?}", big_enough_dirs.iter().min());
    println!("Tree size - {}", size);

    fs
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    parse_shellout(&input);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn infers_directories() {
        parse_shellout(&INPUT);
    }
}
