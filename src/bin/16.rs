use std::convert::identity;

use nom::{
    character::complete::{anychar, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    fn solve(seen: &mut Matrix<Vec<Direction>>, m: &Matrix<Ground>, cur: Pos, from: Direction) {
        if seen[cur.y][cur.x].contains(&from) {
            return;
        }
        seen[cur.y][cur.x].push(from);

        let candidate = next_pos(m, cur, from);
        // dbg!(&candidate, cur, from);
        for (next, from) in candidate {
            solve(seen, m, next, from);
        }
    }

    let m = parse_input(input).unwrap().1;
    let mut seen = vec![vec![vec![]; m[0].len()]; m.len()];
    solve(&mut seen, &m, Pos::new(0, 0), Direction::Left);

    dbg_matrix(&seen);

    Some(
        seen.iter()
            .map(|row| row.iter().filter(|b| !b.is_empty()).count())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    fn solve(seen: &mut Matrix<Vec<Direction>>, m: &Matrix<Ground>, cur: Pos, from: Direction) {
        if seen[cur.y][cur.x].contains(&from) {
            return;
        }
        seen[cur.y][cur.x].push(from);

        let candidate = next_pos(m, cur, from);
        // dbg!(&candidate, cur, from);
        for (next, from) in candidate {
            solve(seen, m, next, from);
        }
    }

    let m = parse_input(input).unwrap().1;

    let x_len = m[0].len();
    let y_len = m.len();
    let x_iter = (0..x_len).flat_map(|x| {
        [
            (Pos::new(x, 0), Direction::Top),
            (Pos::new(x, y_len - 1), Direction::Bottom),
        ]
    });
    let y_iter = (0..y_len).flat_map(|y| {
        [
            (Pos::new(0, y), Direction::Left),
            (Pos::new(x_len - 1, y), Direction::Right),
        ]
    });

    x_iter
        .chain(y_iter)
        .map(|(pos, dir)| {
            let mut seen = vec![vec![vec![]; m[0].len()]; m.len()];
            solve(&mut seen, &m, pos, dir);
            seen.iter()
                .map(|row| row.iter().filter(|b| !b.is_empty()).count())
                .sum()
        })
        .max()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Ground {
    Space,
    LMirror,
    RMirror,
    VSplitter,
    HSplitter,
}

type Matrix<T> = Vec<Vec<T>>;
fn parse_input(input: &str) -> IResult<&str, Matrix<Ground>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Ground>> {
    many1(map_res(anychar, |c| match c {
        '.' => Ok(Ground::Space),
        '/' => Ok(Ground::LMirror),
        '\\' => Ok(Ground::RMirror),
        '|' => Ok(Ground::VSplitter),
        '-' => Ok(Ground::HSplitter),
        _ => Err(()),
    }))(input)
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

    fn neighbor(self, dir: Direction) -> Self {
        let Pos { x, y } = self;
        let (new_x, new_y) = match dir {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        Self { x: new_x, y: new_y }
    }

    fn saturating_neighbor(self, dir: Direction, max: Self) -> Self {
        let b = match dir {
            Direction::Top => self.is_top(0),
            Direction::Right => self.is_right(max.x),
            Direction::Bottom => self.is_bottom(max.y),
            Direction::Left => self.is_left(0),
        };
        if b {
            self
        } else {
            self.neighbor(dir)
        }
    }

    fn is_top(self, top: usize) -> bool {
        self.y == top
    }
    fn is_right(self, right: usize) -> bool {
        self.x == right
    }
    fn is_bottom(self, bottom: usize) -> bool {
        self.y == bottom
    }
    fn is_left(self, left: usize) -> bool {
        self.x == left
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn rev(self) -> Self {
        use Direction::*;
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }
}

fn next_pos(m: &Matrix<Ground>, cur: Pos, from: Direction) -> Vec<(Pos, Direction)> {
    use Direction::*;
    use Ground::*;
    let mut next_dirs = Vec::new();
    match (m[cur.y][cur.x], from) {
        (Space, dir) => {
            let rev = dir.rev();
            next_dirs.push(rev);
        }
        (LMirror, Top) | (RMirror, Bottom) => next_dirs.push(Left),
        (LMirror, Right) | (RMirror, Left) => next_dirs.push(Bottom),
        (LMirror, Bottom) | (RMirror, Top) => next_dirs.push(Right),
        (LMirror, Left) | (RMirror, Right) => next_dirs.push(Top),
        (VSplitter, Top | Bottom) => next_dirs.push(from.rev()),
        (VSplitter, Left | Right) => {
            next_dirs.push(Top);
            next_dirs.push(Bottom);
        }
        (HSplitter, Top | Bottom) => {
            next_dirs.push(Left);
            next_dirs.push(Right);
        }
        (HSplitter, Left | Right) => next_dirs.push(from.rev()),
    }
    let x_max = m[0].len() - 1;
    let y_max = m.len() - 1;
    let max = Pos { x: x_max, y: y_max };
    next_dirs
        .into_iter()
        .filter_map(|dir| {
            let next = cur.saturating_neighbor(dir, max);
            if next != cur {
                Some((next, dir.rev()))
            } else {
                None
            }
        })
        .collect()
}

fn dbg_matrix(m: &Matrix<Vec<Direction>>) {
    for row in m {
        for v in row {
            print!("{}", if !v.is_empty() { '#' } else { '.' });
        }

        println!();
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
