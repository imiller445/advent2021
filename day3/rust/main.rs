use std::fs;

fn main() {
    let binaries_s = fs::read_to_string("../input")
        .expect("Something went wrong reading the file");

    let binaries: Vec<&str> = binaries_s.split("\r\n").collect();
    part_1(&binaries);
    part_2(&binaries)
}

fn part_1(binaries: &Vec<&str>) {
    let (most_common, least_common) = calculate_most_and_least_frequent(binaries);
    let gamma = isize::from_str_radix(most_common.into_iter().collect::<String>().as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(least_common.into_iter().collect::<String>().as_str(), 2).unwrap();
    println!("Day 3 - Part 1: {:?}", gamma * epsilon);
}

fn part_2(binaries: &Vec<&str>) {
    let (mut most_common, _) = calculate_most_and_least_frequent(binaries);
    let mut ogr_found = false;
    let mut co2s_found = false;
    let mut idx = 0;
    let mut ogr_binaries = binaries.clone();
    let mut co2s_binaries = binaries.clone();
    while !ogr_found {
        ogr_binaries.retain(|x| x.chars().nth(idx).unwrap() == most_common[idx]);
        if ogr_binaries.iter().len() == 1 {
            ogr_found = true;
        } else {
            idx +=1;
            let (most_common_t, _) = calculate_most_and_least_frequent(&ogr_binaries);
            most_common = most_common_t;
        }
    }
    idx = 0;
    let (_, mut least_common) = calculate_most_and_least_frequent(binaries);
    while !co2s_found {
        co2s_binaries.retain(|x| x.chars().nth(idx).unwrap() == least_common[idx]);
        if co2s_binaries.iter().len() == 1 {
            co2s_found = true;
        } else {
            idx +=1;
            let (_, least_common_t) = calculate_most_and_least_frequent(&co2s_binaries);
            least_common = least_common_t;
        }
    }
    let ogr = isize::from_str_radix(ogr_binaries[0], 2).unwrap();
    let co2s = isize::from_str_radix(co2s_binaries[0], 2).unwrap();
    println!("Day 3 - Part 2: {:?}", ogr * co2s)
}

fn calculate_most_and_least_frequent(binaries: &Vec<&str>) -> (Vec<char>, Vec<char>) {
    let bin_size = binaries[0].chars().count();
    let mut most_common = vec!['0'; bin_size]; 
    let mut least_common = vec!['0'; bin_size]; 
    let mut holder = vec![0; bin_size]; 
    for binary in binaries {
        let digits: Vec<_> = binary.chars().collect();
        for (i, x) in digits.iter().enumerate() {
            if *x == '0' {
                holder[i] = holder[i]-1;
            } else {
                holder[i] = holder[i]+1;
            }
        }
    }
    for (i, x) in holder.iter().enumerate() {
        if *x >= 0 {
            most_common[i] = '1';
            least_common[i] = '0';
        } else {
            most_common[i] = '0';
            least_common[i] = '1';
        }
    }
    return (most_common, least_common);
}