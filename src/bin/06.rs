use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u64},
    combinator::eof,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = parse_input(input).unwrap().1;
    let pair: Vec<_> = times.into_iter().zip(distances).collect();

    Some(
        pair.into_iter()
            .map(|(time, distance)| {
                (0..=time)
                    .filter(|charge| {
                        let actual = calc_distance(*charge, time);
                        distance < actual
                    })
                    .count() as u64
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (times, distances) = parse_input(input).unwrap().1;

    let s = times.into_iter().map(|v| v.to_string()).collect::<String>();
    let time = s.parse::<u64>().unwrap();

    let s = distances
        .into_iter()
        .map(|v| v.to_string())
        .collect::<String>();
    let distance = s.parse::<u64>().unwrap();

    Some(
        (0..=time)
            .filter(|charge| {
                let actual = calc_distance(*charge, time);
                distance < actual
            })
            .count() as u64,
    )
}

fn calc_distance(charge_msec: u64, total_msec: u64) -> u64 {
    assert!(charge_msec <= total_msec);

    let speed = charge_msec;
    let remain_time = total_msec - charge_msec;

    speed * remain_time
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    terminated(
        permutation((
            terminated(
                preceded(
                    permutation((tag("Time:"), space0)),
                    separated_list1(space1, u64),
                ),
                newline,
            ),
            terminated(
                preceded(
                    permutation((tag("Distance:"), space0)),
                    separated_list1(space1, u64),
                ),
                newline,
            ),
        )),
        eof,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
