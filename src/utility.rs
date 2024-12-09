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