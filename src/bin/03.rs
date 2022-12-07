use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let x: u32 = input
        .lines()
        .map(|l| {
            let (c1, c2) = l.split_at(l.len() / 2);
            c1.chars()
                .find(|c1c| c2.chars().any(|c2c| c2c == *c1c))
                .map_or(0, char_to_priority)
        })
        .sum();
    // dbg!(x);
    Some(x)
}

fn char_to_priority(c: char) -> u32 {
    let val = c as u32;
    if val >= 97 { // lowercase
        val - 97 + 1
    } else {
        val - 65 + 27
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let x = input
        .split('\n')
        .tuples::<(&str, &str, &str)>()
        .map(|t| {
            let (c1, c2, c3) = t;
            c1.chars()
                .find(|c1c| {
                    c2.chars()
                        .any(|c2c| c3.chars().any(|c3c| c3c == c2c && c3c == *c1c))
                })
                .map_or(0, char_to_priority)
        })
        .sum();
    Some(x)
}

// vJrwpWtwJgWrhcsFMMfFFhFp
// vJrwpWtwJgWr
// hcsFMMfFFhFp

// vJrwpWtwJgWr
// hcsFMMfFFhFp
// The only item type that appears in both compartments is lowercase p

// jqHRNqRjqzjGDLGL
// rsFMfFZSrLrFZsSL
// The only item type that appears in both compartments is uppercase L.

// The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
// The fourth rucksack's compartments only share item type v.
// The fifth rucksack's compartments only share item type t.
// The sixth rucksack's compartments only share item type s.

// NOTE: priority:
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.

// In the above example, the priority of the item type that appears in both compartments of each
// rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
