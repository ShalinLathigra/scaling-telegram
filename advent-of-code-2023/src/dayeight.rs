use crate::*;
use regex::Regex;
use std::collections::HashMap;

pub fn solve_a(context: &context::Context) -> Option<bool>{
    println!("Solving Day 8a");
    println!("Parsing Input");

    let splits = context.input.split("\r\n").collect::<Vec<&str>>();
    let directions = (*splits.iter().nth(0)?).chars().collect::<Vec<char>>();
    let re = Regex::new(r"(?<node>\w*) = \((?<left>\w*), (?<right>\w*)\)").unwrap();

    let mut tree = Vec::new();
    let mut added = Vec::new();
    println!("Assembling Set");
    for (i, chunk) in splits.iter().enumerate().skip(2) {
        let caps = re.captures(chunk)?;
        let (node, left, right) = (
            caps.name("node").map_or("".to_string(), |m| m.as_str().to_string()),
            caps.name("left").map_or("".to_string(), |m| m.as_str().to_string()),
            caps.name("right").map_or("".to_string(), |m| m.as_str().to_string())
        );
        // if node doesn't exist, add it
        let node_index = match added.iter().position(|s: &String| *s == node) {
            Some(index) => index,
            _ => {
                added.push(node);
                tree.push((added.len() - 1, usize::MAX, usize::MAX));
                added.len() - 1
            },
        };

        // Find or Add left point
        match added.iter().position(|s: &String| *s == left) {
            Some(index) => {
                tree[node_index].1 = index;
            },
            _ => {
                added.push(left);
                tree.push((added.len() - 1, usize::MAX, usize::MAX));
                tree[node_index].1 = added.len() - 1;
            },
        };

        // Find or Add right point
        match added.iter().position(|s: &String| *s == right) {
            Some(index) => {
                tree[node_index].2 = index;
            },
            _ => {
                added.push(right);
                tree.push((added.len() - 1, usize::MAX, usize::MAX));
                tree[node_index].2 = added.len() - 1;
            },
        };
        println!("{}: {}({}) -> {}({}), {}({})", i - 1, added[node_index], node_index, added[tree[node_index].1], tree[node_index].1, added[tree[node_index].2], tree[node_index].2);
    }

    println!("Finding start index");
    let mut current_index = match added.iter().position(|s| *s == "AAA") {
        Some(index) => index,
        _ => usize::MAX
    };
    println!("Finding target index");
    let target_index = match added.iter().position(|s| *s == "ZZZ") {
        Some(index) => index,
        _ => usize::MAX
    };

    if target_index == usize::MAX {
        return Some(false);
    }
    // println!("{:?}", tree);
    println!("Searching for ZZZ: {}", target_index);
    let mut dir_index: usize = 0;
    let mut num_steps: u32 = 0;
    while current_index != target_index {
        let (_, left, right) = tree[current_index];
        match directions[dir_index] {
            'L' => {
                // println!("Current: {} {} ({}, {}) -> Left: {} {}", added[current_index], current_index, added[current], current, added[left], left);
                current_index = left
            },
            'R' => {
                // println!("Current: {} {} ({}, {}) -> Right: {} {}", added[current_index], current_index, added[current], current, added[right], right);
                current_index = right
            },
            _ => break,
        }
        println!("{num_steps}");
        num_steps += 1;
        dir_index = (dir_index + 1) % directions.len();
    }

    println!("Found target in: {} steps", num_steps);
    Some(true)
}


fn parse_input(input: &str) -> Option<(HashMap<&str, (&str, &str)>, Vec<char>)> {
    let mut map = HashMap::new();

    let splits = input.split("\r\n").collect::<Vec<&str>>();
    let directions = (*splits.iter().nth(0)?).chars().collect::<Vec<char>>();
    let re = Regex::new(r"(?<node>\w*) = \((?<left>\w*), (?<right>\w*)\)").unwrap();
    println!("Assembling Set");
    for (_i, chunk) in splits.iter().enumerate().skip(2) {
        let caps = re.captures(chunk)?;
        let (node, left, right) = (
            caps.name("node").map( | m | m.as_str()).unwrap(),
            caps.name("left").map( | m | m.as_str()).unwrap(),
            caps.name("right").map( | m | m.as_str()).unwrap()
        );
        map.insert(node, (left, right));
    }

    Some((map, directions))
}

pub fn solve_b(context: &context::Context) -> Option<bool> {
    println!("Solving Day 8b");
    println!("Parsing Input");

    let (graph, directions) = parse_input(&context.input)?;
    let start_indices = graph.keys().filter(|k| k.ends_with("A")).map(|s| s.to_string()).collect::<Vec<String>>();
    let path_lengths = start_indices.iter().map(|i| traverse_graph(i, &graph, &directions).unwrap_or(u64::MAX)).collect::<Vec<u64>>();
    println!("{:?}\n{:?}", start_indices, path_lengths);
    let sum_length = path_lengths.into_iter().reduce(| acc, b | lcm(acc, b)).unwrap();
    println!("{sum_length}");
    Some(true)
}

fn gcd(a: u64, b: u64) -> u64 {
    match a % b {
        0 => b,
        r => gcd(b, r)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b)
}

fn traverse_graph(start: &str, graph: &HashMap<&str, (&str, &str)>, directions: &Vec<char>) -> Option<u64> {
    let mut current = start;
    let mut count = 0;
    let mut dir_index = 0;
    loop {
        count += 1;
        current = match directions[dir_index] {
            'L' => graph[current].0,
            'R' => graph[current].1,
            _ => return None,
        };
        if current.ends_with("Z") {
            return Some(count);
        }
        dir_index = (dir_index + 1) % directions.len();
    }
}