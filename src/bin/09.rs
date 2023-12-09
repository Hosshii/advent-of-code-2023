use itertools::Itertools;
use nom::{
    character::complete::{i64, newline, space1},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    fn create_next_line(v: &[i64]) -> Vec<i64> {
        v.iter()
            .tuple_windows()
            .inspect(|(l, r)| {
                dbg!(l, r);
            })
            .map(|(l, r)| r - l)
            .collect_vec()
    }

    fn all_zero(v: &[i64]) -> bool {
        v.iter().all(|v| *v == 0)
    }

    fn update_upper_line(upper: &mut Vec<i64>, cur: &[i64]) {
        assert_eq!(upper.len(), cur.len());

        let upper_last = upper.last().unwrap();
        let cur_last = cur.last().unwrap();
        let next = *upper_last + *cur_last;
        upper.push(next);
    }

    fn solve(mut v: Vec<i64>) -> Vec<i64> {
        if all_zero(&v) {
            v.push(0);
            return v;
        }

        let next_line = create_next_line(&v);
        let next_line = solve(next_line);
        update_upper_line(&mut v, &next_line);
        v
    }

    let v = parse_input(input).unwrap().1;
    Some(v.into_iter().map(solve).map(|v| *v.last().unwrap()).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    fn create_next_line(v: &[i64]) -> Vec<i64> {
        v.iter()
            .tuple_windows()
            .inspect(|(l, r)| {
                dbg!(l, r);
            })
            .map(|(l, r)| r - l)
            .collect_vec()
    }

    fn all_zero(v: &[i64]) -> bool {
        v.iter().all(|v| *v == 0)
    }

    fn update_upper_line(upper: &mut Vec<i64>, cur: &[i64]) {
        assert_eq!(upper.len(), cur.len());

        let upper_first = upper.first().unwrap();
        let cur_first = cur.first().unwrap();
        let next = *upper_first - *cur_first;
        upper.insert(0, next);
    }

    fn solve(mut v: Vec<i64>) -> Vec<i64> {
        if all_zero(&v) {
            v.insert(0, 0);
            return v;
        }

        let next_line = create_next_line(&v);
        let next_line = solve(next_line);
        update_upper_line(&mut v, &next_line);
        v
    }

    let v = parse_input(input).unwrap().1;
    Some(v.into_iter().map(solve).map(|v| *v.first().unwrap()).sum())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, i64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
