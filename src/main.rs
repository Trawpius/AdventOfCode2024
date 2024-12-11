use std::fs::File;
use std::io::{prelude::*, BufReader, Write};
use std::collections::HashMap;
use itertools::Itertools;
use regex::{Regex};
use utility::guard_walking_sim;
use std::time::Instant;


mod utility;



fn main() {
    #![allow(warnings)]
    let mut start = Instant::now();
    /*println!("Day 1 Puzzle 1: {},{:?}",day_1_puzzle_1(), start.elapsed()); start = Instant::now();
    println!("Day 1 Puzzle 2: {},{:?}",day_1_puzzle_2(), start.elapsed()); start = Instant::now();
    println!("Day 2 Puzzle 1: {},{:?}",day_2_puzzle_1(), start.elapsed()); start = Instant::now();
    println!("Day 2 Puzzle 2: {},{:?}",day_2_puzzle_2(), start.elapsed()); start = Instant::now();           // NOT SOLVED
    println!("Day 3 Puzzle 2: {},{:?}",day_3_puzzle_1(), start.elapsed()); start = Instant::now();
    println!("Day 3 Puzzle 2: {},{:?}",day_3_puzzle_2(), start.elapsed()); start = Instant::now();
    println!("Day 4 Puzzle 2: {},{:?}",day_4_puzzle_1(), start.elapsed()); start = Instant::now();
    println!("Day 4 Puzzle 2: {},{:?}",day_4_puzzle_2(), start.elapsed()); start = Instant::now();
    println!("Day 5 Puzzle 1: {},{:?}",day_5_puzzle_1(), start.elapsed()); start = Instant::now();
    println!("Day 5 Puzzle 2: {},{:?}",day_5_puzzle_2(), start.elapsed()); start = Instant::now();
    println!("Day 6 Puzzle 1: {},{:?}",day_6_puzzle_1(), start.elapsed()); start = Instant::now();
    println!("Day 6 Puzzle 2: {},{:?}",day_6_puzzle_2(), start.elapsed()); start = Instant::now();*/
    //println!("Day 7 Puzzle 1: {},{:?}",day_7_puzzle_1(), start.elapsed()); start = Instant::now();
    //println!("Day 7 Puzzle 2: {},{:?}",day_7_puzzle_2(), start.elapsed()); start = Instant::now();
    //println!("Day 8 Puzzle 1: {},{:?}",day_8_puzzle_1(), start.elapsed()); start = Instant::now();
    //println!("Day 8 Puzzle 2: {},{:?}",day_8_puzzle_2(), start.elapsed()); start = Instant::now();
    println!("Day 9 Puzzle 1: {},{:?}",day_9_puzzle_1(), start.elapsed()); start = Instant::now();
    //println!("Day 9 Puzzle 2: {},{:?}",day_9_puzzle_2(), start.elapsed()); start = Instant::now();
    //println!("Day 10 Puzzle 1: {},{:?}",day_10_puzzle_1().0, start.elapsed()); start = Instant::now();
    //println!("Day 10 Puzzle 2: {},{:?}",day_10_puzzle_1().1, start.elapsed()); start = Instant::now();

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
fn day_6_puzzle_1() -> i64{
    #![allow(warnings)]
    let file = File::open("./inputs/day6_puzzle1.txt").expect("Unable to open file for reading");
    let lines = BufReader::new(file).lines();
    
    let mut board: Vec<Vec<char>> = Vec::new(); 

    
    let characters_symbol: [char; 4] = ['^', '>', 'v', '<'];
    let direction: [(i64,i64);4] =  [(-1,0), (0,1), (1,0),(0,-1)];
    // 0 = up; 1 = right; 2 = down; 3 = left
    let mut movement = 0;
    let mut position = (0 as i64,0 as i64);
    
    

    // create gameboard and initial position
    for (y,row) in lines.enumerate(){
        let mut item: Vec<char> = Vec::new();
        
        for (x, col) in row.unwrap().chars().enumerate(){
            item.push(col);
            
            if characters_symbol.contains(&col){
                position = (y as i64,x as i64);
                println!("Original: {:?}",position);
                movement = characters_symbol.iter().position(|sym| *sym == col).unwrap();
            }
        }
        board.push(item);
    }

    let mut exit = false;
    while !exit{
        // new position
        let new_position = (position.0 + direction[movement].0, position.1 + direction[movement].1);
        if new_position.0 < 0 || new_position.0 > (board.len()-1) as i64 || new_position.1 < 0 || new_position.1 > (board[0].len()-1) as i64{
            // is this x-y
            board[position.0 as usize][position.1 as usize] = 'X';
            /*for row in board.clone(){
                println!("{:?}", row);
            }*/
            exit = true;
        }
        else{
            let item = board[new_position.0 as usize][new_position.1 as usize];
            if item =='#'{
                movement = (movement + 1) % 4;
            }
            else {
                // set position to X
                board[position.0 as usize][position.1 as usize] = 'X';
                // update position to new position
                position = new_position;
            }
        }
    }
    return board.iter() .flat_map(|b| b.iter()) .filter(|&&x| x == 'X') .count() as i64;
}
fn day_6_puzzle_2() -> i64{
    #![allow(warnings)]
    let file = File::open("./inputs/day6_puzzle1.txt").expect("Unable to open file for reading");
    let lines = BufReader::new(file).lines();
    
    let mut board: Vec<Vec<char>> = Vec::new(); 

    
    let characters_symbol: [char; 4] = ['^', '>', 'v', '<'];
    let direction: [(i64,i64);4] =  [(-1,0), (0,1), (1,0),(0,-1)];
    // 0 = up; 1 = right; 2 = down; 3 = left
    let mut movement = 0;
    let mut position = (0 as i64,0 as i64);

    // create gameboard and initial position
    for (y,row) in lines.enumerate(){
        let mut item: Vec<char> = Vec::new();
        for (x, col) in row.unwrap().chars().enumerate(){
            item.push(col);
            
            if characters_symbol.contains(&col){
                position = (y as i64,x as i64);
                movement = characters_symbol.iter().position(|sym| *sym == col).unwrap();
            }
        }
        board.push(item);
    }
    let original_board = board.clone();
    let original_position = position.clone();
    let original_movement = movement;

    utility::guard_walking_sim(&mut board, position, movement);
    let unobstructed_path = utility::get_unobstructed_path(&mut board);

    println!("Total {}",unobstructed_path.clone().len());
    let mut valid_obstacles = 0;

    for obstacle in unobstructed_path{
        // reset gameboard
        board = original_board.clone();
        position = original_position.clone();
        movement = original_movement;
        
        // check if an obstacle can be placed
        if board[obstacle.0 as usize][obstacle.1 as usize] != '.'{
            continue;
        }
        else {
            board[obstacle.0 as usize][obstacle.1 as usize] = '#';
        }

        let looper = utility::guard_walking_sim(&mut board, position, movement);
        if looper{
            valid_obstacles += 1;
        }
    }
    return valid_obstacles;
    //return board.iter() .flat_map(|b| b.iter()) .filter(|&&x| x == 'X') .count() as i64;
}
fn day_7_puzzle_1() -> i64{
    #![allow(warnings)]
    let file = File::open("./inputs/day7_puzzle1.txt").expect("Unable to open file for reading");
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap().replace(":", "")).collect();

    let mut sum = 0;
    let permutation_space = ['*','+'];

    for line in lines.iter(){
        let splits: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        let resultant = *splits.get(0).unwrap();
        let number_of_operators = splits.len() - 2;

        let permutations = utility::permutate(permutation_space, number_of_operators);
        
        let mut success = false;
        for operator_set in permutations{

            let mut calculated_resultant = 0;
            for y in 0..operator_set.len(){
                let operator = *operator_set.get(y).unwrap();
                if y == 0{
                    calculated_resultant= *splits.get(y+1).unwrap();
                }
                let right = *splits.get(y+2).unwrap();
                if operator == '+'{
                    calculated_resultant = calculated_resultant + right;
                }
                else if operator == '*'{
                    calculated_resultant = calculated_resultant * right;
                }
            }

            if calculated_resultant == resultant{
                success = true;
            }
        }

        if success{
            sum+= resultant;
        }
    }   

    return sum;
}
fn day_7_puzzle_2() -> i64{
    #![allow(warnings)]
    let file = File::open("./inputs/day7_puzzle1.txt").expect("Unable to open file for reading");
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap().replace(":", "")).collect();

    let mut sum = 0;
    let permutation_space = ['*','+','|'];

    for line in lines.iter(){
        let splits: Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

        let resultant = *splits.get(0).unwrap();
        let number_of_operators = splits.len() - 2;

        let permutations = utility::permutate_3(permutation_space, number_of_operators);
        
        let mut success = false;
        for operator_set in permutations{

            let mut calculated_resultant = 0;
            for y in 0..operator_set.len(){
                let operator = *operator_set.get(y).unwrap();
                if y == 0{
                    calculated_resultant= *splits.get(y+1).unwrap();
                }
                let right = *splits.get(y+2).unwrap();
                if operator == '+'{
                    calculated_resultant = calculated_resultant + right;
                }
                else if operator == '*'{
                    calculated_resultant = calculated_resultant * right;
                }
                else if operator == '|'{
                    
                    calculated_resultant = format!("{}{}",calculated_resultant, right).parse::<i64>().unwrap();
                }
            }

            if calculated_resultant == resultant{
                success = true;
            }
        }

        if success{
            sum+= resultant;
        }
    }   

    return sum;
}
fn day_8_puzzle_1() -> i64 {
    #![allow(warnings)]
    let file = File::open("./inputs/day8_puzzle1.txt").expect("Unable to open file for reading");
    let lines: Vec<Vec<char>> = BufReader::new(file).lines().map(|x| x.unwrap().chars().collect()).collect();
    
    let num_rows = lines.len();
    let num_cols = (*lines.get(0).unwrap()).len();
    
    let character_position = utility::character_position_hashmap(&lines);
    
    let mut antinodes = vec![];
    
    for key in character_position.keys(){
        let positions : Vec<(i64,i64)> = character_position[key].clone();
        let temp =  utility::antinodes(&lines, positions, *key);
        for x in temp.iter(){
            if !antinodes.contains(x){
                antinodes.push(*x);
            }
        }
    }

    return antinodes.len() as i64;
}
fn day_8_puzzle_2() -> i64 {
    #![allow(warnings)]
    let file = File::open("./inputs/day8_puzzle1.txt").expect("Unable to open file for reading");
    let lines: Vec<Vec<char>> = BufReader::new(file).lines().map(|x| x.unwrap().chars().collect()).collect();
    
    let num_rows = lines.len();
    let num_cols = (*lines.get(0).unwrap()).len();
    
    let character_position = utility::character_position_hashmap(&lines);
    
    let mut antinodes = vec![];
    
    for key in character_position.keys(){
        let positions : Vec<(i64,i64)> = character_position[key].clone();
        let temp =  utility::antinodes_2(&lines, positions, *key);
        for x in temp.iter(){
            if !antinodes.contains(x){
                antinodes.push(*x);
            }
        }
    }

    return antinodes.len() as i64;
}
fn day_9_puzzle_1() -> i64 {
    #![allow(warnings)]
    let file = File::open("./inputs/day9_puzzle1_short.txt").expect("Unable to open file for reading");
    let mut contents = String::new();
    let lines = BufReader::new(file).read_to_string(&mut contents);
    
    let nums: Vec<u32> = contents.chars().map(|x| x.to_digit(10).unwrap()).collect();
    
    let mut space: Vec<Vec<String>>= nums.iter().enumerate().filter(|(index,_)| index%2==1).map(|(_,x)| vec![".".to_string(); (*x as usize)]).collect();
    let mut blocks: Vec<Vec<String>>= nums.iter().enumerate().filter(|(index,_)| index%2==0).map(|(item,x)| vec![(item/2).to_string(); (*x as usize)]).collect();

    /*let mut file = File::create("./output_space.txt").unwrap();
    for item in &space { writeln!(file, "{:?}", item);} 
    let mut file = File::create("./output_block.txt").unwrap();
    for item in &blocks { writeln!(file, "{:?}", item);} */

    // point to the current last block
    let mut end_pointer = blocks.len() - 1;
    let mut end_block = &mut blocks[end_pointer];
    
    // enumerate through all empty space blocks
    let mut index = 0;
    let mut max_index = space.len();
    
    while index < max_index{
        let empty = &mut space[index];
        
        for (_ , inner_empty ) in empty.iter_mut().enumerate(){
            // wait i dont think this can happen. enter for loop if iterator is empty
            if inner_empty.len() == 0{
                continue;
            }
            
            // if last block still contains elements
            // pop item off last block, and place in empty space
            if end_block.len() > 0{
                *inner_empty = end_block.pop().clone().unwrap();
            }
            else {
                // search for next block, which contains more than 0 elements
                while end_block.len() == 0{
                    
                    // change end block pointers
                    end_pointer -= 1;

                    // don't fill trailing empty space
                    max_index -= 1;
                    end_block = &mut blocks[end_pointer];
                }
                // pop item off new block, and place in empty space
                *inner_empty = end_block.pop().clone().unwrap();
            }
        }
        // go to next empty block
        index += 1;
    }
    // count = index
    let mut count = 0;    
    let mut defragged = Vec::new();

    // interlace both lists
    for x in blocks.iter().zip(space.iter_mut()){
        let mut xlist:Vec<_> = x.0.iter().map(|x| x.to_string()).collect();
        let mut ylist:Vec<_> = x.1.iter().map(|x| x.to_string()).collect();
        defragged.append(&mut xlist);
        defragged.append(&mut ylist);        
    }
    /*let mut file = File::create("./output_defragged.txt").unwrap();
    for item in &defragged { writeln!(file, "{:?}", item);} */
    //println!("{:?}",defragged);
    let mut checksum = 0;
    for (index, item) in defragged.iter().enumerate(){
        // parse string to i64. add to checksum
        if let Ok(integer) = item.parse::<i64>(){
            checksum += (index as i64) * integer;
        }
    }
    return checksum;
}

// also day10 puzzle 2
fn day_10_puzzle_1() -> (i64,i64) {
    #![allow(warnings)]
    let file = File::open("./inputs/day10_puzzle1.txt").expect("Unable to open file for reading");
    let lines: Vec<Vec<char>> = BufReader::new(file).lines().map(|x| x.unwrap().chars().collect::<Vec<char>>()).collect();
    
    let train_head_positions = utility::trail_head_positions(&lines);
    
    let mut sum_trailheads = 0;
    let mut sum_trailrating = 0;
    for trail_head in train_head_positions{
        let trails = utility::count_trails(trail_head,0, &lines);
        sum_trailheads += trails.iter().unique().count();
        sum_trailrating += trails.len();
    }

    return (sum_trailheads as i64, sum_trailrating as i64);
}

