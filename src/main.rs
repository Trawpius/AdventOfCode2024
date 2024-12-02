use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
    
    println!("Day 1 Puzzle 1: {}",day_1_puzzle_1());
    println!("Day 1 Puzzle 2: {}",day_1_puzzle_2());
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