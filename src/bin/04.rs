use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let x: u32 = input
        .split('\n')
        .flat_map(|l| l.split(','))
        .flat_map(|l| l.split('-'))
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .tuples::<(u32, u32, u32, u32)>()
        .map(|(a, b, c, d)| {
            if a <= c && b >= d {
                1
            } else if c <= a && d >= b {
                1
            } else {
                0
            }
        })
        .sum();
    // println!("{:?}", x);

    Some(x)
}

pub fn part_two(input: &str) -> Option<u32> {
    let x: u32 = input
        .split('\n')
        .flat_map(|l| l.split(','))
        .flat_map(|l| l.split('-'))
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .tuples::<(u32, u32, u32, u32)>()
        .map(|(a, b, c, d)| {
            if a <= d && b >= c {
                1
            } else if a >= d && b <= c {
                1
            } else {
                0
            }
        })
        .sum();

    Some(x)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
