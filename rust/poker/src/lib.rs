use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
pub enum HandType {
    Empty,
    High(Vec<u8>),
    OnePair(Vec<u8>),
    TwoPairs(Vec<u8>),
    ThreeOfAKind(Vec<u8>),
    Straight(u8),
    Flush(Vec<u8>),
    FullHouse(Vec<u8>),
    FourOfAKind(Vec<u8>),
    StraightFlush(u8),
    FiveOfAKind(u8),
}

#[derive(Debug, Eq)]
pub struct Hand<'a> {
    kind: HandType,
    handstr: &'a str,
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind.cmp(&other.kind)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl<'a> Default for Hand<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Hand<'a> {
    fn parse_card_value(card: &str) -> u8 {
        match card.chars().next().unwrap() {
            i if char::is_numeric(i) => match i.to_digit(10).unwrap() {
                n if n == 1 => 10,
                n => n as u8,
            },
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        }
    }

    fn parse_card_suit(card: &'a str) -> char {
        card.chars().rev().next().unwrap()
    }

    fn count_pairs_and_triplets(vals: &[u8]) -> (u8, u8, u8) {
        let mut counts: HashMap<u8, u8> = HashMap::new();

        vals.iter().for_each(|&n| {
            let c = counts.entry(n).or_insert(0);
            *c += 1;
        });

        let mut pairs = 0;
        let mut triplets = 0;
        let mut quads = 0;

        counts.values().for_each(|&val| {
            match val {
                2 => pairs += 1,
                3 => triplets += 1,
                4 => quads += 1,
                _ => {}
            };
        });
        (pairs, triplets, quads)
    }

    fn sort_hand(vals: &[u8]) -> Vec<u8> {
        let mut counts: HashMap<u8, u8> = HashMap::new();

        vals.iter().for_each(|&n| {
            let c = counts.entry(n).or_insert(0);
            *c += 1;
        });

        let mut counted_vec: Vec<(usize, u8)> =
            counts.iter().map(|(&k, &v)| (v as usize, k)).collect();

        // sort by count, then rank value
        counted_vec.sort_unstable();
        counted_vec.reverse();

        let mut sorted_vec: Vec<u8> = Vec::new();
        for (c, v) in counted_vec {
            sorted_vec.append(&mut vec![v; c]);
        }

        println!("{:?}", sorted_vec);

        sorted_vec
    }

    fn evaluate_hand(mut vals: Vec<u8>, mut suits: Vec<char>) -> HandType {
        const STRAIGHTS: &[u8] = &[14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 14, 5, 4, 3, 2];

        suits.sort_unstable();
        suits.dedup();
        let is_flush = suits.len() == 1;

        vals.sort_unstable();
        vals.reverse();
        // println!("{:?}", vals);
        let maxval = *vals.get(0).unwrap();

        // reduced is a vector of unique values in the hand, in descending order
        let reduced = Hand::sort_hand(&vals);

        if reduced.len() == 1 {
            return HandType::FiveOfAKind(maxval);
        }

        // check for straights
        if STRAIGHTS.windows(5).any(|s| s == vals.as_slice()) {
            if vals.contains(&14) && vals.contains(&5) {
                // low straight with A
                if is_flush {
                    return HandType::StraightFlush(5);
                } else {
                    return HandType::Straight(5);
                }
            } else if is_flush {
                return HandType::StraightFlush(maxval);
            } else {
                return HandType::Straight(maxval);
            }
        }

        // if it's not a straight, and it's a flush, it's just a flush
        if is_flush {
            return HandType::Flush(reduced);
        }

        match Hand::count_pairs_and_triplets(&vals) {
            (_, _, 1) => HandType::FourOfAKind(reduced),
            (1, 0, _) => HandType::OnePair(reduced),
            (1, 1, _) => HandType::FullHouse(reduced),
            (2, 0, _) => HandType::TwoPairs(reduced),
            (0, 1, _) => HandType::ThreeOfAKind(reduced),
            _ => HandType::High(reduced),
        }
    }

    fn process_hand(handlist: Vec<&str>) -> HandType {
        let vals: Vec<u8> = handlist
            .iter()
            .map(|card| Hand::parse_card_value(card))
            .collect();
        // println!("vals of {:?} are {:?}", handlist, vals);

        let suits: Vec<char> = handlist
            .iter()
            .map(|card| Hand::parse_card_suit(card))
            .collect();
        // println!("suits of {:?} are {:?}", handlist, suits);

        Hand::evaluate_hand(vals, suits)
    }

    pub fn from_handstr(handstr: &'a str) -> Self {
        let mut hand = Hand::new();
        hand.handstr = handstr;

        let handlist: Vec<&str> = handstr.split_whitespace().collect();
        hand.kind = Hand::process_hand(handlist);

        hand
    }

    pub fn new() -> Self {
        Hand {
            kind: HandType::Empty,
            handstr: "",
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands = hands
        .iter()
        .map(|h| Hand::from_handstr(h))
        .collect::<Vec<Hand>>();
    hands.sort_unstable();

    println!("{:?}", hands);
    let best_hand = hands.iter().max().unwrap();

    Some(
        hands
            .iter()
            .filter(|&h| h == best_hand)
            .map(|h| h.handstr)
            .collect::<Vec<&'a str>>(),
    )
}
