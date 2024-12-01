use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut split = line.split_whitespace();
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            Some((left.parse::<u32>().ok()?, right.parse::<u32>().ok()?))
        })
        .unzip();

    left.sort();
    right.sort();

    let total = std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();

    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut split = line.split_whitespace();
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            Some((left.parse::<u32>().ok()?, right.parse::<u32>().ok()?))
        })
        .unzip();

    for num in right {
        frequency_map
            .entry(num)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let result = left
        .iter()
        .map(|location| *location * frequency_map.get(location).unwrap_or(&0))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
