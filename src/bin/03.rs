use std::collections::{HashMap, HashSet};

use nom::{
    branch::permutation,
    bytes::complete::take_while1,
    character::{
        complete::{digit1, line_ending},
        is_alphanumeric,
    },
    combinator::{map, map_res, opt},
    multi::{many0, many1, separated_list1},
    IResult,
};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let sum = input
        .lines()
        .enumerate()
        .map(|(y, line)| process_line(line, &schematic, y))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(p2(input))
}

fn process_line(input: &str, schematic: &[Vec<char>], y: usize) -> u32 {
    let (_, (arr, _)) = parse_line(input).unwrap();
    let mut result = 0;

    let mut x = 0;
    for (offset, len, n) in arr {
        x += offset;

        for i in 0..len {
            let pos = Pos {
                x: (x + i) as i32,
                y: y as i32,
            };

            if is_adjacent_sym(schematic, pos) {
                result += n;
                break;
            }
        }

        x += len;
    }

    result
}

fn parse_line(input: &str) -> IResult<&str, (Vec<(usize, usize, u32)>, usize)> {
    map(
        permutation((many0(parse_sp_num), opt(parse_non_num1))),
        |(sp1, v)| (sp1, v.unwrap_or("").len()),
    )(input)
}

fn parse_sp_num(input: &str) -> IResult<&str, (usize, usize, u32)> {
    map_res(permutation((opt(parse_non_num1), digit1)), |(sp, n)| {
        let num_len = n.len();
        n.parse::<u32>()
            .map(|v| (sp.unwrap_or("").len(), num_len, v))
    })(input)
}

fn parse_non_num1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_numeric())(input)
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.' && !c.is_whitespace()
}

fn is_adjacent_sym(schematic: &[Vec<char>], pos: Pos) -> bool {
    use Direction::*;
    const DIRS: [Direction; 8] = [
        Top,
        TopRight,
        Right,
        BottomRight,
        Bottom,
        BottomLeft,
        Left,
        TopLeft,
    ];

    let x_min = 0;
    let x_max = schematic[0].len() as i32 - 1;

    let y_min = 0;
    let y_max = schematic.len() as i32 - 1;
    let border = Border {
        x_min,
        x_max,
        y_min,
        y_max,
    };

    DIRS.iter().any(|dir| {
        let adj_pos = pos.mov(dir, &border);
        let c = schematic[adj_pos.y as usize][adj_pos.x as usize];
        is_symbol(c)
    })
}

enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

struct Border {
    x_min: i32,
    x_max: i32,

    y_min: i32,
    y_max: i32,
}

impl Pos {
    pub fn mov(self, dir: &Direction, border: &Border) -> Self {
        let (x, y) = (self.x, self.y);

        let x = match dir {
            Direction::Top | Direction::Bottom => x,
            Direction::TopRight | Direction::Right | Direction::BottomRight => {
                (x + 1).min(border.x_max)
            }
            Direction::TopLeft | Direction::Left | Direction::BottomLeft => {
                (x - 1).max(border.x_min)
            }
        };

        let y = match dir {
            Direction::Left | Direction::Right => y,
            Direction::TopLeft | Direction::Top | Direction::TopRight => (y + 1).min(border.y_max),
            Direction::BottomLeft | Direction::Bottom | Direction::BottomRight => {
                (y - 1).max(border.y_min)
            }
        };

        Self { x, y }
    }
}

fn p2(input: &str) -> u32 {
    use Direction::*;
    const DIRS: [Direction; 8] = [
        Top,
        TopRight,
        Right,
        BottomRight,
        Bottom,
        BottomLeft,
        Left,
        TopLeft,
    ];

    let x_min = 0;
    let x_max = input.lines().next().unwrap().len() as i32 - 1;

    let y_min = 0;
    let y_max = input.lines().count() as i32 - 1;
    let border = Border {
        x_min,
        x_max,
        y_min,
        y_max,
    };

    let map = pase_input(input);
    let set = find_gear_pos(input);
    let mut result = 0;

    for pos in set {
        let mut tmp = HashSet::new();
        for dir in DIRS {
            let moved = pos.mov(&dir, &border);

            if let Some(v) = map.get(&moved) {
                tmp.insert(v);
            }
        }

        if tmp.len() == 2 {
            let mut iter = tmp.iter();
            let lhs = iter.next().unwrap().2;
            let rhs = iter.next().unwrap().2;
            result += lhs * rhs;
        }
    }

    result
}

fn pase_input(input: &str) -> HashMap<Pos, (Pos, Pos, u32)> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        let (_, (arr, _)) = parse_line(line).unwrap();

        for (offset, len, n) in arr {
            x += offset;
            let first = Pos {
                x: x as i32,
                y: y as i32,
            };
            let last = Pos {
                x: (x + len - 1) as i32,
                y: y as i32,
            };
            for i in 0..len {
                let pos = Pos {
                    x: (x + i) as i32,
                    y: y as i32,
                };

                map.insert(pos, (first, last, n));
            }
            x += len;
        }
    }

    map
}

fn find_gear_pos(input: &str) -> HashSet<Pos> {
    let mut map = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                map.insert(Pos {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
