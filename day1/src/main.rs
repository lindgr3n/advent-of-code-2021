use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<_>>();
    let mut input = vec![];
    for x in lines {
        input.push(x.parse::<i32>().unwrap());
    }

    let result = depth_increase(input.clone());
    let result_windowed = depth_windowed(input.clone());
    println!("Result Part One: {}", result);
    println!("Result Part Two: {}", result_windowed);
}

fn depth_increase(input: Vec<i32>) -> i32 {
    let mut previous_value: i32 = -1;
    let mut increments: i32 = 0;
    for n in input {
        if n > previous_value && previous_value != -1 {
            increments += 1;
        }
        previous_value = n;
        // println!("{} {} {}", n, previous_value, increments);
    }
    return increments;
}

fn depth_windowed(input: Vec<i32>) -> i32 {
    let mut previous_value: i32 = -1;
    let mut increments: i32 = 0;
    let mut counter = 0;
    for n in &input {
        if counter > 1 {
            let value_one = input[counter - 2];
            let value_two = input[counter - 1];
            let value_three = n;
            let sum = value_one + value_two + value_three;
            if previous_value != -1 && sum > previous_value {
                increments += 1;
                // println!("{} {} {}", previous_value, sum, increments);
            } else {
                // println!("{} {} ", previous_value, sum);
            }
            previous_value = sum;
        }
        counter += 1;
    }
    return increments;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_depth_increase() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(depth_increase(input), 7)
    }
    #[test]
    fn test_depth_windowed_increase() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(depth_windowed(input), 5)
    }
}
