use {
    regex::Regex, std::{
        env, time::Instant
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

#[derive(Debug,Clone)]
pub struct Number {
    bounds: Bounds,
    str: String,
    val: u64,
    valid: bool,
}

impl Number {
    fn from(str: String, bounds: Bounds) -> Result<Number, String> {
        let val = str.parse::<u64>();
        if val.is_err() {
            return Err(format!("Attempted to parse bad err: {}", val.unwrap_err()).to_string())
        }
        let val = val.unwrap();
        Ok(Number{
            bounds,
            str,
            val,
            valid: false,
        })
    }
}

#[derive(Debug,Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn to_index(&self, width: &usize) -> usize {
        self.x + self.y * width
    }
}

#[derive(Debug, Clone)]
struct Bounds {
    point: Point,
    length: usize,
    skin_width: usize,
}

impl Bounds {
    pub fn overlaps(&self, target: &Bounds) -> bool {
        let (s_left, s_right, s_top, s_bot) = (
            if self.point.x > 0 {self.point.x-self.skin_width} else {0},
            self.point.x + self.length,
            self.point.y + self.skin_width,
            if self.point.y > 0 {self.point.y-self.skin_width} else {0},
        );
        let (t_left, t_right, t_top, t_bot) = (
            if target.point.x > 0 {target.point.x-target.skin_width} else {0},
            target.point.x + target.length,
            target.point.y + target.skin_width,
            if target.point.y > 0 {target.point.y-target.skin_width} else {0},
        );
        // Make sure target is not outside of the bounds of the source
        if t_left > s_right || t_right < s_left {
            return false;
        }
        // One of the two is below x bounds
        if t_top < s_bot || t_bot > s_top {
            return false;
        }
        // Within the bounds on both axes, therefore a collision has occurred
        true
    }
}

#[derive(Debug,Clone)]
struct Symbol {
    bounds: Bounds,
    ch: char,
}

impl Symbol {
    fn from(ch: char, bounds: Bounds) -> Result<Symbol, String> {
        Ok(Symbol{
            bounds,
            ch,
        })
    }
}

struct Board <> {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Board {
    pub fn create() -> Board{
        let numbers = Vec::<Number>::new();
        let symbols = Vec::<Symbol>::new();
        Board{numbers,symbols}
    }

    pub fn add_number(&mut self, bounds: Bounds, str: String) {
        let mut num = Number::from(str, bounds).unwrap();
        // println!("ADD N: {:?}", num);
        for symbol in self.symbols.iter().rev() {
            // println!("\tComparing against {} at {:?}", symbol.ch, symbol.bounds);
            if num.bounds.overlaps(&symbol.bounds) {
                num.valid = true;
                break;
            }
            if num.bounds.point.y > symbol.bounds.point.y + 1 {
                break;
            }
        }
        // println!("\t\tNum: {} valid = {}", num.val, num.valid);
        self.numbers.push(num);
        // println!("\tDONE");
    }

    pub fn add_symbol(&mut self, bounds: Bounds, ch: char) {
        let symbol = Symbol::from(ch, bounds).unwrap();
        // println!("ADD S: {:?}", symbol);
        for num in self.numbers.iter_mut().rev() {
            // println!("\tComparing against {} at {:?} ", num.val, num.bounds);
            if symbol.bounds.point.y > num.bounds.point.y + 1 {
                // gone too far
                break;
            }
            if !num.valid && symbol.bounds.overlaps(&num.bounds){
                num.valid = true;
            }
            // println!("\t\tNum: {} valid = {:?}", num.val, num.valid);
        }
        self.symbols.push(symbol);
        // println!("\tDONE");
    }
}

fn solve_3a(input: &str) {
    let mut actual: u64 = 0;
    // let expected: u64 = 4361;

    let width = input.split("\n").nth(0).unwrap().len();

    let mut board = Board::create();

    // when reading the array, how do we find the ones that are valid?
    // When we encounter a number, add it to
    let mut collected_number = String::new();
    let mut start_point = Point{x:0,y:0};
    let mut last_point = Point{x:0,y:0};
    for (y, line) in input.split("\n").enumerate(){
        let segments = line.char_indices().filter(| (_, char) | *char != '.' && !char.is_ascii_whitespace());
        for (x, ch) in segments {
            let current_point = Point{x,y};
            // finish numbers or track symbols
            if (!ch.is_numeric()) || current_point.to_index(&width) > last_point.to_index(&width) + 1 {
                if collected_number.len() > 0 {
                    board.add_number(Bounds{point: start_point.clone(), length: collected_number.len(), skin_width: 0}, collected_number.clone());
                }
                collected_number.clear();

                if !ch.is_numeric() {
                    board.add_symbol(Bounds{point: current_point.clone(), length: 1, skin_width: 1}, ch);
                }
            }
            // collect numeric chars into a string
            if ch.is_numeric(){
                if collected_number.len() == 0 {
                    start_point = Point{x,y};
                }
                collected_number.push(ch);
            }
            last_point = current_point;
        }
    }
    // make sure any numbers at the very end of the list are captured
    if collected_number.len() > 0 {
        board.add_number(Bounds{point: start_point.clone(), length: collected_number.len(), skin_width: 0}, collected_number.clone());
    }
    // if actual !
    actual = board.numbers.iter().filter(| n | n.valid).map(| c | c.val).sum();
    println!("Got: {}", actual);
    // if actual == expected {
    //     println!("SUCCESS");
    // } else {
    //     println!("FAILED got: {} expected: {}", actual, expected);
    // }
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