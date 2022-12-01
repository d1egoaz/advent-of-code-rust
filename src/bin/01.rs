pub fn calories_per_elve(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|l| l.lines().filter_map(|i| i.parse::<u32>().ok()).sum())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    calories_per_elve(input).iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x: Vec<u32> = calories_per_elve(input);
    x.sort_by(|a, b| b.cmp(a));
    Some(x.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }

    #[test]
    fn test_input_part_one() {
        let input = advent_of_code::read_file("inputs", 1);
        assert_eq!(part_one(&input), Some(66306));
    }

    #[test]
    fn test_input_part_two() {
        let input = advent_of_code::read_file("inputs", 1);
        assert_eq!(part_two(&input), Some(195292));
    }
}
