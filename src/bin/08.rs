use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    ops::ControlFlow,
};

use itertools::{multizip, Itertools};
use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, newline, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

advent_of_code::solution!(8);

const START_NODE: Node = Node(('A', 'A', 'A'));
const END_NODE: Node = Node(('Z', 'Z', 'Z'));

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, maps) = parse_input(input).unwrap().1;
    let map: Map = maps.into_iter().collect();
    let result =
        instructions
            .into_iter()
            .cycle()
            .try_fold((0, START_NODE), |(count, node), dir| {
                if node == END_NODE {
                    ControlFlow::Break(Ok(count))
                } else {
                    let Some(v) = map.get(&node) else {
                        return ControlFlow::Break(Err(()));
                    };

                    let next = match dir {
                        Direction::Left => v.0,
                        Direction::Right => v.1,
                    };

                    ControlFlow::Continue((count + 1, next))
                }
            });

    match result {
        ControlFlow::Break(Ok(v)) => Some(v),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    fn find_end_point(instructions: &[Direction], map: &Map, start_node: Node) -> u64 {
        // let mut seen = HashMap::new();

        let mut cur_node = start_node;
        // let mut result = Vec::new();

        for (count, i) in instructions.iter().cycle().enumerate() {
            if cur_node.0 .2 == 'Z' {
                return count as u64;
            }
            // if seen.get(&cur_node).is_some() {
            //     return count as u64;
            // }

            // seen.insert(cur_node, count);
            // result.push(count);

            let next = map.get(&cur_node).unwrap();

            let next = match i {
                Direction::Left => next.0,
                Direction::Right => next.1,
            };
            cur_node = next;
        }

        unreachable!()
    }

    let (instructions, maps) = parse_input(input).unwrap().1;
    let map: Map = maps.into_iter().collect();
    let start_nodes: Vec<_> = map.keys().copied().filter(|v| v.0 .2 == 'A').collect();

    let mut iters = Vec::new();
    for start_node in start_nodes {
        let iter = find_end_point(&instructions, &map, start_node);
        iters.push(iter);
    }

    let mut v = iters[0];
    dbg!(&iters);
    for i in iters.into_iter().skip(1) {
        v = lcm(v, i);
    }

    Some(v)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a < b {
        gcd(b, a)
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    let d = gcd(a, b);
    a / d * b
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node((char, char, char));

type Map = HashMap<Node, (Node, Node)>;
type Instructions = Vec<Direction>;

fn parse_input(input: &str) -> IResult<&str, (Instructions, Vec<(Node, (Node, Node))>)> {
    tuple((
        terminated(parse_instructions, tuple((newline, newline))),
        separated_list1(newline, parse_map),
    ))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(map_res(anychar, |c| match c {
        'L' => Ok(Direction::Left),
        'R' => Ok(Direction::Right),
        _ => Err(()),
    }))(input)
}

fn parse_map(input: &str) -> IResult<&str, (Node, (Node, Node))> {
    map(
        tuple((
            parse_node,
            space1,
            tag("="),
            space1,
            tuple((
                tag("("),
                tuple((parse_node, tag(", "), parse_node)),
                tag(")"),
            )),
        )),
        |(key, _, _, _, (_, (l, _, r), _))| (key, (l, r)),
    )(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    map(tuple((anychar, anychar, anychar)), Node)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
