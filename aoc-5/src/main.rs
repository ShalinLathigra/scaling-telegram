use {
    std::{
        cmp,
        env,
        fmt,
        time::Instant,
    },
    regex::Regex,
};

const SEED_REGEX: &str = r"seeds: ((\d* ?)*)";

#[derive(Debug)]
struct RangeOld {
    source_start: u64,
    dest_start: u64,
    range: u64,
}

impl fmt::Display for RangeOld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RangeOld ({}-{} -> {}-{})", self.source_start, self.source_start + self.range, self.dest_start, self.dest_start + self.range)
    }
}
impl RangeOld {
    fn convert(&self, index: u64) -> (bool, u64) {
        // println!("i:{} s:{} d:{}", index, self.source_start, self.dest_start);
        if index < self.source_start || index >= self.source_start + self.range {
            return (false, index);
        } else {
            return (true, self.dest_start + index - self.source_start);
        }
    }
}

#[derive(Debug)]
struct MapOld {
    name: String,
    // be able to sort ranges by source_start?
    ranges: Vec<RangeOld>,
    // need a method for indexing into a MapOld. Look at ranges and their coverage
    // if a RangeOld covers the source index, return the corresponding dest index
    // else, return source index
}

impl fmt::Display for MapOld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MapOld: {}\n{}", self.name, self.ranges.iter().fold(String::new(), |acc, range| acc + &format!("\t{}\n", range)))
    }
}

impl MapOld {
    fn convert(&self, index: u64) -> u64 {
        for range in &self.ranges {
            let (success, dest_index) = range.convert(index);
            if success {
                return dest_index;
            }
        }
        index
    }
}

// Gist of it is, generate a whole ass set of maps
// pass seed values in first, apply values all the way through
// find the results
// Print out the final value.
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        return;
    }
    let start = Instant::now();
    let input = match args[1].as_str() {
        "real" => include_str!("inputs/day-5.txt"),
        _ => include_str!("inputs/day-5-test.txt")
    };
    match args[2].as_str(){
        "2" => complex_solve(input),
        _ => naive_solve(input)
    };
    let duration = start.elapsed();
    println!("Time Elapsed: {:?}", duration);
}

fn naive_solve(input: &str) -> () {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps = Vec::new();
    for (i, section) in input.split("\r\n\r\n").enumerate() {
        // if i == 0, extract seeds
        if i == 0 {
            extract_seeds_old(section, &mut seeds);
            continue;
        } else {
            maps.push(parse_map_old(section));
        }
    }
    
    let mut clones: Vec<u64> = seeds.clone();
    for (_i, map) in maps.iter().enumerate() {
        for (_j, clone) in clones.iter_mut().enumerate() {
            *clone = map.convert(*clone);
        }
    }
    match clones.iter().min() {
        Some(min) => println!("min: {:?}", min),
        _ => ()
    };
}
fn extract_seeds_old(section: &str, seeds: &mut Vec<u64>) {
    let captures = Regex::new(SEED_REGEX).unwrap().captures(section);
    return match captures.unwrap().get(1) {
        Some(seed_match) => {
            let seed_data: Vec<u64> = seed_match.as_str().split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            for i in seed_data {
                seeds.push(i);
            }
        },
        None => ()
    }
}

fn parse_map_old(section: &str) -> MapOld {
    let mut map_old = MapOld {
        name: String::from(""),
        ranges: vec![]
    };
    for (i, line) in section.split("\r\n").enumerate() {
        if i == 0 {
            map_old.name = String::from(line.strip_suffix(":").unwrap());
            continue;
        }
        let range_old = parse_range_old(line);
        map_old.ranges.push(range_old);
    }
    // println!("{}", &MapOld);
    return map_old;
}

fn parse_range_old(line: &str) -> RangeOld {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let dest_start = parts[0].parse::<u64>().unwrap();
    let source_start = parts[1].parse::<u64>().unwrap();
    let range = parts[2].parse::<u64>().unwrap();
    return RangeOld {
        source_start,
        dest_start,
        range
    }
}

fn complex_solve(input: &str) -> () {
    let mut seeds: Vec<(u64, u64, bool)> = Vec::new();
    let mut maps = Vec::new();
    for (i, section) in input.split("\r\n\r\n").enumerate() {
        // if i == 0, extract seeds
        if i == 0 {
            extract_seeds(section, &mut seeds);
            continue;
        } else {
            maps.push(parse_map(section));
        }
    }
    
    // println!("{:?}", seeds);
    for (m, map) in maps.iter().enumerate() {
        println!("Applying Map: {}", m);
        for (_s, seed) in seeds.iter_mut().enumerate() {
            seed.2 = false;
        }
        // println!("{:?}", seeds);
        for (_r, range) in map.iter().enumerate() {
            let mut next_seeds: Vec<(u64, u64, bool)> = Vec::new();
            let (dest_start, source_start, range) = range;
            let (r_left, r_right) = (*source_start, *source_start + *range);
            // println!("\tRange: ({},{}) -> ({}, {})", r_left, r_right, *dest_start, *dest_start + *range);
            for (_s, seed) in seeds.iter_mut().enumerate() {
                let (s_left, s_right, traversed) = seed;
                // println!("\t\tSeed: ({},{})", s_left, s_right);
                // what component of the seed is below the range?
                if *traversed {
                    next_seeds.push(*seed);
                    continue;
                }
                if *s_left < r_left {
                    if *s_right < r_left {
                        next_seeds.push(*seed);
                        continue;
                    }
                    let right = cmp::min(*s_right, r_left - 1);
                    // println!("\t\t\tAdding Left: ({},{})", s_left, right);
                    next_seeds.push((*s_left, right, false));
                    *s_left = right + 1;
                    // println!("\t\t\tRemainder: ({},{})", s_left, s_right);
                }
                // what component of the seed is above the range?
                if *s_right > r_right {
                    if *s_left > r_right {
                        next_seeds.push(*seed);
                        continue;
                    }
                    let left = cmp::max(*s_left, r_right + 1);
                    // println!("\t\t\tAdding Right: ({},{})", left, *s_right);
                    next_seeds.push((left, *s_right, false));
                    *s_right = left - 1;
                    // println!("\t\t\tRemainder: ({},{})", s_left, s_right);
                }
                if r_left <= *s_left && *s_right <= r_right && s_left <= s_right {
                    seed.2 = true;
                    // println!("\t\t\tAdding Inside: ({},{}) -> ({},{})", s_left, s_right, *s_left + *dest_start - *source_start, *s_right + *dest_start - *source_start);
                    next_seeds.push((*s_left + *dest_start - *source_start, *s_right + *dest_start - *source_start, true));
                }
            }
            // next_seeds.append(&mut seeds.iter().filter(|s| s.2 == false).map(|s| *s).collect::<Vec<(u64, u64, bool)>>());
            seeds = next_seeds;
            // println!("{:?}", seeds);
        }
    }
    println!("{:?}", seeds.iter().min_by(|a, b| a.0.cmp(&b.0)));
}


fn extract_seeds(section: &str, seeds: &mut Vec<(u64, u64, bool)>) {
    let captures = Regex::new(SEED_REGEX).unwrap().captures(section);
    return match captures.unwrap().get(1) {
        Some(seed_match) => {
            let seed_data: Vec<u64> = seed_match.as_str().split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            let mut iter = seed_data.iter().peekable();
            while iter.peek().is_some() {
                let start = *iter.next().unwrap();
                let range = *iter.next().unwrap();
                seeds.push((start, start + range, false));
            }
        },
        None => ()
    }
}

// source_start, dest_start, applied offset
fn parse_map(section: &str) -> Vec<(u64, u64, u64)>{
    let mut map = Vec::new();
    for (i, line) in section.split("\r\n").enumerate() {
        if i > 0 {
            let range = parse_range(line);
            map.push(range);
        }
    }
    map
}

fn parse_range(line: &str) -> (u64, u64, u64) {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let dest_start = parts[0].parse::<u64>().unwrap();
    let source_start = parts[1].parse::<u64>().unwrap();
    let range = parts[2].parse::<u64>().unwrap();
    return (dest_start, source_start, range)
}