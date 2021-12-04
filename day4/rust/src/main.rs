use std::fs;
use itertools::Itertools;

fn main() {
    let instructions_s = fs::read_to_string("../input")
        .expect("Something went wrong reading the file");
    let order = get_order(&instructions_s);
    let starting_boards = get_boards(&instructions_s);
    part_1(order.clone(), starting_boards.clone());
    part_2(order.clone(), starting_boards.clone());
}

fn part_1(order: Vec<i32>, starting_boards: Vec<Vec<Vec<i32>>>) {
    let (sum, winning_number) = play_round(true, order, starting_boards);
    println!("Day 4 - Part 1: {:?}", sum * winning_number)
}

fn part_2(order: Vec<i32>, starting_boards: Vec<Vec<Vec<i32>>>) {
    let (sum, winning_number) = play_round(false, order, starting_boards);
    println!("Day 4 - Part 2: {:?}", sum * winning_number)
}

fn play_round(first: bool, order: Vec<i32>, starting_boards: Vec<Vec<Vec<i32>>>) -> (i32, i32) {
    let mut boards = starting_boards;
    let mut found_board = Vec::new();
    let mut winning_number = 0;
    for num in order {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for space in row.iter_mut() {
                    if *space == num {
                        *space = -1;
                    }
                }
            }
        }
        let (winning_boards, remaining_boards): (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<i32>>>) = boards.into_iter().partition(check_if_won);
        if first && winning_boards.len() != 0 {
            found_board = winning_boards[0].clone();
            winning_number = num;
            break;
        } else if !first && remaining_boards.len() == 0 && winning_boards.len() == 1  {
            found_board = winning_boards[0].clone();
            winning_number = num;
            break;
        }

        boards = remaining_boards;
    }
    let sum: i32 = found_board
        .into_iter()
        .flatten()
        .filter(|value| *value != -1)
        .sum();
    return (sum, winning_number);
}

fn check_if_won(board: &Vec<Vec<i32>>) -> bool {
    if board.iter().any(|row| row.iter().all(|space| *space == -1)) {
        return true;
    } else if (0..board[0].len()).any(|col| board.iter().all(|row| row[col] == -1)) {
        return true;
    }
    return false;
}

fn get_order(input: &str) -> Vec<i32> {
    return input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();
}

fn get_boards(input: &str) -> Vec<Vec<Vec<i32>>> {
    return input
        .lines()
        .skip(1)
        .chunks(6)
        .into_iter()
        .map(|board| {
            board
            .skip(1)
            .map(|row| {
                row
                .split_whitespace()
                .map(|space| space.parse::<i32>().unwrap())
                .collect_vec()
            }).collect_vec()
        }).collect_vec();
}