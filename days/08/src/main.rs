use std::io::prelude::*;
use std::io::stdin;

// A tree is visible if it is the tallest tree in at least one of four directions:
// left, right, up, or down
fn visible_trees(grid: &Vec<Vec<u32>>) -> usize {
    let mut visible = 0;

    for (i, v) in grid.iter().enumerate() {
        for (j, tree) in v.iter().enumerate() {
            let row = &v;
            let column = grid
                .iter()
                .map(|row| *row.get(j).unwrap())
                .collect::<Vec<u32>>();

            let up = column[0..i].iter().max().map(|m| m < tree).unwrap_or(true);
            let down = column[i + 1..]
                .iter()
                .max()
                .map(|m| m < tree)
                .unwrap_or(true);
            let left = row[..j].iter().max().map(|m| m < tree).unwrap_or(true);
            let right = row[j + 1..].iter().max().map(|m| m < tree).unwrap_or(true);

            if up || down || left || right {
                visible += 1
            }
        }
    }

    visible
}

fn scene_direction(height: u32, dir: &[u32], rev: bool) -> usize {
    let mut count = 0;

    // In a column or row orientation, you need to walk backwards from your current
    // position.
    if rev {
        for t in dir.iter().rev() {
            count += 1;

            if *t >= height {
                return count;
            }
        }
    } else {
        for t in dir.iter() {
            count += 1;

            if *t == height {
                return count;
            }
        }
    };

    count
}

fn near_visibility(grid: &Vec<Vec<u32>>) -> u32 {
    let mut visibilities = vec![];
    for (i, v) in grid.iter().enumerate() {
        for (j, tree) in v.iter().enumerate() {
            let row = &v;
            let column = grid
                .iter()
                .map(|row| *row.get(j).unwrap())
                .collect::<Vec<u32>>();

            let up = scene_direction(*tree, &column[0..i], true);
            let down = scene_direction(*tree, &column[i + 1..], false);
            let left = scene_direction(*tree, &row[..j], true);
            let right = scene_direction(*tree, &row[j + 1..], false);

            visibilities.push(up * right * down * left);
        }
    }

    *visibilities.iter().max().unwrap() as u32
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    let mut grid = vec![];
    for line in input.trim().lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    grid
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let visible_tree_count = visible_trees(&parse_grid(&input));
    let local_visibility = near_visibility(&parse_grid(&input));

    println!("There are {} visible trees", visible_tree_count);
    println!("{} is the highest visibility", local_visibility);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
30373
25512
65332
33549
35390
"#;
    #[test]
    fn parses_input() {
        parse_grid(&INPUT);
    }

    #[test]
    fn counts_visible_trees() {
        assert_eq!(visible_trees(&parse_grid(&INPUT)), 21);
    }

    #[test]
    fn counts_nearby_visibility() {
        assert_eq!(near_visibility(&parse_grid(&INPUT)), 8);
    }
}
