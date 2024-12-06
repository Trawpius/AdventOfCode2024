use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use regex::{Regex, RegexSet};
mod utility;


fn main() {
    #![allow(warnings)]
    /*println!("Day 1 Puzzle 1: {}",day_1_puzzle_1());
    println!("Day 1 Puzzle 2: {}",day_1_puzzle_2());
    println!("Day 2 Puzzle 1: {}",day_2_puzzle_1());
    println!("Day 2 Puzzle 2: {}",day_2_puzzle_2());            // NOT SOLVED
    println!("Day 3 Puzzle 2: {}",day_3_puzzle_1());
    println!("Day 3 Puzzle 2: {}",day_3_puzzle_2());*/
    //println!("Day 4 Puzzle 2: {}",day_4_puzzle_1());
    //println!("Day 4 Puzzle 2: {}",day_4_puzzle_2());
    //println!("Day 5 Puzzle 1: {}",day_5_puzzle_1());
    println!("Day 5 Puzzle 2: {}",day_5_puzzle_2());
}


fn day_1_puzzle_1() -> i64 {
    #![allow(warnings)]
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
    #![allow(warnings)]
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
    #![allow(warnings)]
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
    #![allow(warnings)]
    let mut safe = 0;

    let file = File::open("./inputs/day2_puzzle1.txt").expect("Unable to open file for reading");
    let linerator = BufReader::new(file).lines();

    for line in linerator {
        let unwrapped_line = line.expect("Unable to read line");
        let mut spliterator: Vec<_> = unwrapped_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        let unsafe_index = utility::first_unsafe_index(&spliterator);

        if unsafe_index < 0 {
            safe += 1;
        }
        else {
            spliterator.remove(unsafe_index as usize);
            let unsafe_index = utility::first_unsafe_index(&spliterator);
            if unsafe_index < 0 {
                safe += 1;
            }
        }
    }
    return safe;
}
fn day_3_puzzle_1() -> i64{
    #![allow(warnings)]
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
    #![allow(warnings)]
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
fn day_4_puzzle_1() -> i64{
    #![allow(warnings)]
    let file = File::open("./inputs/day4_puzzle1.txt").expect("Unable to open file for reading");
    
    // 2d vector of characters
    let content = BufReader::new(file).lines().map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let width = content[0].len();
    let height = content.len();
    let pattern_1 = "XMAS".to_string();

    // forward not backward
    // down not up
    // downright not upleft
    // downleft not upright
    let mut sum = 0;
        for y in 0..height{
            for x in 0..width{

                if content[y][x]=='X' || content[y][x] == 'S'{
                    let mut horizontal_string = String::new();
                    if x + 3 < width{
                        horizontal_string.push(content[y][x]);
                        horizontal_string.push(content[y][x+1]);
                        horizontal_string.push(content[y][x+2]);
                        horizontal_string.push(content[y][x+3]);
                        match horizontal_string.as_str() {
                            "XMAS" => sum+=1,
                            "SAMX" => sum+=1,
                            _ => (),
                        };
                    }


                    let mut vertical_string = String::new();
                    if y + 3 < height{
                        vertical_string.push(content[y][x]);
                        vertical_string.push(content[y+1][x]);
                        vertical_string.push(content[y+2][x]);
                        vertical_string.push(content[y+3][x]);
                        match vertical_string.as_str() {
                            "XMAS" => sum+=1,
                            "SAMX" => sum+=1,
                            _ => (),
                        };
                    }


                    let mut diag_string = String::new();
                    if x + 3 < width && y + 3 < height {
                        diag_string.push(content[y][x]);
                        diag_string.push(content[y+1][x+1]);
                        diag_string.push(content[y+2][x+2]);
                        diag_string.push(content[y+3][x+3]);
                        match diag_string.as_str() {
                            "XMAS" => sum+=1,
                            "SAMX" => sum+=1,
                            _ => (),
                        };
                    }

                    let mut diag_string = String::new();
                    if x >= 3  && y + 3 < height{
                        diag_string.push(content[y][x]);
                        diag_string.push(content[y+1][x-1]);
                        diag_string.push(content[y+2][x-2]);
                        diag_string.push(content[y+3][x-3]);
                        match diag_string.as_str() {
                            "XMAS" => sum+=1,
                            "SAMX" => sum+=1,
                            _ => (),
                        };
                    }
                }
            }
        }
    return sum;
}
fn day_4_puzzle_2() -> i64{
    #![allow(warnings)]
    let file = File::open("./inputs/day4_puzzle1.txt").expect("Unable to open file for reading");
    
    // 2d vector of characters
    let content = BufReader::new(file).lines().map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let width = content[0].len();
    let height = content.len();
    
    let regex_uno = Regex::new(r"M.S.A.M.S|S.S.A.M.M|M.M.A.S.S|S.M.A.S.M").unwrap();

    let mut sum = 0;
        for y in 0..height-2{
            for x in 0..width-2{
                // m.s.a.m.s; s.s.a.m.m; m.m.a.s.s;s.m.a.s.m 
                let window: String = [content[x][y], content[x+1][y], content[x+2][y],
                    content[x][y+1], content[x+1][y+1], content[x+2][y+1],
                    content[x][y+2], content[x+1][y+2], content[x+2][y+2]].iter().collect();
                
                sum += regex_uno.find_iter(window.as_str()).count();
            }
        }
    return sum as i64;
}
fn day_5_puzzle_1() -> i64 {
    #![allow(warnings)]
    let file = File::open("./inputs/day5_puzzle1.txt").expect("Unable to open file for reading");
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    
    let  ordering_rules = lines
                            .clone()
                            .into_iter()
                            .filter(|x| x.split('|').count() == 2 );
    let  update_list = lines
                            .clone()
                            .into_iter()
                            .filter(|x| x.split(',').count() > 1 );

    let mut order_tuples:Vec<(i64,i64)> = vec![];
    for order in ordering_rules.clone(){
        let mut splitter = order.split('|');
        let before: i64 = splitter
                            .nth(0)
                            .expect("Could not access splitter index")
                            .parse::<i64>()
                            .expect("Could not parse order item");
        let after: i64 = splitter
                            .nth(0)
                            .expect("Could not access splitter index")
                            .parse::<i64>()
                            .expect("Could not parse order item");
        order_tuples.push((before,after));
    }
    

    let mut sum = 0;
    for update in update_list {
        
        let splitter: Vec<_> = update.split(',').map(|x| x.parse::<i64>()).collect();

        let center = (((splitter.len() as f64) / 2.0) - 1.0).ceil();

        let mut ok = true;
        for window in splitter.windows(2){
            let first = window[0].clone().unwrap();
            let second = window[1].clone().unwrap();
            if order_tuples.iter().filter(|x| x.0 == second && x.1 == first).count() > 0{
                ok = false;
            }
        }
        if ok {
            sum += splitter[center as usize].clone().unwrap();
        }
    }

    return sum;
}
fn day_5_puzzle_2() -> i64 {
    #![allow(warnings)]
    let file = File::open("./inputs/day5_puzzle1.txt").expect("Unable to open file for reading");
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    
    let  ordering_rules = lines
                            .clone()
                            .into_iter()
                            .filter(|x| x.split('|').count() == 2 );
    let  update_list = lines
                            .clone()
                            .into_iter()
                            .filter(|x| x.split(',').count() > 1 );

    let mut order_tuples:Vec<(i64,i64)> = vec![];
    for order in ordering_rules.clone(){
        let mut splitter = order.split('|');
        let before: i64 = splitter
                            .nth(0)
                            .expect("Could not access splitter index")
                            .parse::<i64>()
                            .expect("Could not parse order item");
        let after: i64 = splitter
                            .nth(0)
                            .expect("Could not access splitter index")
                            .parse::<i64>()
                            .expect("Could not parse order item");
        order_tuples.push((before,after));
    }
    

    let mut sum = 0;
    for update in update_list {
        
        let mut splitter: Vec<_> = update.split(',').map(|x| x.parse::<i64>()).collect();

        let center = (((splitter.len() as f64) / 2.0) - 1.0).ceil();

        let mut ok = true;
        for window in splitter.windows(2){
            let first = window[0].clone().unwrap();
            let second = window[1].clone().unwrap();
            if order_tuples.iter().filter(|x| x.0 == second && x.1 == first).count() > 0{
                ok = false;
                break;
            }
        }
        if !ok {
            let mut swaps: bool = true;
            while  swaps {
                swaps = false;
                for i in 0.. splitter.len()-1{
                    let first = splitter[i].clone().unwrap();
                    let second = splitter[i+1].clone().unwrap();
                    if order_tuples.iter().filter(|x| x.0 == second && x.1 == first).count() > 0{
                        let temp = splitter[i].clone();
                        splitter[i] = splitter[i+1].clone();
                        splitter[i+1] = temp;
                        swaps = true;
                    }
                }
            }
            sum += splitter[center as usize].clone().unwrap();
        }
    }

    return sum;
}