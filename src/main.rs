use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use regex::{Regex, RegexSet};

fn main() {
    /*println!("Day 1 Puzzle 1: {}",day_1_puzzle_1());
    println!("Day 1 Puzzle 2: {}",day_1_puzzle_2());
    println!("Day 1 Puzzle 1: {}",day_2_puzzle_1());
    println!("Day 1 Puzzle 2: {}",day_2_puzzle_2());    // NOT SOLVED
    println!("Day 1 Puzzle 2: {}",day_3_puzzle_1());*/
    println!("Day 1 Puzzle 2: {}",day_3_puzzle_2());
}


fn day_1_puzzle_1() -> i64 {

    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: Vec<i64> = Vec::new();

    let file = File::open("./inputs/day1_puzzle1.txt").expect("Unable to open file for reading");
    let linerator = BufReader::new(file).lines();

    for line in linerator {
        let unwrapped_line = line.expect("Unable to read line");
        let spliterator = unwrapped_line.split_whitespace();

        // check validity of line input
        let count = spliterator.clone().count();
        if count != 2{
            panic!("Incorrect input format");
        }

        let first_item = spliterator.clone().nth(0).expect("Unable to access the first item in split iterator").parse::<i64>().expect("Unable to parse text into i64");
        let second_item = spliterator.clone().nth(1).expect("Unable to access the second item in split iterator").parse::<i64>().expect("Unable to parse text into i64");

        first_list.push(first_item);
        second_list.push(second_item);

    }

    first_list.sort();
    second_list.sort();

    if first_list.len() != second_list.len() {
        panic!("Vector lengths are not equal");
    }

    let mut sum = 0;
    for (a,b) in first_list.iter().zip(second_list.iter()){
        sum += (a-b).abs();
    }

    return sum;

}

fn day_1_puzzle_2() -> i64 {

    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: HashMap<i64, i64> = HashMap::new();

    let file = File::open("./inputs/day1_puzzle1.txt").expect("Unable to open file for reading");
    let linerator = BufReader::new(file).lines();

    for line in linerator {
        let unwrapped_line = line.expect("Unable to read line");
        let spliterator = unwrapped_line.split_whitespace();

        // check validity of line input
        let count = spliterator.clone().count();
        if count != 2{
            panic!("Incorrect input format");
        }

        let first_item = spliterator.clone().nth(0).expect("Unable to access the first item in split iterator").parse::<i64>().expect("Unable to parse text into i64");
        let second_item = spliterator.clone().nth(1).expect("Unable to access the second item in split iterator").parse::<i64>().expect("Unable to parse text into i64");

        // list
        first_list.push(first_item);

        // frequency hashmap
        *second_list.entry(second_item).or_insert(0) += 1;
    }

    let mut sum = 0;
    for a in first_list.iter(){
       if second_list.contains_key(a){
            let temp = *second_list.entry(*a).or_insert(0);
            sum += temp*a;
        }
    }

    return sum;
}

fn day_2_puzzle_1() -> i64 {
    let mut safe = 0;
    let max = 3;

    let file = File::open("./inputs/day2_puzzle1.txt").expect("Unable to open file for reading");
    let linerator = BufReader::new(file).lines();

    for line in linerator {
        let unwrapped_line = line.expect("Unable to read line");
        let spliterator: Vec<_> = unwrapped_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let differences: Vec<_> = spliterator.windows(2).map(|window| window[1] - window[0]).collect();

        let count_greater_than_3 = differences.clone().iter().fold(0, |acc, num: &i64| if (*num).abs() > max {acc + 1} else {acc});
        let count_negative = differences.clone().iter().fold(0, |acc, num: &i64| if *num <= 0 {acc + 1} else {acc});
        let count_positive = differences.clone().iter().fold(0, |acc, num: &i64| if *num >= 0 {acc + 1} else {acc});

        if count_greater_than_3 == 0 && ((count_negative > 0) ^ (count_positive > 0)) {
            safe += 1;
        }
    }
    return safe;
}
fn day_2_puzzle_2() -> i64 {
    let mut safe = 0;

    let file = File::open("./inputs/day2_puzzle1.txt").expect("Unable to open file for reading");
    let linerator = BufReader::new(file).lines();

    for line in linerator {
        let unwrapped_line = line.expect("Unable to read line");
        let mut spliterator: Vec<_> = unwrapped_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        let unsafe_index = first_unsafe_index(&spliterator);

        if unsafe_index < 0 {
            safe += 1;
        }
        else {
            spliterator.remove(unsafe_index as usize);
            let unsafe_index = first_unsafe_index(&spliterator);
            if unsafe_index < 0 {
                safe += 1;
            }
        }
    }
    return safe;
}
fn first_unsafe_index(spliterator: &Vec<i64>) -> i64{
    let length = (spliterator.len() - 1);
    let mut direction = 0;

    // special directionality check? Presumes min four entries I guess
    let first_delta = spliterator[0+1] - spliterator[0];
    let second_delta = spliterator[0+2] - spliterator[0+1];
    let third_delta = spliterator[0+3] - spliterator[0+2];
    if first_delta > 0{
        if second_delta < 0 && third_delta < 0{
            return 0;
        }
    }

    if first_delta < 0{
        if second_delta > 0 && third_delta > 0{
            return 0;
        }
    }

    for i in 0..length {
        let first = spliterator[i];
        let second = spliterator[i+1];
        let delta = second-first;
        if delta.abs() > 3{
            return i as i64;
        }
        if delta == 0{
            return i as i64;
        }
        if direction == 0{
            direction = if delta > 0 { 1 } else { -1}; 
        }
        if direction == 1 && delta < 0{
            return i as i64;
        }
        if direction == -1 && delta > 0{
            return i as i64;
        }
    }
    return -1;
}

fn day_3_puzzle_1() -> i64{
    let file = File::open("./inputs/day3_puzzle1.txt").expect("Unable to open file for reading");
    let regex_uno = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let content = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>().concat();


    let mut sum_of_products = 0;
    for matching in regex_uno.find_iter(&content){
        let regex_dos = Regex::new(r"\d+").unwrap();
        let multiplicators = regex_dos.find_iter(&matching.as_str()).map(|x| x.as_str().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let len = multiplicators.len();
        if len != 2{
            continue;
        }
        let product = multiplicators[0] * multiplicators[1];
        sum_of_products += product;
    }
    return sum_of_products;
}

fn day_3_puzzle_2() -> i64{
    let file = File::open("./inputs/day3_puzzle1.txt").expect("Unable to open file for reading");
    let regex_uno = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    let content = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>().concat();


    let mut sum_of_products = 0;
    let mut on = true;
    for matching in regex_uno.find_iter(&content){
        match matching.as_str() {
            "do()" => on = true,
            "don't()" => on = false,
            _ => {
                let regex_dos = Regex::new(r"\d+").unwrap();
                let multiplicators = regex_dos.find_iter(&matching.as_str()).map(|x| x.as_str().parse::<i64>().unwrap()).collect::<Vec<i64>>();
                let len = multiplicators.len();
                if len != 2{
                    continue;
                }
                if on{
                let product = multiplicators[0] * multiplicators[1];
                sum_of_products += product;
                }
            },
        }
    }
    return sum_of_products;
}