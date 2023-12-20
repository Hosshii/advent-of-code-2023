use std::{
    fmt::Display,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use nom::{
    character::complete::{newline, u64},
    combinator::map,
    multi::{many1, separated_list1},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Matrix<T>(Vec<Vec<T>>);

impl<T> Matrix<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        Self(v)
    }
}

impl<T> Parse for Matrix<T>
where
    T: Parse,
{
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        map(separated_list1(newline, many1(T::parse)), Matrix::new)(input)
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self {
            for v in row {
                write!(f, "{}", v)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = Vec<T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a Vec<T>;

    type IntoIter = std::slice::Iter<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut Vec<T>;

    type IntoIter = std::slice::IterMut<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

pub trait Parse {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parse for u64 {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        u64(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    pub fn rev(self) -> Self {
        use Direction::*;
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn neighbor(self, dir: Direction) -> Self {
        let Pos { x, y } = self;
        let (new_x, new_y) = match dir {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        Self { x: new_x, y: new_y }
    }

    pub fn saturating_neighbor(self, dir: Direction, max: Self) -> Self {
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

    pub fn is_top(self, top: usize) -> bool {
        self.y == top
    }
    pub fn is_right(self, right: usize) -> bool {
        self.x == right
    }
    pub fn is_bottom(self, bottom: usize) -> bool {
        self.y == bottom
    }
    pub fn is_left(self, left: usize) -> bool {
        self.x == left
    }
}
