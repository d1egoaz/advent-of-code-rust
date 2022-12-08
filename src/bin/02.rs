use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let x = input
        .split('\n')
        .flat_map(|l| l.split(' '))
        .filter(|l| !l.is_empty())
        .tuples::<(&str, &str)>()
        .map(|t| match t {
            // draw
            ("A", "X") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Z") => 3 + 3,
            // win
            ("A", "Y") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            // lose
            (_, "X") => 1,
            (_, "Y") => 2,
            (_, "Z") => 3,
            (&_, _) => 0,
        })
        .sum::<u32>();

    Some(x)
}

pub fn part_two(input: &str) -> Option<u32> {
    // X for Rock, Y for Paper, and Z for Scissors.
    // A for Rock, B for Paper, and C for Scissors.

    // X means you need to lose,
    // Y means you need to end the round in a draw, and
    // Z means you need to win. Good luck!"

    let x = input
        .split('\n')
        .flat_map(|l| l.split(' '))
        .filter(|l| !l.is_empty())
        .tuples::<(&str, &str)>()
        .map(|t| match t {
            // lose X
            ("A", "X") => 3,
            ("B", "X") => 1,
            ("C", "X") => 2,
            // draw Y
            ("A", "Y") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 3 + 3,
            // win Z
            ("A", "Z") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 1 + 6,
            (&_, _) => 0,
        })
        .sum::<u32>();

    Some(x)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
