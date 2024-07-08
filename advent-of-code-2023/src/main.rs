use {
    regex::Regex, std::{
        env, ops::Index, time::Instant
        }
};

fn main() {
    // implement vecdeque?
    // Sounds like a good idea
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Must specify an index!");
        return;
    }

    // for each element in args after first:
    for argument in &args[1 .. args.len()] {
        let start = Instant::now();
        match argument.as_str() {
            "1a" => solve_1a(),
            "1b" => solve_1b(),
            "2a" => solve_2a(),
            "2b" => solve_2b(),
            "3t" => solve_3a(include_str!("inputs/day-3-test.txt")),
            "3a" => solve_3a(include_str!("inputs/day-3.txt")),
            _ => println!("Input: [{}] not recognized. Skipping...", argument)
        }
        let duration = start.elapsed();
        println!("Time Elapsed: {:?}", duration);
    }
}

/* DAY 1 */
fn solve_1a() {
    let text_input: &str = include_str!("inputs/day-1.txt");
    let mut number_vec = Vec::new();
    for line in text_input.split("\n"){
        let chars: Vec<char> = line.chars().collect();
        match extract_numeric(chars) {
            Ok(some_value) => {number_vec.push(some_value)},
            _ => {eprintln!("Failed to extract value from {} as int", line)}
        }
    }
    // print sum of all values in vec
    let result: u64 = number_vec.into_iter().sum();
    println!("Day 1: Naive Sum of all inputs is: {:?}", result);
}
fn extract_numeric(chars: Vec<char>) -> Result<u64, std::num::ParseIntError> {
    let mut left_index = 0;
    let mut right_index = chars.len() - 1;
    while left_index < chars.len() && right_index > 0 {
        if !chars[left_index].is_numeric(){
            left_index += 1;
        }
        if !chars[right_index].is_numeric(){
            right_index -= 1;
        }
        if chars[left_index].is_numeric() && chars[right_index].is_numeric(){
            break;
        }
    }
    let mut num_str: String = String::new();
    num_str.push(chars[left_index]);
    num_str.push(chars[right_index]);
    num_str.parse::<u64>()
}

fn solve_1b() {
    let text_input: &str = include_str!("inputs/day-1.txt");
    let mut number_vec = Vec::new();
    for line in text_input.split("\n"){
        // println!("line:{}", line);
        match extract_numeric_complex(&line) {
            Ok(some_value) => {println!("\tPASS: {}", some_value); number_vec.push(some_value)},
            // Ok(some_value) => {},
            Err(msg) => {eprintln!("\tFAILED: {}", msg)}
        }
    }
    // print sum of all values in vec
    let result: u64 = number_vec.into_iter().sum();
    println!("Day 1: Complex Sum of all inputs is: {:?}", result);
}

// define viable words
static NUM_STRS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
fn extract_numeric_complex(line: &str) -> Result<u64, String> {
    let chars: Vec<char> = line.chars().collect();
    let mut left_index = 0;
    let mut left_word = "";
    let mut left_is_int = false;
    let mut right_offset = 1;
    let mut right_word = "";
    let mut right_is_int = false;
    while left_index < chars.len() && right_offset <= chars.len(){
        left_is_int = chars[left_index].is_numeric();
        right_is_int = chars[chars.len() - right_offset].is_numeric();
        // if exit?
        if (left_word != "" || left_is_int) && (right_word != "" || right_is_int) {
            break;
        }
        if !left_is_int && left_word == ""{
            for i in 0..9 {
                if matches_substr(&line, left_index, NUM_STRS[i], false).unwrap() {
                    left_word = NUM_STRS[i];
                    break;
                }
            }
            left_index += 1;
        }

        if !right_is_int && right_word == ""{
            for i in 0..9 {
                if matches_substr(&line, right_offset, NUM_STRS[i], true).unwrap() {
                    right_word = NUM_STRS[i];
                    break;
                }
            }
            right_offset += 1;
        }
    }

    let left_val;
    if left_word != "" {
        left_val = text_to_int(&left_word).unwrap();
    } else if left_is_int {
        left_val = String::from(chars[left_index]).parse::<u64>().unwrap();
    } else {
        return Err(format!("Failed to find left num or string in: {}", line).to_string());
    }

    let right_val;
    if right_word != "" {
        right_val = text_to_int(&right_word).unwrap();
    } else if right_is_int {
        right_val = String::from(chars[chars.len() - right_offset]).parse::<u64>().unwrap();
    } else {
        return Err(format!("Failed to find right num or string in: {}", line).to_string());
    }

    Ok(left_val * 10 + right_val)

}

/* DAY 2 */

fn solve_2a() {
    let test_input = include_str!("inputs/day-2.txt");
    let game_re = Regex::new(r"Game (\d*): (.+)\n?").unwrap();
    let match_re = Regex::new(r"(?<count>\d*) (?<colour>green|red|blue)").unwrap();
    let mut sum = 0;
    // let mut results = vec![];
    for (_, [game, body]) in game_re.captures_iter(test_input).map(|c| c.extract()) {
        // game is string holding a number
        // body is the whole text of the game
        let mut possible: bool = true;
        for round in body.split("; "){
            let matches = match_re.captures_iter(round);
            for capture in matches {
                let count = &capture["count"].parse::<u64>().unwrap();
                match &capture["colour"] {
                    "red" => possible &= *count <= 12,
                    "green" => possible &= *count <= 13,
                    "blue" => possible &= *count <= 14,
                    _ => println!("Matching colour {}: {}", &capture["colour"], &capture["count"].parse::<u64>().unwrap())
                }
            }
        }
        if possible {
            println!("game: {} is possible", game);
            sum += game.parse::<u64>().unwrap();
        }
    }

    println!("Sum of possible IDs is: {}", sum);
}

fn solve_2b() {
    let test_input = include_str!("inputs/day-2.txt");
    let game_re = Regex::new(r"Game (\d*): (.+)\n?").unwrap();
    let match_re = Regex::new(r"(?<count>\d*) (?<colour>green|red|blue)").unwrap();
    let mut sum = 0;
    // let mut results = vec![];
    for (_, [game, body]) in game_re.captures_iter(test_input).map(|c| c.extract()) {
        // game is string holding a number
        // body is the whole text of the game
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in body.split("; "){
            let matches = match_re.captures_iter(round);
            for capture in matches {
                let count = &capture["count"].parse::<u64>().unwrap();
                match &capture["colour"] {
                    "red" => min_red = max(min_red, *count) ,
                    "green" => min_green = max(min_green, *count) ,
                    "blue" => min_blue = max(min_blue, *count) ,
                    _ => println!("Matching colour {}: {}", &capture["colour"], &capture["count"].parse::<u64>().unwrap())
                }
            }
        }
        println!("{} : ({},{},{}) => {}", game, min_red, min_green, min_blue, min_red * min_blue * min_green);
        sum += min_red * min_blue * min_green;
    }

    println!("Sum of game powers is: {}", sum);
}

/* DAY 3 */

// structured data.
// Struct representing the map, contains a list numbers and a list of symbols
// numbers have a starting point and length, string representation, and a value
//  numbers can be created and extended
// symbols are coords + a symbol
// Game board is created by reading in "input"
// it starts from top left, progresses through
// to bottom right
// Numbers are created and added sequentially
// Same with symbols
// two different 2d arrays

#[derive(Debug)]
pub struct Number {
    point: Point,
    str: String,
    val: u16,
}

impl Number {
    fn from(str: String, point: Point) -> Result<Number, String> {
        let val = str.parse::<u16>();
        if val.is_err() {
            return Err(format!("Attempted to parse bad err: {}", val.unwrap_err()).to_string())
        }
        let val = val.unwrap();
        Ok(Number{
            point,
            str,
            val,
        })
    }
}

#[derive(Debug,Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn make(&self, width: &usize) -> usize {
        self.x + self.y * width
    }
}

#[derive(Debug,Clone)]
struct Symbol {
    point: Point,
    ch: char,
}

impl Symbol {
    fn from(ch: char, point: Point) -> Result<Symbol, String> {
        Ok(Symbol{
            point,
            ch,
        })
    }
}



fn solve_3a(input: &str) {
    let actual = 0;
    let expected = 4361;

    let width = input.split("\n").nth(0).unwrap().len();

    // when reading the array, how do we find the ones that are valid?
    // When we encounter a number, add it to
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut collected_number = String::new();
    let mut start_index = Point{x:0,y:0};
    let mut last_index = Point{x:0,y:0};
    for (y, line) in input.split("\n").enumerate(){
        let segments = line.char_indices().filter(| (_, char) | *char != '.' && !char.is_ascii_whitespace());
        for (x, char) in segments {
            let index = Point{x,y};
            // if char is not numeric or > last_index + 1
            // tie off numbers, track symbols
            if (!char.is_numeric()) || index.make(&width) > last_index.make(&width) + 1 {
                if collected_number.len() > 0 {
                    let num = Number::from(collected_number.clone(), start_index.clone()).unwrap();
                    println!("FOUND {:?}", num);
                    numbers.push(num);
                }
                collected_number.clear();

                if !char.is_numeric() {
                    let symbol = Symbol::from(char, index.clone()).unwrap();
                    // create game board that has the add_symbol and add_number functions
                    // add_symbol will check over all numbers in reverse order (current row, last row)
                        // once hit number that is before last row, skip
                    // add_number will check over all symbols in reverse order (current row, last row)
                        // once hit symbol that is before last row, skip
                    // add "in bounds" function to compare two bounding boxes
                    println!("FOUND {:?}", symbol);
                    symbols.push(symbol);
                }
            }
            if char.is_numeric(){
                if collected_number.len() == 0 {
                    start_index = Point{x,y};
                }
                collected_number.push(char);
            }
            last_index = index;
        }
    }
    if collected_number.len() > 0 {
        let new_num = Number::from(collected_number.clone(), start_index.clone()).unwrap();
        println!("FOUND {:?}", new_num);
        numbers.push(new_num);
    }
    // if actual !
    if actual == expected {
        println!("SUCCESS");
    } else {
        println!("FAILED");
    }
}

/* UTILS */
fn max(a: u64, b: u64) -> u64 {
    println!("comparing {} and {}", a, b);
    if a > b {
        return a;
    }
    b
}

fn text_to_int (text: &str) -> Result<u64, String> {
    return match text {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => {
            Err(format!("Failed converting: {}", text).to_string())
        },
    };
}

fn matches_substr(source: &str, start_offset: usize, target: &str, reverse: bool) -> Result<bool, String> {
    if source.len() < target.len() + start_offset {
        return Ok(false);
    }
    let subject: &str;
    if reverse {
        // "two1nine" vs "five" from offset 2 in reverse
        // initially, comparing nine vs five, start_offset is 0, range is source.len()-target.len() .. source.len()-1
        subject = &source[source.len() - target.len() - start_offset .. source.len() - start_offset ];
    } else {
        subject = &source[start_offset.. start_offset + target.len()];
    }
    Ok(subject.starts_with(target))
}