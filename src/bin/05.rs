use std::collections::HashMap;

use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    combinator::eof,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input).unwrap().1;

    let seeds = input.0;
    let maps = Maps {
        se2so: into_hmap(input.1),
        so2fe: into_hmap(input.2),
        fe2wa: into_hmap(input.3),
        wa2li: into_hmap(input.4),
        li2te: into_hmap(input.5),
        te2hu: into_hmap(input.6),
        hu2lo: into_hmap(input.7),
    };

    seeds
        .into_iter()
        .map(|seed| {
            // dbg!(seed);
            maps.location(seed).0
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input).unwrap().1;

    let seeds = input.0;
    let maps = Maps {
        se2so: into_hmap(input.1),
        so2fe: into_hmap(input.2),
        fe2wa: into_hmap(input.3),
        wa2li: into_hmap(input.4),
        li2te: into_hmap(input.5),
        te2hu: into_hmap(input.6),
        hu2lo: into_hmap(input.7),
    };

    let mut min = u64::MAX;
    for s in seeds.chunks(2) {
        let start = s[0];
        let len = s[1];

        let mut cur = start;
        while cur <= start + len {
            let (v, offset) = maps.location(cur);
            min = min.min(v);
            dbg!(offset);
            if let Some(offset) = offset {
                cur += offset;
            } else {
                break;
            }
        }
    }

    // let mut iter = seeds.chunks(2).flat_map(|s| {
    //     let start = s[0];
    //     let len = s[1];

    //     start..(start + len)
    // });

    // let mut min = u64::MAX;
    // loop {
    //     if let Some(seed) = iter.next() {
    //         let (v, skip) = maps.location(seed);
    //         iter.skip(skip as usize - 1);
    //     } else {
    //         break;
    //     }
    // }
    Some(min)
    // .min()
    // seeds
    // .into_iter()
    // .map(|seed| {
    //     dbg!(seed);
    //     maps.location(seed)
    // })
    // .min()
}

fn into_hmap(map: Vec<Map>) -> HMap {
    // let mut m = HMap::new();
    // for (dst, src, size) in map {
    //     for i in 0..size {
    //         m.insert(src + i, dst + i);
    //     }
    // }

    // m
    map
}

struct Maps {
    se2so: HMap,
    so2fe: HMap,
    fe2wa: HMap,
    wa2li: HMap,
    li2te: HMap,
    te2hu: HMap,
    hu2lo: HMap,
}

impl Maps {
    fn location(&self, seed: u64) -> (u64, Option<u64>) {
        fn get_next(map: &HMap, v: u64) -> (u64, Option<u64>) {
            for (dst, src, size) in map {
                if (*src..(*src + *size)).contains(&v) {
                    return (v - src + dst, Some(*src + *size - v));
                }
            }

            let min = map
                .iter()
                .filter_map(|(_, src, _)| if v < *src { Some(*src) } else { None })
                .min();
            let max = map
                .iter()
                .filter_map(|(_, src, len)| if *src < v { Some((*src, len)) } else { None })
                .max();

            let offset = match (max, min) {
                (None, None) => None,
                (None, Some(next)) => Some(next - v),
                (Some((max, len)), None) => {
                    if v < max + *len {
                        Some(1)
                    } else {
                        None
                    }
                }
                (Some((max, len)), Some(next)) => {
                    if v < max + *len {
                        Some(1)
                    } else {
                        Some(next - v)
                    }
                }
            };

            assert!(1 <= offset.unwrap_or(u64::MAX));
            (v, offset)
            // if let Some(v) = map.get(&v) {
            //     *v
            // } else {
            //     v
            // }
        }

        let (so, range1) = get_next(&self.se2so, seed);
        let (fe, range2) = get_next(&self.so2fe, so);
        let (wa, range3) = get_next(&self.fe2wa, fe);
        let (li, range4) = get_next(&self.wa2li, wa);
        let (te, range5) = get_next(&self.li2te, li);
        let (hu, range6) = get_next(&self.te2hu, te);
        let (lo, range7) = get_next(&self.hu2lo, hu);

        let next = [range1, range2, range3, range4, range5, range6, range7]
            .into_iter()
            .flatten()
            .min();

        (lo, next)
    }
}

// type HMap = HashMap<u64, u64>;
type HMap = Vec<Map>;

type Input = (
    Vec<u64>,
    Vec<Map>,
    Vec<Map>,
    Vec<Map>,
    Vec<Map>,
    Vec<Map>,
    Vec<Map>,
    Vec<Map>,
);
fn parse_input(input: &str) -> IResult<&str, Input> {
    permutation((
        terminated(parse_seeds, permutation((newline, newline))),
        terminated(parse_maps("seed-to-soil"), newline),
        terminated(parse_maps("soil-to-fertilizer"), newline),
        terminated(parse_maps("fertilizer-to-water"), newline),
        terminated(parse_maps("water-to-light"), newline),
        terminated(parse_maps("light-to-temperature"), newline),
        terminated(parse_maps("temperature-to-humidity"), newline),
        terminated(parse_maps("humidity-to-location"), eof),
    ))(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        permutation((tag("seeds:"), space1)),
        separated_list1(space1, u64),
    )(input)
}

type Map = (u64, u64, u64);
fn parse_map(input: &str) -> IResult<&str, Map> {
    permutation((terminated(u64, space1), terminated(u64, space1), u64))(input)
}

fn parse_maps<'a, 'b>(title: &'b str) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Map>> + 'b
where
    'a: 'b,
{
    preceded(
        permutation((tag(title), space1, tag("map:"), newline)),
        terminated(separated_list0(newline, parse_map), newline),
    )
}

fn parse_seed_to_soil_map(input: &str) -> IResult<&str, Vec<Map>> {
    preceded(
        permutation((tag("seed-to-soil map:"), newline)),
        separated_list0(newline, parse_map),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
