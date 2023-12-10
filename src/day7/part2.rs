use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_type(hand: &str) -> HandType {
    let mut buckets: HashMap<char, usize> = HashMap::new();

    for char in hand.chars() {
        *buckets.entry(char).or_insert(0) += 1;
    }

    match buckets.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        3 => {
            if buckets.values().any(|&n| n == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        2 => {
            if buckets.values().any(|&n| n == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        1 => HandType::FiveOfAKind,
        _ => panic!("wtf"),
    }
}

fn get_hand_type_joker(hand: &str) -> HandType {
    let mut possible_hands: Vec<HandType> = vec![];
    let mut next: Vec<String> = vec![hand.to_owned()];

    while !next.is_empty() {
        let hand = next.pop().unwrap();

        match hand.split_once("J") {
            None => possible_hands.push(get_hand_type(&hand)),
            Some((prefix, suffix)) => {
                let possible = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
                    .iter()
                    .map(|c| prefix.to_owned() + &c.to_string() + suffix)
                    .collect::<Vec<String>>();
                for hand in possible {
                    next.push(hand);
                }
            }
        }
    }

    possible_hands.sort_by(|a, b| a.cmp(b));

    *possible_hands.last().unwrap()

    // let possible_hands = hand
    //     .chars()
    //     .flat_map(|c| {
    //         if c == 'J' {
    //             vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].iter()
    //         } else {
    //             vec![c].iter()
    //         }
    //     })
    //     .collect::<Vec<String>>();

    //possible_hands.sort_by(|(a, b)| a)
}

fn get_score_of_card(a: &char) -> u32 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("Not a valid card {}", a),
    }
}

fn compare_hand(a: &str, b: &str) -> Ordering {
    let result = a
        .chars()
        .zip(b.chars())
        .map(|(a, b)| {
            let score_a = get_score_of_card(&a);
            let score_b = get_score_of_card(&b);
            score_a.cmp(&score_b)
        })
        .find(|&ordering| ordering != Ordering::Equal)
        .unwrap_or(Ordering::Equal);

    result
}

fn main() {
    let input = include_str!("./input.txt");

    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();

            (get_hand_type_joker(hand), hand, bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<(HandType, &str, u32)>>();

    hands.sort_by(
        |(type_a, hand_a, _), (type_b, hand_b, _)| match type_a.cmp(type_b) {
            std::cmp::Ordering::Equal => compare_hand(hand_a, hand_b),
            x => x,
        },
    );

    let total_winning: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.2)
        .sum();

    dbg!(total_winning);
}
