use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("../input")
        .expect("Something went wrong reading the file");
    let starting_ages = get_starting_fish_ages(&input);
    println!("{:?}", starting_ages);
    part_1(starting_ages.clone());
    part_2(starting_ages.clone());
}

fn part_1(starting_ages: Vec<u64>) {
    let num_of_fish =  simulate_days(starting_ages, 80);
    println!("Day 6 - Part 1: {:?}", num_of_fish);
}

fn part_2(starting_ages: Vec<u64>) {
    let num_of_fish =  simulate_days(starting_ages, 256);
    println!("Day 6 - Part 2: {:?}", num_of_fish);
}

fn simulate_days(starting_ages: Vec<u64>, num_of_days: i32) -> u64 { 
    let mut ages = starting_ages.clone();
    let mut day = 0;
    while day < num_of_days {
        ages.rotate_left(1);
        ages[6] += ages[8];
        day += 1;
    }
    return ages.iter().sum();
}

fn get_starting_fish_ages(input: &str) -> Vec<u64> {
    return input
        .split(",")
        .map(|age| age.parse::<u64>().unwrap())
        .collect_vec()
        .iter()
        .fold(vec![0;9], |mut v, x| {
            v[(*x as usize)] += 1;
            v
        });
}