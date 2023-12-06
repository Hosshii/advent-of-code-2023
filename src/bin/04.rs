use std::collections::HashSet;

use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{line_ending, space0, space1, u32},
    combinator::{all_consuming, eof, recognize},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_lines(input)
            .unwrap()
            .1
            .into_iter()
            .map(|(_, winnings, numbers)| {
                let num = count_win(winnings, numbers);
                if num == 0 {
                    0
                } else {
                    2u32.pow(num as u32 - 1)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.lines().count();
    let mut v = vec![1; rows];
    parse_lines(input)
        .unwrap()
        .1
        .into_iter()
        .for_each(|(cur, winnings, numbers)| {
            let cur = cur as usize;
            let num = count_win(winnings, numbers);
            for i in cur..(cur + num) {
                v[i] += v[cur - 1];
            }
        });

    Some(v.iter().sum())
}

fn count_win(winnings: Vec<u32>, numbers: Vec<u32>) -> usize {
    let winnings: HashSet<_> = winnings.into_iter().collect();
    let num = numbers.iter().filter(|n| winnings.contains(n)).count();
    num
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(u32, Vec<u32>, Vec<u32>)>> {
    all_consuming(terminated(
        separated_list1(line_ending, parse_line),
        permutation((line_ending, eof)),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    permutation((
        parse_header,
        parse_list_nums,
        preceded(permutation((space0, tag("|"), space0)), parse_list_nums),
    ))(input)
}

fn parse_header(input: &str) -> IResult<&str, u32> {
    preceded(
        permutation((tag("Card"), space1)),
        terminated(u32, tag(":")),
    )(input)
}

fn parse_list_nums(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(space0, separated_list1(space1, u32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
