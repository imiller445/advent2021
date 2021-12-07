use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("../input")
        .expect("Something went wrong reading the file");
    let starting_positions = get_starting_positions(&input);
    part_1(starting_positions.clone());
    part_2(starting_positions.clone());
}

fn part_1(starting_positions: Vec<i32>) {
    let mode = mode(&starting_positions);
    let fuel: i32 = starting_positions
        .iter()
        .map(|pos| (pos - mode).abs())
        .sum();
    println!("Day 7 - Part 1: {:?}", fuel);
}

fn part_2(starting_positions: Vec<i32>) {
    let mean = starting_positions.iter().sum::<i32>() as f64/ starting_positions.len() as f64;
    let min_mean = mean.floor() as i32;
    let max_mean = mean.ceil() as i32;
    let fuel: i32 = (starting_positions
        .iter()
        .map(|pos| {
            let distance = (pos - min_mean).abs();
            distance * (distance + 1) /2
        })
        .sum::<i32>()
    ).min(
        starting_positions
        .iter()
        .map(|pos| {
            let distance = (pos - max_mean).abs();
            distance * (distance + 1) /2
        })
        .sum::<i32>()
    );
    
    println!("Day 7 - Part 2: {:?}", fuel);
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut nums = numbers.clone();
    nums.sort();
    if (nums.len() % 2) as i32 == 0 {
        let left = nums.len() / 2 -1 ;
        let right = nums.len() / 2;
        (nums[left] + nums[right]) / 2
    } else {
        nums[nums.len() / 2]
    }
}

fn get_starting_positions(input: &str) -> Vec<i32> {
    return input
        .split(",")
        .map(|pos| pos.parse::<i32>().unwrap())
        .collect_vec();
}