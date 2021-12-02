use std::fs;

fn main() {
    let depths_s = fs::read_to_string("../input")
        .expect("Something went wrong reading the file");

    let depths: Vec<i32> = depths_s.split("\r\n").map(|x| x.parse::<i32>().unwrap()).collect();

    part1(&depths);
    part2(&depths);
}

fn part1(depths: &Vec<i32>) {
    let mut prev = std::i32::MAX;
    let mut total = 0;
    for depth in depths {
       if depth > &prev {
           total+=1;
       }
       prev = *depth
    }
    println!("Day 1 - Part 1: {:?}", total)
}

fn part2(depths: &Vec<i32>) {
    let mut prev = depths[0..3].iter().sum::<i32>();
    let mut total = 0;
    for i in 1..depths.iter().len()-2 {
        let cur = depths[i..i+3].iter().sum::<i32>();
        if cur > prev {
            total += 1;
        }
        prev = cur
    }
    println!("Day 1 - Part 2: {:?}", total)
}