use std::{collections::HashMap, ops};

use itertools::Itertools;
use nom::{
    character::complete::{anychar, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    use Direction::*;
    let tiles = parse_input(input).unwrap().1;
    let y_num = tiles.len();
    let x_num = tiles[0].len();

    let map = PipeMap::new(tiles);
    let start = map.find_start();

    // dbg!(start);

    let neighbor =
        [Top, Right, Bottom, Left].map(|dir| start.get_neighbor(dir, x_num - 1, y_num - 1));

    for neighbor in neighbor {
        let iter = map.iter(neighbor);
        let Some(last) = iter.last() else {
            continue;
        };

        if start.is_neighbor(last) {
            let iter = map.iter(neighbor);
            let v = iter.collect::<Vec<_>>();
            let index = v.len() / 2;
            return Some(index as u32 + 1);
        }
    }

    unimplemented!()
}

pub fn part_two(input: &str) -> Option<u32> {
    use Direction::*;
    let tiles = parse_input(input).unwrap().1;
    let y_num = tiles.len();
    let x_num = tiles[0].len();

    let mut map = PipeMap::new(tiles);
    let start = map.find_start();

    // dbg!(start);

    let neighbor = [Top, Right, Bottom, Left].iter().filter_map(|dir| {
        let candidate = start.get_neighbor(*dir, x_num - 1, y_num - 1);
        let rev = dir.rev();
        let next_dir = map.data[candidate].get_direction();
        if next_dir.map(|(l, r)| l == rev || r == rev).unwrap_or(false) {
            Some(candidate)
        } else {
            None
        }
    });

    let mut v = None;
    for neighbor in neighbor {
        let iter = map.iter(neighbor);
        let Some(last) = iter.last() else {
            continue;
        };

        if start.is_neighbor(last) {
            map.relation.connect(start, last);
            map.relation.connect(start, neighbor);
            dbg!(start, last, neighbor);
            let mut vv = map.iter(start).take_while(|v| *v != start).collect_vec();
            vv.push(start);
            v = Some(vv);
            break;
        }
    }
    let Some(v) = v else { todo!() };

    let v: HashMap<_, _> = v.into_iter().map(|cell| (cell, map.data[cell])).collect();

    let mut count = 0;
    for (y, row) in map.data.data.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            // if *tile == Tile::Ground {
            let cell = Cell(x, y);
            if is_in(&v, cell) {
                dbg!(cell);
                count += 1;
            }
            // }
        }
    }

    Some(count)
}

fn is_in(lp: &HashMap<Cell, Tile>, cell: Cell) -> bool {
    if lp.contains_key(&cell) {
        return false;
    }

    if cell == Cell(13, 4) {
        dbg!("n");
    }

    let mut cur = cell;
    let mut count = 0;
    loop {
        if let Some(tile) = lp.get(&cur) {
            // dbg!(tile);
            if *tile != Tile::BottomLeft && *tile != Tile::TopRight {
                count += 1;
            }
        }
        let left = cur.get_neighbor(Direction::Left, usize::MAX, usize::MAX);
        let up = left.get_neighbor(Direction::Top, usize::MAX, usize::MAX);

        if cur.0 == 0 || cur.1 == 0 {
            break;
        }
        cur = up;
    }

    count % 2 == 1
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(Tile::parse)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Vertical,
    Horizontal,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
    Ground,
    Start,
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Tile> {
        use Tile::*;
        map_res(anychar, |c| match c {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(TopRight),
            'J' => Ok(TopLeft),
            '7' => Ok(BottomLeft),
            'F' => Ok(BottomRight),
            '.' => Ok(Ground),
            'S' => Ok(Start),
            _ => Err(()),
        })(input)
    }

    fn get_direction(self) -> Option<(Direction, Direction)> {
        use Direction::*;
        match self {
            Tile::Vertical => Some((Top, Bottom)),
            Tile::Horizontal => Some((Left, Right)),
            Tile::TopRight => Some((Top, Right)),
            Tile::TopLeft => Some((Top, Left)),
            Tile::BottomLeft => Some((Bottom, Left)),
            Tile::BottomRight => Some((Bottom, Right)),
            Tile::Ground | Tile::Start => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cell(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn rev(self) -> Direction {
        use Direction::*;
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }
}

impl Cell {
    fn direction(self, other: Self) -> Direction {
        if self.0 == other.0 && self.1 < other.1 {
            Direction::Top
        } else if self.0 > other.0 && self.1 == other.1 {
            Direction::Right
        } else if self.0 == other.0 && self.1 > other.1 {
            Direction::Bottom
        } else if self.0 < other.0 && self.1 == other.1 {
            Direction::Left
        } else {
            todo!()
        }
    }

    fn get_neighbor(self, dir: Direction, max_x: usize, max_y: usize) -> Self {
        match dir {
            Direction::Top => Self(self.0, self.1.saturating_sub(1)),
            Direction::Right => Self((self.0 + 1).min(max_x), self.1),
            Direction::Bottom => Self(self.0, (self.1 + 1).min(max_y)),
            Direction::Left => Self(self.0.saturating_sub(1), self.1),
        }
    }

    fn is_neighbor(self, other: Self) -> bool {
        if self.0 == other.0 {
            if self.1 < other.1 {
                other.1 - self.1 == 1
            } else {
                self.1 - other.1 == 1
            }
        } else if self.1 == other.1 {
            if self.0 < other.0 {
                other.0 - self.0 == 1
            } else {
                self.0 - other.0 == 1
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Link {
    up: Option<Cell>,
    down: Option<Cell>,
    right: Option<Cell>,
    left: Option<Cell>,
}

struct Matrix {
    data: Vec<Vec<Tile>>,
}
impl Matrix {
    fn find_start(&self) -> Cell {
        for (y, row) in self.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == Tile::Start {
                    return Cell(x, y);
                }
            }
        }

        unreachable!()
    }
}

impl ops::Index<Cell> for Matrix {
    type Output = Tile;

    fn index(&self, index: Cell) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

impl ops::IndexMut<Cell> for Matrix {
    fn index_mut(&mut self, index: Cell) -> &mut Self::Output {
        &mut self.data[index.1][index.0]
    }
}

#[derive(Debug)]
struct Relation {
    data: Vec<Vec<Link>>,
}

impl Relation {
    fn connect(&mut self, cell1: Cell, cell2: Cell) {
        match cell1.direction(cell2) {
            Direction::Top => {
                self[cell1].down = Some(cell2);
                self[cell2].up = Some(cell1);
            }
            Direction::Right => {
                self[cell1].left = Some(cell2);
                self[cell2].right = Some(cell1);
            }
            Direction::Bottom => {
                self[cell1].up = Some(cell2);
                self[cell2].down = Some(cell1);
            }
            Direction::Left => {
                self[cell1].right = Some(cell2);
                self[cell2].left = Some(cell1);
            }
        }
    }
}

impl ops::Index<Cell> for Relation {
    type Output = Link;

    fn index(&self, index: Cell) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

impl ops::IndexMut<Cell> for Relation {
    fn index_mut(&mut self, index: Cell) -> &mut Self::Output {
        &mut self.data[index.1][index.0]
    }
}

struct PipeMap {
    data: Matrix,
    relation: Relation,
}

impl PipeMap {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let row_num = tiles.len();
        let col_num = tiles[0].len();
        let mut relation = Relation {
            data: vec![vec![Link::default(); col_num]; row_num],
        };
        let matrix = Matrix { data: tiles };

        let mut connect_one = |cur_cell: Cell, next_dir: Direction| {
            let next = cur_cell.get_neighbor(next_dir, col_num - 1, row_num - 1);

            if cur_cell == next {
                return;
            }

            let rev = next_dir.rev();

            let next_tile = matrix[next];
            let dir = next_tile.get_direction();
            if let Some((d1, d2)) = dir {
                if d1 == rev || d2 == rev {
                    relation.connect(cur_cell, next);
                }
            }
        };

        for (y, row) in matrix.data.iter().enumerate() {
            for (x, cur) in row.iter().enumerate() {
                let cell = Cell(x, y);

                let Some((next_dir1, next_dir2)) = cur.get_direction() else {
                    continue;
                };

                connect_one(cell, next_dir1);
                connect_one(cell, next_dir2);
            }
        }

        Self {
            data: matrix,
            relation,
        }
    }

    fn iter(&self, start: Cell) -> Iter {
        Iter {
            map: &self.relation,
            cur: Some(start),
            prev: start,
        }
    }

    fn find_start(&self) -> Cell {
        self.data.find_start()
    }
}

struct Iter<'a> {
    map: &'a Relation,
    cur: Option<Cell>,
    prev: Cell,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        // dbg!(self.cur);
        let Some(cur) = self.cur else {
            // dbg!("none");
            return None;
        };

        let link = self.map[cur];
        let mut next = [link.up, link.down, link.left, link.right]
            .into_iter()
            .filter_map(|v| {
                let Some(cell) = v else {
                    return None;
                };

                if cell != self.prev {
                    Some(cell)
                } else {
                    None
                }
            });
        let next = next.next();
        // dbg!(next);

        self.cur = next;
        self.prev = cur;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
