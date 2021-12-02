use std::fs;

fn main() {
    let instruction_list = fs::read_to_string("../input")
        .expect("Something went wrong reading the file");

    let instructions: Vec<&str> = instruction_list.split("\r\n").collect();
    part_1(&instructions);
    part_2(&instructions);
}

fn part_1(instructions: &Vec<&str>) {
    let mut current_x = 0;
    let mut current_d = 0;
    for instruction in instructions {
        let (direction, value) = parse_instruction(instruction);
        match direction {
            "forward" => current_x += value,
            "down" => current_d += value,
            "up" => current_d -= value,
            _ => println!("Command Not Supported")
        }
    }
    println!("{:?}", current_d * current_x);
}

fn part_2(instructions: &Vec<&str>) {
    let mut current_x = 0;
    let mut current_d = 0;
    let mut current_aim = 0;
    for instruction in instructions {
        let (direction, value) = parse_instruction(instruction);
        match direction {
            "forward" => {
                current_x += value;
                current_d += current_aim * value
            },
            "down" => current_aim += value,
            "up" => current_aim -= value,
            _ => println!("Command Not Supported")
        }
    }
    println!("{:?}", current_d * current_x);
}

fn parse_instruction(instruction: &str) -> (&str, i32) {
    let pi: Vec<&str> = instruction.split(" ").collect();
    return (pi[0], pi[1].parse::<i32>().unwrap());
}