use std::fs;
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let instructions_s = fs::read_to_string("../input")
        .expect("Something went wrong reading the file");
    let map = get_map(&instructions_s);
    part_1(map.clone());
    part_2(map.clone());
}

fn part_1(map: Vec<Vec<Vec<i32>>>) {
    let overlaps = find_overlapping_vents(false, map);
    println!("Day 5 - Part 1: {:?}", overlaps);
}

fn part_2(map: Vec<Vec<Vec<i32>>>) {
    let overlaps = find_overlapping_vents(true, map);
    println!("Day 5 - Part 2: {:?}", overlaps);
}

fn find_overlapping_vents(diagonals: bool, map: Vec<Vec<Vec<i32>>>) -> u32 {
    let mut vents = HashMap::new();
    for line in map {
        let x_low = cmp::min(line[0][0], line[1][0]);
        let x_high = cmp::max(line[0][0], line[1][0])+1;
        let y_low = cmp::min(line[0][1], line[1][1]);
        let y_high = cmp::max(line[0][1], line[1][1])+1;
        if line[0][0] == line[1][0] {
            for y in y_low..y_high {
                let coord = (line[0][0], y);
                *vents.entry(coord).or_insert(0) += 1;
            }
        } else if line[0][1] == line[1][1] {
            for x in x_low..x_high {
                let coord = (x, line[0][1]);
                *vents.entry(coord).or_insert(0) += 1;
            }
        } else if diagonals {
            if (line[0][0] < line[1][0] && line[0][1] < line [1][1]) 
                || (line[0][0] > line[1][0] && line[0][1] > line [1][1]) {
                for (i, x) in (x_low..x_high).enumerate() {
                    let coord = (x, y_low + (i as i32));
                    *vents.entry(coord).or_insert(0) += 1;
                }
            } else {
                for (i, x) in (x_low..x_high).enumerate() {
                    let coord = (x, y_high - 1 - (i as i32));
                    *vents.entry(coord).or_insert(0) += 1;
                }
            }
        }
    }
    let overlaps = vents
        .values()
        .filter(|v| **v > 1)
        .count();
    return overlaps.try_into().unwrap();
}

fn get_map(input: &str) -> Vec<Vec<Vec<i32>>> {
    return input
        .lines()
        .into_iter()
        .map(|l| {
            l
            .split(" -> ")
            .map(|coord| {
                coord.split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect_vec()
            }).collect_vec()
        }).collect_vec();
}