use std::str::FromStr;

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .unwrap()
            .1
            .iter()
            .filter(|game| {
                game.res.iter().all(|colors| {
                    colors.iter().all(|(num, color)| match color {
                        Color::Red => *num <= 12,
                        Color::Green => *num <= 13,
                        Color::Blue => *num <= 14,
                    })
                })
            })
            .map(|game| game.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .unwrap()
            .1
            .iter()
            .map(|game| {
                let (red, green, blue) =
                    game.res
                        .iter()
                        .fold((1, 1, 1), |(red, green, blue), colors| {
                            colors.iter().fold(
                                (red, green, blue),
                                |(red, green, blue), (n, color)| match color {
                                    Color::Red => (red.max(*n), green, blue),
                                    Color::Green => (red, green.max(*n), blue),
                                    Color::Blue => (red, green, blue.max(*n)),
                                },
                            )
                        });
                red * green * blue
            })
            .sum(),
    )
}

enum Color {
    Red,
    Green,
    Blue,
}

struct Game {
    id: u32,
    res: Vec<Vec<(u32, Color)>>,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ))(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    fn parse_game_name(input: &str) -> IResult<&str, u32> {
        map(permutation((tag("Game"), space1, u32)), |(_, _, v)| v)(input)
    }

    fn parse_color_num(input: &str) -> IResult<&str, (u32, Color)> {
        map(permutation((u32, space1, parse_color)), |(n, _, c)| (n, c))(input)
    }

    map(
        permutation((
            terminated(parse_game_name, tag(": ")),
            separated_list1(
                terminated(tag(";"), space1),
                separated_list1(terminated(tag(","), space1), parse_color_num),
            ),
        )),
        |(id, res)| Game { id, res },
    )(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    many1(terminated(parse_game, line_ending))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    parse_games(input)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
