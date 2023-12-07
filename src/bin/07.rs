advent_of_code::solution!(7);

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use nom::{
    branch::permutation,
    character::complete::{anychar, newline, space1, u64},
    combinator::map_res,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

pub fn part_one(input: &str) -> Option<u64> {
    let mut v: Vec<_> = parse_input(input)
        .unwrap()
        .1
        .into_iter()
        .map(|(hands, bit)| (Type::from(hands), hands, bit))
        .collect();

    v.sort();
    Some(
        v.into_iter()
            .enumerate()
            // .inspect(|v| {
            //     dbg!(v);
            // })
            .map(|(idx, (_, _, bit))| (idx as u64 + 1) * bit)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut v: Vec<_> = parse_input(input)
        .unwrap()
        .1
        .into_iter()
        .map(|(hands, bit)| {
            let hands = hands.map(|v| JokerCard { card: v });
            (Type::from(hands), hands, bit)
        })
        .collect();

    v.sort();
    Some(
        v.into_iter()
            .enumerate()
            // .inspect(|v| {
            //     dbg!(v);
            // })
            .map(|(idx, (_, _, bit))| (idx as u64 + 1) * bit)
            .sum(),
    )
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Hands, u64)>> {
    separated_list1(newline, permutation((terminated(parse_hands, space1), u64)))(input)
}

fn parse_hands(input: &str) -> IResult<&str, Hands> {
    map_res(
        permutation((anychar, anychar, anychar, anychar, anychar)),
        |(c1, c2, c3, c4, c5)| {
            Ok::<_, ()>([
                Card::try_from(c1)?,
                Card::try_from(c2)?,
                Card::try_from(c3)?,
                Card::try_from(c4)?,
                Card::try_from(c5)?,
            ])
        },
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        use Card::*;
        match s {
            'A' => Ok(A),
            'K' => Ok(K),
            'Q' => Ok(Q),
            'J' => Ok(J),
            'T' => Ok(T),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            _ => Err(()),
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Card::*;
        match s {
            "A" => Ok(A),
            "K" => Ok(K),
            "Q" => Ok(Q),
            "J" => Ok(J),
            "T" => Ok(T),
            "9" => Ok(Nine),
            "8" => Ok(Eight),
            "7" => Ok(Seven),
            "6" => Ok(Six),
            "5" => Ok(Five),
            "4" => Ok(Four),
            "3" => Ok(Three),
            "2" => Ok(Two),
            _ => Err(()),
        }
    }
}

type Hands = [Card; 5];
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Type {
    NoPair,
    OnePair,
    TwoPair,
    ThreeCard,
    FullHouse,
    FourCard,
    FiveCard,
}

impl From<Hands> for Type {
    fn from(value: Hands) -> Self {
        let is_five_card = || value.iter().all_equal();
        let is_four_card = || value.iter().combinations(4).any(|v| v.iter().all_equal());
        let is_three_card = || value.iter().combinations(3).any(|v| v.iter().all_equal());
        let is_full_house = || {
            let set = value.iter().collect::<HashSet<_>>();
            set.len() == 2 && is_three_card()
        };
        let is_two_pair = || {
            value
                .iter()
                .combinations(2)
                .filter(|v| v.iter().all_equal())
                .count()
                == 2
        };
        let is_one_pair = || {
            value
                .iter()
                .combinations(2)
                .filter(|v| v.iter().all_equal())
                .count()
                == 1
        };

        use Type::*;

        if is_five_card() {
            FiveCard
        } else if is_four_card() {
            FourCard
        } else if is_full_house() {
            FullHouse
        } else if is_three_card() {
            ThreeCard
        } else if is_two_pair() {
            TwoPair
        } else if is_one_pair() {
            OnePair
        } else {
            NoPair
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JokerCard {
    card: Card,
}

impl PartialOrd for JokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Card::*;
        match (&self.card, &other.card) {
            (J, J) => std::cmp::Ordering::Equal,
            (J, _) => std::cmp::Ordering::Less,
            (_, J) => std::cmp::Ordering::Greater,
            (x, y) => x.cmp(y),
        }
    }
}

impl From<Card> for JokerCard {
    fn from(value: Card) -> Self {
        Self { card: value }
    }
}

type JokerHands = [JokerCard; 5];
impl From<JokerHands> for Type {
    fn from(value: JokerHands) -> Self {
        let mut map = HashMap::new();
        for c in value {
            *map.entry(c).or_insert(0) += 1;
        }

        let j = map.remove(&JokerCard { card: Card::J });
        let max = map.iter().max_by_key(|(_, v)| **v);
        let value = match (j, max) {
            (Some(_), Some((c, _))) => {
                value.map(|v| if v.card == Card::J { c.card } else { v.card })
            }
            _ => value.map(|v| v.card),
        };

        Type::from(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
