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
            SIMPLE_TEST_ARG => solve_simple(include_str!("inputs/day-3-test.txt")),
            SIMPLE_REAL_ARG => solve_simple(include_str!("inputs/day-3.txt")),
            COMPLEX_TEST_ARG => solve_complex(include_str!("inputs/day-3-test.txt")),
            COMPLEX_REAL_ARG => solve_complex(include_str!("inputs/day-3.txt")),
            _ => print_usage()
        }
    }
    println!("Elapsed: {:?}", start.elapsed());
}

fn print_usage() {
    eprintln!("Usage: cargo run {:?}", VALID_ARGS)
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "P({},{})", self.x, self.y)
    }
}


#[derive(Debug)]
struct NumberBuilder {
    width: usize,
    start: Point,
    string: String,
}

impl NumberBuilder {
    fn new(width: usize) -> NumberBuilder {
        NumberBuilder{width, start: Point{x: usize::MAX, y: usize::MAX}, string: String::from("START")}
    }
    
    fn index(&self) -> usize {
        if self.start.y == usize::MAX || self.start.y == usize::MAX {
            return usize::MAX - 1
        }
        self.start.x + self.start.y * self.width + self.string.len()
    }
    fn restart(&mut self, x: usize, y: usize) -> &NumberBuilder {
        self.start = Point{x, y};
        self.string.clear();
        self
    }
    
    fn save(&self) -> Result<Number, String> {
        match self.string.parse::<u32>() {
            // 145 => start index 0, end index 2, len 3
            Ok(val) => Ok(Number{
                bbox: BoundingBox {
                    min: Point{x:self.start.x, y:self.start.y},
                    max: Point{x:self.start.x + self.string.len() - 1, y:self.start.y},
                },
                valid: false,
                val
            }),
            Err(e) => Err(format!("Failed to parse: {}", self.string).to_string())
        }
    }
}

#[derive(Debug)]
struct BoundingBox {
    min: Point,
    max: Point,
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "B[{},{}]", self.min, self.max)
    }
}

#[derive(Debug)]
struct Number {
    bbox: BoundingBox,
    val: u32,
    valid: bool,
}

#[derive(Debug)]
struct Symbol {
    bbox: BoundingBox,
    ch: char,
    adjacent: Vec<u32>,
}

fn check_symbol_against_numbers(numbers: &mut Vec<Number>, symbol: &mut Symbol) {
    for number in numbers.iter_mut().rev() {
        match perform_comparison(&number.bbox, &symbol.bbox){
            (false, false) => break,
            (true, true) => {
                number.valid = true;
                if symbol.ch == '*' {
                    symbol.adjacent.push(number.val);
                }
            },
            (_,_) => {},
        }
    }
}

fn check_number_against_symbols(number: &mut Number, symbols: &mut Vec<Symbol>) {
    for symbol in symbols.iter_mut().rev() {
        match perform_comparison(&symbol.bbox, &number.bbox){
            (false, false) => break,
            (true, true) => {
                number.valid = true;
                if symbol.ch == '*' {
                    symbol.adjacent.push(number.val);
                }
            },
            (_,_) => {},
        }
    }
}

fn perform_comparison(bbox_1: &BoundingBox, bbox_2: &BoundingBox) -> (bool, bool) {
    if bbox_1.max.y < bbox_2.min.y {
        return (false, false);
    }
    if bbox_1.max.x >= bbox_2.min.x && bbox_1.min.x <= bbox_2.max.x {
        return (true, true);
    }
    return (false, true);
}

fn solve_simple(input: &str) {
    let width = input.split("\n").nth(0).unwrap().len();
    let mut builder = NumberBuilder::new(width);
    let mut numbers = Vec::<Number>::new();
    let mut symbols = Vec::<Symbol>::new();
    // println!("{:?}", builder);
    for (y, line) in input.split("\n").enumerate(){
        for (x, ch) in line.char_indices().filter(| (_, ch) | *ch != '.' && !ch.is_ascii_whitespace()){
            // Check if last one needs to close
            if !ch.is_numeric() || x + y * width != builder.index() {
                match builder.save() {
                    Ok(mut val) => {
                        // println!("New: {:?}", val);
                        check_number_against_symbols(&mut val, &mut symbols);
                        numbers.push(val);
                    },
                    _ => {}
                }
                builder.restart(x, y);
            }
            if ch.is_numeric() {
                builder.string.push(ch);
            } else {
                let mut symbol = Symbol{
                    bbox: BoundingBox{
                        min: Point{x: if x > 0 {x-1} else {0}, y: if y > 0 {y-1} else {0}},
                        max: Point{x: x+1, y: y+1},
                    },
                    ch,
                    adjacent: Vec::<u32>::new(),
                };
                check_symbol_against_numbers(
                    &mut numbers,
                    &mut symbol,
                );
                symbols.push(symbol);
                // println!("New: {:?}", symbols.last().unwrap());
            }
        }
    }
    match builder.save() {
        Ok(mut val) => {
            check_number_against_symbols(&mut val, &mut symbols);
            numbers.push(val);
            // println!("New: {:?}", val);
        },
        _ => {}
    }
    
    println!("Found: {} numbers ({} valid) and {} symbols", numbers.len(), numbers.iter().filter(|n| n.valid).count(), symbols.len());
    println!("Sum of valid: {}", numbers.iter().filter(|n| n.valid).map(|n| n.val).sum::<u32>())
}


fn solve_complex(input: &str) {
    let width = input.split("\n").nth(0).unwrap().len();
    let mut builder = NumberBuilder::new(width);
    let mut numbers = Vec::<Number>::new();
    let mut symbols = Vec::<Symbol>::new();
    // println!("{:?}", builder);
    for (y, line) in input.split("\n").enumerate(){
        for (x, ch) in line.char_indices().filter(| (_, ch) | *ch != '.' && !ch.is_ascii_whitespace()){
            // Check if last one needs to close
            if !ch.is_numeric() || x + y * width != builder.index() {
                match builder.save() {
                    Ok(mut val) => {
                        // println!("New: {:?}", val);
                        check_number_against_symbols(&mut val, &mut symbols);
                        numbers.push(val);
                    },
                    _ => {}
                }
                builder.restart(x, y);
            }
            if ch.is_numeric() {
                builder.string.push(ch);
            } else {
                let mut symbol = Symbol{
                    bbox: BoundingBox{
                        min: Point{x: if x > 0 {x-1} else {0}, y: if y > 0 {y-1} else {0}},
                        max: Point{x: x+1, y: y+1},
                    },
                    ch,
                    adjacent: Vec::<u32>::new(),
                };
                check_symbol_against_numbers(
                    &mut numbers,
                    &mut symbol,
                );
                symbols.push(symbol);
                // println!("New: {:?}", symbols.last().unwrap());
            }
        }
    }
    match builder.save() {
        Ok(mut val) => {
            check_number_against_symbols(&mut val, &mut symbols);
            numbers.push(val);
            // println!("New: {:?}", val);
        },
        _ => {}
    }
    
    println!("Found: {} numbers and {} symbols ({} gears)", numbers.len(), symbols.len(), symbols.iter().filter(|s| s.adjacent.len() == 2).count());
    println!("Sum of gears: {}", symbols.iter().filter(|s| s.adjacent.len() == 2).map(|s| s.adjacent[0] * s.adjacent[1]).sum::<u32>())
}