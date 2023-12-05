advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    fn pase_input(input: &str) -> impl Iterator<Item = u32> + '_ {
        input.lines().map(|v| {
            let first = v.chars().find(|c| c.is_numeric()).unwrap();
            let first = first.to_digit(10).unwrap();
            let last = v.chars().rev().find(|c| c.is_numeric()).unwrap();
            let last = last.to_digit(10).unwrap();
            first * 10 + last
        })
    }

    Some(pase_input(input).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().flat_map(p_two::find_digit).sum())
}

mod p_two {

    const ARR: [[&str; 2]; 9] = [
        ["1", "one"],
        ["2", "two"],
        ["3", "three"],
        ["4", "four"],
        ["5", "five"],
        ["6", "six"],
        ["7", "seven"],
        ["8", "eight"],
        ["9", "nine"],
    ];

    pub fn find_digit(input: &str) -> Option<u32> {
        let mut first = None;
        let mut last = None;
        for i in 0..input.len() {
            for j in 0..ARR.len() {
                for k in 0..2 {
                    if input[i..].starts_with(ARR[j][k]) {
                        if first.is_none() {
                            first = Some(j + 1)
                        }
                        last = Some(j + 1)
                    }
                }
            }
        }

        first.zip(last).map(|(l, r)| l as u32 * 10 + r as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
