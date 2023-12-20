use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, space1, u64},
    combinator::map_res,
    multi::{many1, separated_list0, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    fn is_valid(springs: &[Kind], num: &[u64]) -> bool {
        let mut v = Vec::new();
        let mut count = 0;
        for s in springs {
            if s == &Kind::Damaged {
                count += 1
            } else if 0 < count {
                v.push(count);
                count = 0;
            }
        }

        if 0 < count {
            v.push(count);
        }

        v.len() == num.len() && v.iter().zip(num).all(|(l, r)| l == r)
    }

    fn solve_line(line: &mut Vec<Kind>, num: &[u64]) -> u64 {
        if line.iter().all(|v| v != &Kind::Unknown) {
            if is_valid(line, num) {
                1
            } else {
                0
            }
        } else {
            let first = line.iter().position(|v| v == &Kind::Unknown).unwrap();
            line[first] = Kind::Damaged;

            let l = solve_line(line, num);
            line[first] = Kind::Operational;
            let r = solve_line(line, num);
            line[first] = Kind::Unknown;

            l + r
        }
    }

    Some(
        Parse::parse(input)
            .unwrap()
            .1
            .into_iter()
            .map(|(mut line, num)| solve_line(&mut line, &num))
            .inspect(|v| {
                // dbg!(v);
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

trait Parse {
    type Output;
    type Error;

    fn parse(self) -> Result<Self::Output, Self::Error>;
}

impl<'a> Parse for &'a str {
    type Output = (&'a str, Vec<(Vec<Kind>, Vec<u64>)>);

    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn parse(self) -> Result<Self::Output, Self::Error> {
        separated_list1(
            newline,
            tuple((terminated(parse_spring, space1), parse_num)),
        )(self)
    }
}

fn parse_num(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(","), u64)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Operational,
    Damaged,
    Unknown,
}

fn parse_spring(input: &str) -> IResult<&str, Vec<Kind>> {
    many1(map_res(
        alt((char('#'), char('.'), char('?'))),
        |v| match v {
            '.' => Ok(Kind::Operational),
            '#' => Ok(Kind::Damaged),
            '?' => Ok(Kind::Unknown),
            _ => Err(()),
        },
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
