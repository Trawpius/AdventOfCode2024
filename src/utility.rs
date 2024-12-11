use std::collections::HashMap;
use itertools::{iproduct, Itertools, structs};

pub fn first_unsafe_index(spliterator: &Vec<i64>) -> i64{
    #![allow(warnings)]
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

// does reference mean board will be affected in
pub fn guard_walking_sim( board: &mut Vec<Vec<char>>, mut position: (i64,i64), mut movement: usize) -> bool {
    let direction: [(i64,i64);4] =  [(-1,0), (0,1), (1,0),(0,-1)];
    
    // game loop
    let mut exit = false;
    let mut looper = false;

    // stores x, y, and direction index
    let mut position_and_direction: Vec<(i64, i64, i64)> = Vec::new();

    while !exit && !looper{
        // new position
        let new_position = (position.0 + direction[movement].0, position.1 + direction[movement].1);
        if new_position.0 < 0 || new_position.0 > (board.len()-1) as i64 || new_position.1 < 0 || new_position.1 > (board[0].len()-1) as i64{
            board[position.0 as usize][position.1 as usize] = 'X';
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

                // store position in history
                let position_and_direction_tuple = (position.0, position.1, movement as i64);
                if position_and_direction.contains(&position_and_direction_tuple){
                    looper = true;
                }
                else{
                    position_and_direction.push(position_and_direction_tuple);
                }
                // update position to new position
                position = new_position;
            }
        }
    }

    return looper;
}

pub fn get_unobstructed_path(board: &mut Vec<Vec<char>>) -> Vec<(i64,i64)>{
    let mut path: Vec<(i64,i64)> = Vec::new();
    for y in 0..board.len(){
        for x in 0..board.len(){
            let pos = (y as i64,x as i64);
            if !path.contains(&pos) && board[y][x]=='X'{
                path.push(pos);
            }
        }
    }
    return path;
}

pub fn permutate(permutation_space: [char;2], output_size: usize) -> Vec<Vec<char>>{
    let mut permutations :Vec<Vec<char>> = Vec::new();
    
    for x in std::iter::repeat(permutation_space).take(output_size).multi_cartesian_product(){
       //println!("{:?}", x);
        permutations.push(x);
    }
    return permutations;
}

pub fn permutate_3(permutation_space: [char;3], output_size: usize) -> Vec<Vec<char>>{
    let mut permutations :Vec<Vec<char>> = Vec::new();
    
    for x in std::iter::repeat(permutation_space).take(output_size).multi_cartesian_product(){
       //println!("{:?}", x);
        permutations.push(x);
    }
    return permutations;
}

pub fn character_position_hashmap( lines: &Vec<Vec<char>>) -> HashMap<char, Vec<(i64,i64)>>{
    let mut char_pos_map: HashMap<char, Vec<(i64,i64)>> = HashMap::new();
    // row - column
    for x in 0..lines.len(){
        for y in 0..(*lines.get(0).unwrap()).len(){
            let item = lines[x][y];
            if item != '.'{
                let mut entry = char_pos_map.entry(item).or_insert(vec![]);
                entry.push((x as i64,y as i64));
            }
        }
    }
    return char_pos_map;
}

pub fn antinodes(board: &Vec<Vec<char>>, positions:Vec<(i64,i64)>, key:char) -> Vec<(i64,i64)>{
    let num_positions = positions.len();
    let mut antinode_positions : Vec<(i64,i64)> = Vec::new();
    for x in 0..(num_positions-1){
        let first = positions.get(x).unwrap();
        
        for y in 1..num_positions{
            if x == y{
                continue;
            }
            let second = positions.get(y).unwrap();
            
            let rise = first.0 - second.0;
            let run = first.1 - second.1;

            let antinode_one = (first.0 + rise, first.1 + run);
            let antinode_two = (second.0 - rise, second.1 - run);

            //println!("{:?},{:?} - ANTINODE1: {:?}, ANTINODE2: {:?}",first,second, antinode_one,antinode_two);

            // perform if let some on antinode one position
            let antinode_position_one_row = (*board).get(antinode_one.0 as usize);
            if let Some(row) = antinode_position_one_row{
                let antinode_position_one_item = (*row).get(antinode_one.1 as usize);
                if let Some(item) = antinode_position_one_item{
                    if *item != key && !antinode_positions.contains(&antinode_one){
                        antinode_positions.push(antinode_one);
                    }
                }
            }

            // perform if let some on antinode one position
            let antinode_position_two_row = (*board).get(antinode_two.0 as usize);
            if let Some(row) = antinode_position_two_row{
                let antinode_position_two_item = (*row).get(antinode_two.1 as usize);
                if let Some(item) = antinode_position_two_item{
                    if *item != key && !antinode_positions.contains(&antinode_two){
                        antinode_positions.push(antinode_two);
                    }
                }
            }
        }
    }
    return antinode_positions;
}

pub fn antinodes_2(board: &Vec<Vec<char>>, positions:Vec<(i64,i64)>, key:char) -> Vec<(i64,i64)>{
    let num_positions = positions.len();
    let mut antinode_positions : Vec<(i64,i64)> = Vec::new();
    for x in 0..(num_positions-1){
        let first = positions.get(x).unwrap();
        
        for y in 1..num_positions{
            if x == y{
                continue;
            }
            let second = positions.get(y).unwrap();
            
            // add antennna location if it isnt already in antinode position
            if !antinode_positions.contains(first) { antinode_positions.push(*first); }
            if !antinode_positions.contains(second) { antinode_positions.push(*second); }

            let rise = first.0 - second.0;
            let run = first.1 - second.1;

            let mut on_the_board = true;
            let mut multiplicator = 1;
            while on_the_board{
                let antinode_one = (first.0 + rise*multiplicator, first.1 + run*multiplicator);

                 // perform if let some on antinode one position
                let antinode_position_one_row = (*board).get(antinode_one.0 as usize);
                if let Some(row) = antinode_position_one_row{
                    let antinode_position_one_item = (*row).get(antinode_one.1 as usize);
                    if let Some(item) = antinode_position_one_item{
                        if *item != key && !antinode_positions.contains(&antinode_one){
                            antinode_positions.push(antinode_one);
                        }
                    }
                    else{
                        on_the_board = false;
                    }
                }
                else{
                    on_the_board = false;
                }
                multiplicator += 1;
            }

            let mut on_the_board = true;
            let mut multiplicator = 1;
            while on_the_board{
                let antinode_two = (second.0 -  rise*multiplicator, second.1 - run*multiplicator);

                 // perform if let some on antinode one position
                let antinode_position_two_row = (*board).get(antinode_two.0 as usize);
                if let Some(row) = antinode_position_two_row{
                    let antinode_position_two_item = (*row).get(antinode_two.1 as usize);
                    if let Some(item) = antinode_position_two_item{
                        if *item != key && !antinode_positions.contains(&antinode_two){
                            antinode_positions.push(antinode_two);
                        }
                    }
                    else{
                        on_the_board = false;
                    }
                }
                else{
                    on_the_board = false;
                }
                multiplicator += 1;
            }
        }
    }
    return antinode_positions;
}


pub fn trail_head_positions(lines:&Vec<Vec<char>>) -> Vec<(i64,i64)> {
    let mut positions = vec![];
    for (row,row_item) in lines.iter().enumerate(){
        for (column, item) in row_item.iter().enumerate(){
            if *item == '0'{
                positions.push((row as i64,column as i64));
            }
        }
    }
    return positions;
}
pub fn count_trails(position:(i64,i64), place_marker: usize, board:&Vec<Vec<char>>, ) -> Vec<(i64, i64)>{ 
    // return if trail end
    if place_marker == 9{
        return vec![position];
    }

    let directions = vec![(0,1),(0,-1),(1,0),(-1,0)];
    let valid_next_marker = place_marker + 1;

    let mut trails = vec![];
    for d in directions{
        let new_position = ((position.0 as i64)+d.0, (position.1 as i64)+d.1);
        // is row index valid
        if let Some(row) = board.get(new_position.0 as usize){
            // is colum index valid
            if let Some(column) = row.get(new_position.1 as usize){
                
                // convert char to digit
                if let Some(digit) = column.to_digit(10) {
                    if digit == valid_next_marker as u32{
                        let mut rtn = count_trails(new_position, digit as usize, &board);
                        trails.append(&mut rtn);
                    }
                }
            }
        }
    }
    return trails;
}
