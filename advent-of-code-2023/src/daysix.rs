use crate::*;

pub fn solve_a(context: &context::Context) -> Option<bool>{
    println!("Solving Day 6a using context:\n{}", context.input);
    let races = parse_times_and_distances_simple(&context.input);
    let mut product = 1;
    for (i, (time, to_beat)) in races.iter().enumerate() {
        let mut ways_to_win = 0;
        for hold_duration in 1..*time {
            // considered a win if speed(hold_duration) * time - hold_duration > to_beat
            if hold_duration * (time - hold_duration) > *to_beat {
                // println!("Time: {} Dist: {} Beat By: {}", time, to_beat, hold_duration);
                ways_to_win += 1;
            }
        }
        println!("RACE: {} Time: {}ms, Dist: {}mm Winning Times: {}", i, time, to_beat, ways_to_win);
        product *= ways_to_win;
    }
    println!("Product: {}", product);
    Some(true)
}

fn parse_times_and_distances_simple(input: &str) -> Vec<(u32, u32)> {
    let mut splits = input.split("\r\n");
    let (mut time_str, mut distance_str) = ("", "");
    match (splits.nth(0), splits.nth(0)) {
        (Some(t), Some(d)) => (time_str, distance_str) = (t, d),
        (_, _) => {
            eprintln!("Failed to parse contents")
        },
    };
    let times = time_str.split_ascii_whitespace().skip(1).map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let distances = distance_str.split_ascii_whitespace().skip(1).map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    if times.len() != distances.len() {
        eprintln!("Parsed {} times and {} distances. These must match.", times.len(), distances.len());
    }
    (0..times.len()).map(| i | (times[i], distances[i])).collect::<Vec<(u32, u32)>>()
}

pub fn solve_b(context: &context::Context) -> Option<bool>{
    println!("Solving Day 6b using context:\n{}", context.input);
    let (time, to_beat) = parse_time_and_distance_complex(&context.input);
    let mut min_win_time = 0;
    for hold_duration in 1..time-1 {
        // considered a win if speed(hold_duration) * time - hold_duration > to_beat
        if hold_duration * (time - hold_duration) > to_beat {
            // println!("Time: {} Dist: {} Beat By: {}", time, to_beat, hold_duration);
            min_win_time = hold_duration;
            break;
        }
    }
    // time = max # ways to win
    // min_win_time = # ways to win from 0
    // max_win_time = time - min_win_time (i.e. time = 10, win first time at 3ms. Therefore, highest win is at 7 )
    // total ways = max_win_time - min-win_time = time - 2 * min_win_time
    println!("Time: {}ms, Dist: {}mm Range:{:?} Count:{}", time, to_beat, (min_win_time, time - min_win_time), time - 2 * min_win_time + 1);
    Some(true)
}


fn parse_time_and_distance_complex(input: &str) -> (u64, u64) {
    let mut splits = input.split("\r\n");
    let (mut time_str, mut distance_str) = ("", "");
    match (splits.nth(0), splits.nth(0)) {
        (Some(t), Some(d)) => (time_str, distance_str) = (t, d),
        (_, _) => {
            eprintln!("Failed to parse contents")
        },
    };
    let time = time_str.split_ascii_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap();
    let distance = distance_str.split_ascii_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap();
    (time, distance)
}