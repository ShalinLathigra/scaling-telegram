use {
    std::{
        env,
        fmt,
        time::Instant,
    },
};

const SIMPLE_TEST_ARG: &str = "simple_test";
const SIMPLE_REAL_ARG: &str = "simple_real";
const COMPLEX_TEST_ARG: &str = "complex_test";
const COMPLEX_REAL_ARG: &str = "complex_real";
static VALID_ARGS: [&str; 4] = [
    SIMPLE_TEST_ARG,
    SIMPLE_REAL_ARG,
    COMPLEX_TEST_ARG,
    COMPLEX_REAL_ARG
];

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
        print_usage();
    } else {
        match args[1].as_str() {
            SIMPLE_TEST_ARG => solve_simple(include_str!("inputs/day-4-test.txt")),
            SIMPLE_REAL_ARG => solve_simple(include_str!("inputs/day-4.txt")),
            COMPLEX_TEST_ARG => solve_complex(include_str!("inputs/day-4-test.txt")),
            COMPLEX_REAL_ARG => solve_complex(include_str!("inputs/day-4.txt")),
            _ => print_usage()
        }
    }
    println!("Elapsed: {:?}", start.elapsed());
}

fn print_usage() {
    eprintln!("Usage: cargo run {:?}", VALID_ARGS)
}

fn solve_simple(input: &str) {
    let input = str::replace(input, "  ", " ");
    let mut totals = Vec::new();
    for (i, dirty_line) in input.split("\n").enumerate() {
        let line: &str;
        match dirty_line.strip_suffix("\r") {
            Some(clean_line) => line = clean_line,
            None => line = dirty_line,
        }
        let body = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let winners = body[0].split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let numbers = body[1].split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        let mut num_winners = 0;
        for number in &numbers {
            if winners.iter().any(|&x| x == *number) {
                num_winners += 1;
            }
        }
        match num_winners {
            0 => { println!("{}: Sucker!", i)},
            _ => totals.push(2_u32.pow(num_winners - 1)),
        }
        println!("{}: Winners: {:?}", i, num_winners);
    }
    println!("Total: {}", totals.iter().sum::<u32>());
    // take in scratch card
    // Title  : Winning  | Available
    // CARD ##: ## ## .. | ## ## ..
    // Count winning numbers that appear in available
    // card value = 2^n-1 (if n > 0, else 0)
    // result = sum(card value)
    // regex == Card (\d+): ([\d\s]+) \| ([\d\s]+
}

#[derive(Clone,Copy)]
struct Card {
    card_number: u32,
    count: u32,
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}: {})", self.card_number, self.count)
        
    }
}

fn solve_complex(input: &str) {
    let input = str::replace(input, "  ", " ");
    let mut cards = Vec::new();
    for (_i, dirty_line) in input.split("\n").enumerate() {
        let line: &str;
        match dirty_line.strip_suffix("\r") {
            Some(clean_line) => line = clean_line,
            None => line = dirty_line,
        }
        let body = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let winners = body[0].split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let numbers = body[1].split(" ").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        let mut num_winners = 0;
        for number in &numbers {
            if winners.iter().any(|&x| x == *number) {
                num_winners += 1;
            }
        }
        cards.push(Card{card_number: _i as u32, count: num_winners});
    }
    
    let mut card_counts = cards.clone().iter().map(|_| 1).collect::<Vec<u32>>();
    for (i, card) in cards.iter().enumerate() {
        for j in 0..card.count {
            let index = i + (j + 1) as usize;
            // if trying to add something beyond the list, then don't
            if index > card_counts.len() {
                break;
            }
            card_counts[index] += card_counts[i];
        }
    }
    
    println!("counts:{:?}\nsum:{}", card_counts, card_counts.iter().sum::<u32>());
}