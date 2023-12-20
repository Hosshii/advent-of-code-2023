use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use nom::{
    character::complete::{anychar, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_input(input).unwrap().1;

    let mut transposed = transpose(&matrix);
    move_matrix(&mut transposed);
    Some(
        transposed
            .iter()
            .map(|row| {
                row.iter()
                    .rev()
                    .enumerate()
                    .filter_map(|(idx, k)| {
                        if *k == Kind::Round {
                            Some(idx + 1)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    fn cycle(matrix: Matrix<Kind>) -> Matrix<Kind> {
        let mut north = transpose(&matrix);
        move_matrix(&mut north);

        // dbg_matrix(&transpose(&north));

        let mut west = transpose(&north);
        move_matrix(&mut west);
        // dbg_matrix(&west);

        let east = reverse(west);
        let mut south = transpose(&east);
        move_matrix(&mut south);
        // dbg_matrix(&reverse(transpose(&south)));

        let mut east = transpose(&south);
        move_matrix(&mut east);
        reverse(east)
    }

    fn count(matrix: &Matrix<Kind>) -> usize {
        matrix
            .iter()
            .map(|row| {
                row.iter()
                    .rev()
                    .enumerate()
                    .filter_map(|(idx, k)| {
                        if *k == Kind::Round {
                            Some(idx + 1)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    let mut matrix = parse_input(input).unwrap().1;

    for i in 0..10000 {
        matrix = cycle(matrix);
    }

    let mut result = Vec::new();
    for i in 0..10000 {
        matrix = cycle(matrix);
        let transposed = transpose(&matrix);
        let v = count(&transposed);
        result.push(v);
    }

    // dbg!(&result[0..20]);

    let p = find_loop(&result).unwrap();
    // dbg!(&p);
    let len = p.len();
    let idx = ((1000000000 - 2000) % len) + 2000 % len;
    Some(result[idx])

    // let mut map = HashMap::new();
    // for i in result {
    //     *map.entry(i).or_insert(0) += 1;
    // }
    // dbg!(map.iter().filter(|(_, v)| **v > 10).count());

    // let transposed = transpose(&matrix);
    // let v = count(&transposed);
    // Some(v)
}

type Matrix<T> = Vec<Vec<T>>;

fn dbg_matrix(m: &Matrix<Kind>) {
    for row in m {
        for v in row {
            print!("{}", v);
        }

        println!();
    }
    println!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Round,
    Cube,
    Space,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Round => write!(f, "O"),
            Kind::Cube => write!(f, "#"),
            Kind::Space => write!(f, "."),
        }
    }
}

fn move_matrix(matrix: &mut Matrix<Kind>) {
    for row in matrix.iter_mut() {
        move_row(row);
    }
}

fn move_row(row: &mut Vec<Kind>) {
    for g in row.split_mut(|k| k == &Kind::Cube) {
        let num_round = g.iter().filter(|v| **v == Kind::Round).count();
        let num_space = g.iter().filter(|v| **v == Kind::Space).count();
        for elem in g.iter_mut().take(num_round) {
            *elem = Kind::Round;
        }

        for elem in g.iter_mut().skip(num_round).take(num_space) {
            *elem = Kind::Space;
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Matrix<Kind>> {
    parse_matrix(input)
}

fn parse_matrix(input: &str) -> IResult<&str, Matrix<Kind>> {
    separated_list1(newline, parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Kind>> {
    many1(map_res(anychar, |c| match c {
        'O' => Ok(Kind::Round),
        '#' => Ok(Kind::Cube),
        '.' => Ok(Kind::Space),
        _ => Err(()),
    }))(input)
}

fn transpose<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    if v.is_empty() {
        return result;
    }

    for x in 0..v[0].len() {
        let mut new_row = Vec::new();
        for row in v.iter() {
            new_row.push(row[x].clone());
        }
        result.push(new_row);
    }
    result
}

fn reverse<T>(v: Matrix<T>) -> Matrix<T> {
    v.into_iter()
        .map(|row| row.into_iter().rev().collect_vec())
        .rev()
        .collect_vec()
}

fn find_loop(v: &[usize]) -> Option<Vec<usize>> {
    for i in 1..v.len() / 2 {
        let chunks = v.iter().chunks(i);
        let v = chunks.into_iter().collect_vec();
        let rev = v
            .into_iter()
            .rev()
            .skip(1)
            .map(|c| c.copied().collect_vec())
            .collect_vec();

        if 1 < rev.len() && rev.iter().all_equal() {
            return Some(rev[0].clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
