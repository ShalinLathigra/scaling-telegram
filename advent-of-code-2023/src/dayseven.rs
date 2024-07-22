use crate::*;

pub fn solve_a(context: &context::Context) -> Option<bool>{
    println!("Solving Day 7a\nParsing Input");
    let splits = context.input.split_ascii_whitespace().collect::<Vec<&str>>();
    // let mut hands: Vec<(u8, u64, u64, &str)> = splits.chunks(2).map(| c | parse_hand_from(c[0], c[1])).collect();
    let mut hands: Vec<(u8, u64, u64)> = splits.chunks(2).map(| c | parse_hand_from_a(c[0], c[1])).collect();
    // sort by
    println!("Sorting Chunks");
    hands.sort_by(| (type_1, value_1, _), (type_2, value_2, _) | {
            let type_order = type_1.partial_cmp(type_2).unwrap();
            match type_order {
                std::cmp::Ordering::Equal => return value_1.partial_cmp(value_2).unwrap(),
                _ => type_order,
        }
    });
    for (hand_type, value, bid) in hands.iter() {
        println!("chunk: ({}, {:#07X}, {})", hand_type, value, bid);
    }
    println!("Total winnings: {}", hands.iter().enumerate().map(| (i, (_, _, b)) | (i + 1) as u64 * b).sum::<u64>());
    Some(true)
}

fn parse_hand_from_a<'a>(hand: &'a str, bid: &str) -> (u8, u64, u64) {//, &'a str) {
    // starting with a hand likely of length 5
    // for each char in hand (right to left)
    // sum = hand[end-i-1].parse() * 16^i
    let hand_type: u8;
    let mut raw_value = 0;
    let mut counts: [u8; 13] = [0,0,0,0,0,0,0,0,0,0,0,0,0];
    let len = hand.len();
    for (i, ch) in hand.char_indices().rev() {
        raw_value += 16_u64.pow((len - i - 1) as u32) * match ch {
            'A' => {counts[0xC] += 1; 0xC},
            'K' => {counts[0xB] += 1; 0xB},
            'Q' => {counts[0xA] += 1; 0xA},
            'J' => {counts[0x9] += 1; 0x9},
            'T' => {counts[0x8] += 1; 0x8},
            '9' => {counts[0x7] += 1; 0x7},
            '8' => {counts[0x6] += 1; 0x6},
            '7' => {counts[0x5] += 1; 0x5},
            '6' => {counts[0x4] += 1; 0x4},
            '5' => {counts[0x3] += 1; 0x3},
            '4' => {counts[0x2] += 1; 0x2},
            '3' => {counts[0x1] += 1; 0x1},
            '2' => {counts[0x0] += 1; 0x0},
            _ => 15
        }
    }
    // parse out max and second max
    let mut max = 0;
    let mut second_max = 0;
    for i in 0..counts.len() {
        if counts[i] > max {
            (second_max, max) = (max, counts[i]);
        } else if counts[i] > second_max {
            second_max = counts[i];
        }
    }
    hand_type = match (max, second_max) {
        (3, 2) => 4, // full house
        (3, _) => 3, // 3 of a kind
        (2, 2) => 2, // two pair else one pair
        (2, _) => 1, // two pair else one pair
        (1, _) => 0, // single high
        (other, _) => other + 1, // 4 or 5 of a kind => 5 or 6
    };
    (hand_type, raw_value, bid.parse::<u64>().unwrap())//, hand)
}


pub fn solve_b(context: &context::Context) -> Option<bool> {
    println!("Solving Day 7b\nParsing Input");
    let splits = context.input.split_ascii_whitespace().collect::<Vec<&str>>();
    let mut hands: Vec<(u8, u64, u64, &str)> = splits.chunks(2).map(| c | parse_hand_from_b(c[0], c[1])).collect();
    // sort by
    println!("Sorting Chunks");
    hands.sort_by(| (type_1, value_1, _, _), (type_2, value_2, _, _) | {
            let type_order = type_1.partial_cmp(type_2).unwrap();
            match type_order {
                std::cmp::Ordering::Equal => return value_1.partial_cmp(value_2).unwrap(),
                _ => type_order,
        }
    });
    for (hand_type, value, bid, hand) in hands.iter() {
        println!("chunk: {hand} ({}, {:#07X}, {})", hand_type, value, bid);
    }
    println!("Total winnings: {}", hands.iter().enumerate().map(| (i, (_, _, b, _)) | (i + 1) as u64 * b).sum::<u64>());
    Some(true)
}

fn parse_hand_from_b<'a>(hand: &'a str, bid: &str) -> (u8, u64, u64, &'a str) {
    // starting with a hand likely of length 5
    // for each char in hand (right to left)
    // sum = hand[end-i-1].parse() * 16^i
    let hand_type: u8;
    let mut raw_value = 0;
    let mut counts: [u8; 13] = [0,0,0,0,0,0,0,0,0,0,0,0,0];
    let len = hand.len();
    for (i, ch) in hand.char_indices().rev() {
        raw_value += 16_u64.pow((len - i - 1) as u32) * match ch {
            'A' => {counts[0xC] += 1; 0xC},
            'K' => {counts[0xB] += 1; 0xB},
            'Q' => {counts[0xA] += 1; 0xA},
            'T' => {counts[0x9] += 1; 0x9},
            '9' => {counts[0x8] += 1; 0x8},
            '8' => {counts[0x7] += 1; 0x7},
            '7' => {counts[0x6] += 1; 0x6},
            '6' => {counts[0x5] += 1; 0x5},
            '5' => {counts[0x4] += 1; 0x4},
            '4' => {counts[0x3] += 1; 0x3},
            '3' => {counts[0x2] += 1; 0x2},
            '2' => {counts[0x1] += 1; 0x1},
            'J' => {counts[0x0] += 1; 0x0},
            _ => 15
        }
    }
    // parse out max and second max
    let mut max = 0;
    let mut second_max = 0;
    for i in 1..counts.len() {
        if counts[i] > max {
            (second_max, max) = (max, counts[i]);
        } else if counts[i] > second_max {
            second_max = counts[i];
        }
    }

    // scenarios, if max, second max are equal, then any jokers turn it into a full house
    // otherwise, you're just gonna take whatever you get and add it

    /*
    i.e., if I have a high card and 2, I'd go for 3 of a kind. Pair and 2, better to go straight for 4 of a kind
    if I have 2 pair and 1, then full house, otherwise, I'm instead going for just max + j
     */
    let jokers = counts[0];
    if max == 2 && max == second_max && jokers > 0 {
        hand_type = 4; // full house
    } else {
        // single high => 0 + jokers
        // pair => 1 + jokers
        // 3 of a kind or full house => 2 + jokers

        hand_type = match (max, second_max, jokers) {
            (3, 2, 0) => 4, // full house
            (3, _, 0) => 3, // 3 of a kind
            (3, _, j) => 4 + j, // secret 4-5 of a kind
            (2, 2, 1) => 4, // Secret 4 of a kind
            (2, 2, _) => 2, // two pair
            (2, _, 0) => 1, // one pair
            (2, _, 1) => 3, // three of a kind
            (2, _, j) => 3 + j, // Secret 4-5 of a kind (with j >= 2)
            (1, _, 0) => 0, // single high
            (1, _, 1) => 1, // single high
            (1, _, 2) => 3, // single high
            (1, _, j) => 2 + j, // Secret 4-5 of a kind (with j >= 3)
            (four_or_five, _, j) => four_or_five + 1 + j, // 4 or 5 of a kind => 5 or 6
        };
    }
    (hand_type, raw_value, bid.parse::<u64>().unwrap(), hand)
}
