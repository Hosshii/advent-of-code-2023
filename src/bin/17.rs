use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use advent_of_code::common::*;
use nom::{character::complete::anychar, combinator::map_res};
advent_of_code::solution!(17);

const DIRS: [Direction; 4] = [
    Direction::Top,
    Direction::Right,
    Direction::Bottom,
    Direction::Left,
];

type DP = Vec<Vec<[[u64; MAX_LEN]; 4]>>;
const MAX_LEN: usize = 3;

pub fn part_one(input: &str) -> Option<u64> {
    let m = Matrix::<N>::parse(input).unwrap().1;

    let mut dp = vec![vec![[[u64::MAX / 2; MAX_LEN]; 4]; m[0].len()]; m.len()];

    for v in &mut dp[0][0] {
        for v in v {
            *v = 0;
        }
    }

    for dir in 0..4 {
        for len in 0..MAX_LEN {
            let from = match dir {
                0 => Direction::Top,
                1 => Direction::Right,
                2 => Direction::Bottom,
                3 => Direction::Left,
                _ => todo!(),
            };
            let start = Idx::new(Pos::new(0, 0), from, len + 1);
            dijkstra(&mut dp, &m, start);
        }
    }

    let mut min = u64::MAX;
    let max = Pos::new(m[0].len() - 1, m.len() - 1);
    for dir in 0..4 {
        for len in 0..MAX_LEN {
            min = min.min(dp[max.y][max.x][dir][len]);
        }
    }
    Some(min)
}

fn dijkstra(dp: &mut DP, m: &Matrix<N>, start: Idx) {
    let mut cur = start;
    let mut seen = HashSet::new();
    let max = Pos::new(m[0].len() - 1, m.len() - 1);
    // let mut count = 0;
    let mut heap = BinaryHeap::new();

    for (y, row) in dp.iter().enumerate() {
        for (x, dirs) in row.iter().enumerate() {
            for (dir, lens) in dirs.iter().enumerate() {
                for (len, v) in lens.iter().enumerate() {
                    if *v != u64::MAX / 2 {
                        let from = match dir {
                            0 => Direction::Top,
                            1 => Direction::Right,
                            2 => Direction::Bottom,
                            3 => Direction::Left,
                            _ => todo!(),
                        };
                        let idx = Idx::new(Pos::new(x, y), from, len + 1);
                        heap.push((Reverse(*v), idx));
                    }
                }
            }
        }
    }
    // heap.push((Reverse(idx(dp, cur)), cur));

    loop {
        // if seen.len() == num {
        //     break;
        // }
        if seen.insert(cur) {
            // dbg!("ee");
            continue;
        }

        for edge in DIRS {
            let candidate = cur.saturating_neighbor(edge, max);
            if candidate == cur || MAX_LEN < candidate.len {
                continue;
            }

            let new_cost = m[candidate.pos.y][candidate.pos.x].0 + idx(dp, cur);
            let cur_cost = idx_mut(dp, candidate);

            if new_cost < *cur_cost {
                *cur_cost = new_cost;
                heap.push((Reverse(new_cost), candidate));
            }
        }

        // let mut min = None;
        // for (y, row) in dp.iter().enumerate() {
        //     for (x, dirs) in row.iter().enumerate() {
        //         for (dir, lens) in dirs.iter().enumerate() {
        //             for (len, v) in lens.iter().enumerate() {
        //                 count += 1;
        //                 let from = match dir {
        //                     0 => Direction::Top,
        //                     1 => Direction::Right,
        //                     2 => Direction::Bottom,
        //                     3 => Direction::Left,
        //                     _ => todo!(),
        //                 };
        //                 let idx = Idx::new(Pos::new(x, y), from, len + 1);
        //                 if seen.contains(&idx) {
        //                     continue;
        //                 }

        //                 match min {
        //                     None => min = Some((*v, idx)),
        //                     Some((w, _)) => {
        //                         if *v <= w {
        //                             min = Some((*v, idx));
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        // if let Some(v) = min {
        //     cur = v.1;
        // } else {
        //     let num = m[0].len() * m.len() * 4 * MAX_LEN;
        //     dbg!(num, count);
        //     assert_eq!(seen.len(), num);
        //     break;
        // }

        if let Some(v) = heap.pop() {
            cur = v.1;
        } else {
            // let num = m[0].len() * m.len() * 4 * MAX_LEN;
            // dbg!(num, count);
            // assert_eq!(seen.len(), num);
            break;
        }
    }
}

fn idx(dp: &DP, idx: Idx) -> u64 {
    dp[idx.pos.y][idx.pos.x][idx.from as usize][idx.len - 1]
}

fn idx_mut(dp: &mut DP, idx: Idx) -> &mut u64 {
    &mut dp[idx.pos.y][idx.pos.x][idx.from as usize][idx.len - 1]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Idx {
    pos: Pos,
    from: Direction,
    len: usize, // 1 ~ MAX_LEN
}

impl Idx {
    fn new(pos: Pos, from: Direction, len: usize) -> Self {
        Self { pos, from, len }
    }

    fn saturating_neighbor(self, to: Direction, max: Pos) -> Self {
        let pos = self.pos.saturating_neighbor(to, max);

        if pos == self.pos {
            self
        } else if self.from.rev() == to {
            Self::new(pos, to.rev(), self.len + 1)
        } else {
            Self::new(pos, to.rev(), 1)
        }
    }
}

struct N(u64);
impl Parse for N {
    fn parse(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        map_res(anychar, |c| c.to_digit(10).map(|v| N(v as u64)).ok_or(()))(input)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
