#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum HandType {
    High(u8),
    OnePair(u8),
    TwoPairs(u8, u8),
    ThreeOfAKind(u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8),
    FourOfAKind(u8),
    StraightFlush(u8),
    FiveOfAKind(u8),
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Hand<'a> {
    handstr: &'a str,
    kind: HandType,
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

    fn evaluate_hand(mut vals: Vec<u8>, mut suits: Vec<char>) -> HandType {
        suits.dedup();
        let is_flush = suits.len() == 1;

        let maxval = *vals.iter().rev().collect();
        vals.sort();

        HandType::High(maxval)
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

    pub fn new(handstr: &'a str) -> Self {
        let handlist: Vec<&str> = handstr.split_whitespace().collect();
        // println!("Read: {:?}", handlist);

        // handlist is now a list of string slices containing card info
        // ["4D", "5S", "6S", "8D", "3C"]

        // We want to process this into an enum
        let handkind = Hand::process_hand(handlist);

        Hand {
            handstr: handstr,
            kind: handkind,
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    hands.iter().map(|h| Hand::new(h)).collect::<Vec<Hand>>();

    // Hand Ord check
    // println!(
    //     "{}",
    //     Hand {
    //         handstr: "",
    //         kind: HandType::TwoPairs(3,1)
    //     } > Hand {
    //         handstr: "",
    //         kind: HandType::TwoPairs(3,2)
    //     }
    // );

    Some(hands.to_vec())
}
