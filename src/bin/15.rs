use std::ops::IndexMut;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1, anychar, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .unwrap()
            .1
            .into_iter()
            .map(|v| hash(v) as u64)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    type Boxes<'a> = Vec<Vec<Lens<'a>>>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Lens<'a> {
        label: &'a str,
        focal_len: u32,
    }

    impl<'a> Lens<'a> {
        fn new(label: &'a str, focal_len: u32) -> Self {
            Self { label, focal_len }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Op<'a> {
        Remove { label: &'a str },
        Insert { label: &'a str, focal_length: u32 },
    }
    fn parse_op(input: &str) -> IResult<&str, Op> {
        let (input, label) = alpha1(input)?;
        let (input, sym) = alt((tag("="), tag("-")))(input)?;
        if sym == "=" {
            let (input, len) = u32(input)?;
            Ok((
                input,
                Op::Insert {
                    label,
                    focal_length: len,
                },
            ))
        } else {
            Ok((input, Op::Remove { label }))
        }
    }

    fn parse_input(input: &str) -> IResult<&str, Vec<Op>> {
        separated_list1(tag(","), parse_op)(input)
    }

    let mut boxes: Boxes = vec![Vec::new(); 256];
    let ops = parse_input(input).unwrap().1;

    for op in ops {
        match op {
            Op::Remove { label } => {
                let hash = hash(label);
                let b = boxes.index_mut(hash as usize);
                if let Some(pos) = b.iter().position(|v| v.label == label) {
                    b.remove(pos);
                }
            }
            Op::Insert {
                label,
                focal_length,
            } => {
                let hash = hash(label);
                let b = boxes.index_mut(hash as usize);

                match b.iter().position(|v| v.label == label) {
                    Some(pos) => b[pos] = Lens::new(label, focal_length),
                    None => b.push(Lens::new(label, focal_length)),
                }
            }
        }
    }

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(box_idx, b)| {
                b.iter()
                    .enumerate()
                    .map(|(slot_idx, lens)| {
                        (box_idx + 1) * (slot_idx + 1) * lens.focal_len as usize
                    })
                    .sum::<usize>()
            })
            .sum(),
    )
}

fn hash(input: &str) -> u8 {
    fn hash_of_ascii(current: u8, c: u8) -> u8 {
        let mut current = current as u64;
        current += c as u64;
        current *= 17;
        current as u8
    }
    // dbg!(input);

    input
        .chars()
        .fold(0, |acc, cur| hash_of_ascii(acc, cur as u8))
}

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        tag(","),
        take_while1(|c: char| c != ',' && !c.is_whitespace()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
