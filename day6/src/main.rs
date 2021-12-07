use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result_part_one = simulate_lanternfish(&contents, 80);
    let result_part_two = simulate_undying_lanternfish(&contents, 256); // Solution in part was do not work. Needed to rethink and look up some solutions.
    println!("Result part one: {}", result_part_one);
    println!("Result part two: {}", result_part_two);
}

fn simulate_lanternfish(input: &str, days: i32) -> usize {
    let mut numbers: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut counter = 0;
    while counter < days {
        for index in 0..numbers.len() {
            if numbers[index] == 0 {
                numbers[index] = 6;
                numbers.push(8);
            } else {
                numbers[index] = numbers[index] - 1;
            }
        }
        counter += 1;
    }
    return numbers.len();
}

fn simulate_undying_lanternfish(input: &str, days: i32) -> usize {
    let numbers: Vec<usize> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let mut length = [0; 9];

    for x in &numbers {
        length[*x] += 1;
    }
    for _ in 0..days {
        length.rotate_left(1);
        length[6] += length[8];
    }

    return length.iter().sum();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simulate_lanternfish() {
        assert_eq!(simulate_lanternfish("3,4,3,1,2", 18), 26)
    }
    #[test]
    fn test_simulate_undying_lanternfish() {
        assert_eq!(simulate_undying_lanternfish("3,4,3,1,2", 256), 26984457539)
    }
}
