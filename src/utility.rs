pub fn first_unsafe_index(spliterator: &Vec<i64>) -> i64{
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