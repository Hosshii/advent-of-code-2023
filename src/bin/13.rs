use itertools::Itertools;
use nom::{
    character::complete::{anychar, char, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .unwrap()
            .1
            .into_iter()
            .map(|matrix| {
                let transposed = transpose(&matrix);

                find_mirror_line(&matrix).unwrap_or_default() * 100
                    + find_mirror_line(&transposed).unwrap_or_default()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    fn solve(mut matrix: Matrix<Kind>, base: Option<usize>) -> Option<usize> {
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                matrix[y][x] = matrix[y][x].opposite();
                let v = find_mirror_line2(&matrix);
                if let Some(new) = v.iter().find(|v| Some(**v) != base) {
                    return Some(*new);
                }
                matrix[y][x] = matrix[y][x].opposite();
            }
        }

        None
    }

    fn find_mirror_line2(matrix: &Matrix<Kind>) -> Vec<usize> {
        // dbg!(matrix);
        let mut result = Vec::new();
        for i in 0..matrix.len() {
            let (l, r) = matrix.split_at(i);

            if is_reflect(l, r) {
                result.push(i);
            }
        }

        result
    }

    let matrixes = parse_input(input).unwrap().1;
    let bases = matrixes
        .iter()
        .map(|matrix| {
            let transposed = transpose(matrix);

            (find_mirror_line(matrix), find_mirror_line(&transposed))
        })
        .collect_vec();

    Some(
        matrixes
            .into_iter()
            .zip(bases)
            .map(|(matrix, line)| {
                let transposed = transpose(&matrix);

                solve(matrix, line.0).unwrap_or(0) * 100 + solve(transposed, line.1).unwrap_or(0)
            })
            .sum(),
    )
}

fn is_reflect(l: &[Vec<Kind>], r: &[Vec<Kind>]) -> bool {
    if l.is_empty() || r.is_empty() {
        return false;
    }
    l.iter().rev().zip(r).all(|(l, r)| l == r)
}

fn find_mirror_line(matrix: &Matrix<Kind>) -> Option<usize> {
    // dbg!(matrix);
    for i in 0..matrix.len() {
        let (l, r) = matrix.split_at(i);

        if is_reflect(l, r) {
            return Some(i);
        }
    }

    None
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

type Matrix<T> = Vec<Vec<T>>;
fn parse_input(input: &str) -> IResult<&str, Vec<Matrix<Kind>>> {
    parse_matrixes(input)
}

fn parse_matrixes(input: &str) -> IResult<&str, Vec<Matrix<Kind>>> {
    separated_list1(tuple((newline, newline)), parse_matrix)(input)
}

fn parse_matrix(input: &str) -> IResult<&str, Matrix<Kind>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Kind>> {
    many1(map_res(anychar, |c| match c {
        '.' => Ok(Kind::Ash),
        '#' => Ok(Kind::Rock),
        _ => Err(()),
    }))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Ash,
    Rock,
}

impl Kind {
    fn opposite(self) -> Self {
        match self {
            Kind::Ash => Kind::Rock,
            Kind::Rock => Kind::Ash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
