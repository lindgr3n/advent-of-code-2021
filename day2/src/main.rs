use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<_>>();
    let result_part_one = calculate_position(lines.clone());
    let result_part_two = calculate_position_with_aim(lines.clone());
    println!(
        "Result part 1: {} x {} = {}",
        result_part_one[0],
        result_part_one[1],
        result_part_one[0] * result_part_one[1]
    );
    println!(
        "Result part 2: {} x {} = {}",
        result_part_two[0],
        result_part_two[1],
        result_part_two[0] * result_part_two[1]
    );
}

fn calculate_position(input: Vec<&str>) -> Vec<i32> {
    let mut position = vec![0, 0];
    for x in input {
        let command: Vec<&str> = x.split_whitespace().collect();
        let dir = command[0];
        let distance = command[1];

        if dir.to_uppercase() == "FORWARD" {
            position[0] += distance.parse::<i32>().unwrap();
        }
        if dir.to_uppercase() == "UP" {
            position[1] -= distance.parse::<i32>().unwrap();
        }
        if dir.to_uppercase() == "DOWN" {
            position[1] += distance.parse::<i32>().unwrap();
        }
    }
    return position;
}

fn calculate_position_with_aim(input: Vec<&str>) -> Vec<i32> {
    let mut position = vec![0, 0];
    let mut aim = 0;
    for x in input {
        let command: Vec<&str> = x.split_whitespace().collect();
        let dir = command[0];
        let distance = command[1];

        if dir.to_uppercase() == "FORWARD" {
            position[0] += distance.parse::<i32>().unwrap();
            position[1] += distance.parse::<i32>().unwrap() * aim;
        }
        if dir.to_uppercase() == "UP" {
            aim -= distance.parse::<i32>().unwrap();
        }
        if dir.to_uppercase() == "DOWN" {
            aim += distance.parse::<i32>().unwrap();
        }
    }
    return position;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_depth_increase() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(calculate_position(input), vec![15, 10])
    }
    #[test]
    fn test_depth_increase_with_aim() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(calculate_position_with_aim(input), vec![15, 60])
    }
}
