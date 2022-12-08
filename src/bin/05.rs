use num_traits::cast::AsPrimitive;
use parse_display::Display;

//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2

#[derive(Debug, Clone, Copy, parse_display::FromStr, Display)]
#[display("move {n} from {source_stack} to {target_stack}")]
struct Movement {
    n: usize,
    source_stack: usize,
    target_stack: usize,
}

fn extract_stacks_movements(input: &str) -> Option<(Vec<Vec<char>>, Vec<Movement>)> {
    let (cargo, moves) = input.split_once("\n\n")?;
    let all_crates: Vec<Vec<char>> = cargo
        .lines()
        .map(|l| l.chars().skip(1).step_by(4).collect())
        .collect();
    let max_stacks = all_crates
        .last()?
        .iter()
        .filter_map(|c| c.to_digit(10))
        .max()?
        .as_();

    let mut stacks = vec![Vec::new(); max_stacks];
    all_crates.iter().rev().for_each(|l| {
        l.iter().enumerate().for_each(|(i, c)| {
            if !c.is_whitespace() {
                stacks[i].push(c.to_owned());
            }
        })
    });
    let movements: Vec<Movement> = moves.lines().filter_map(|l| l.parse().ok()).collect();
    Some((stacks, movements))
}


pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, movements) = extract_stacks_movements(input)?;

    for m in movements {
        for _ in 0..m.n {
            if let Some(top_crate) = stacks[m.source_stack - 1].pop() {
                stacks[m.target_stack - 1].push(top_crate);
            }
        }
    }
    // stacks.iter().for_each(|s| println!("{:?}", s[s.len() - 1]));
    let result = stacks.iter().map(|s| s[s.len() - 1]).collect();
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, movements) = extract_stacks_movements(input)?;

    for m in movements {
        let s_source = &mut stacks[m.source_stack - 1];
        let ulen: &usize = &s_source.len().as_();
        let bulk_crates = s_source.split_off(ulen.checked_sub(m.n)?);
        stacks[m.target_stack - 1].extend(bulk_crates);
    }
    // dbg!(stacks);
    // stacks.iter().for_each(|s| println!("{:?}", s[s.len() - 1]));
    let result = stacks.iter().map(|s| s[s.len() - 1]).collect();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
