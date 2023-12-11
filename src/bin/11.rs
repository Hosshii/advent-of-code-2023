use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let u = parse_input(input);
    let u = extend(u);

    for row in &u {
        for b in row {
            if *b {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!()
    }

    let mut set = HashSet::new();
    for (y, row) in u.iter().enumerate() {
        for (x, b) in row.iter().enumerate() {
            if *b {
                set.insert(Pos::new(x, y));
            }
        }
    }

    Some(
        set.into_iter()
            .combinations(2)
            .map(|v| {
                let (l, r) = (v[0], v[1]);
                l.distance(r) as u64
            })
            .sum(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

type Universe = Vec<Vec<bool>>;

fn parse_input(input: &str) -> Universe {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec()
}

fn extend(v: Universe) -> Universe {
    fn extend_row(v: Universe) -> Universe {
        v.into_iter()
            .flat_map(|v| {
                if v.iter().all(|v| !v) {
                    vec![v.clone(), v]
                } else {
                    vec![v]
                }
            })
            .collect_vec()
    }

    fn extend_col(mut v: Universe) -> Universe {
        let mut dot_cols = Vec::new();
        for x in 0..v[0].len() {
            let mut is_dot = true;
            for row in &v {
                if row[x] {
                    is_dot = false;
                    break;
                }
            }

            if is_dot {
                dot_cols.push(x);
            }
        }

        let dot_cols = dot_cols.into_iter().enumerate().collect_vec();
        for (x, offset) in dot_cols {
            for row in &mut v {
                row.insert(x + offset, false);
            }
        }
        v
    }

    extend_row(extend_col(v))
}

const TIMES: usize = 1000000;

pub fn part_two(input: &str) -> Option<u64> {
    let u = parse_input(input);

    for row in &u {
        for b in row {
            if *b {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!()
    }

    let mut set = Vec::new();
    for (y, row) in u.iter().enumerate() {
        for (x, b) in row.iter().enumerate() {
            if *b {
                set.push((Pos::new(x, y), 0, 0));
            }
        }
    }

    for (y, row) in u.iter().enumerate() {
        if row.iter().all(|v| !v) {
            for v in set.iter_mut() {
                if y < v.0.y {
                    v.2 += TIMES - 1;
                }
            }
        }
    }

    for x in 0..u[0].len() {
        let mut is_dot = true;
        for row in &u {
            if row[x] {
                is_dot = false;
                break;
            }
        }

        if is_dot {
            for v in set.iter_mut() {
                if x < v.0.x {
                    v.1 += TIMES - 1;
                }
            }
        }
    }

    let set = set
        .into_iter()
        .map(|(p, x, y)| Pos {
            x: p.x + x,
            y: p.y + y,
        })
        .collect_vec();

    Some(
        set.into_iter()
            .combinations(2)
            .map(|v| {
                let (l, r) = (v[0], v[1]);
                l.distance(r) as u64
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
