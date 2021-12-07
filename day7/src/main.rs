use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result_part_one = calculate_fule_consumtion(&contents);
    let result_part_two = calculate_exponential_fule_consumtion(&contents);
    println!("Result part one: {}", result_part_one);
    println!("Result part two: {}", result_part_two);
}

fn calculate_fule_consumtion(input: &str) -> i32 {
    let numbers: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut min_consumtion = -1;
    let mut target = 0;
    let mut counter = 0;
    while counter < 1500 {
        // Move all to 0, 1, 2 check consumtion
        let mut sum = 0;
        for x in &numbers {
            sum += i32::abs(x - target);
        }

        if sum < min_consumtion || min_consumtion == -1 {
            min_consumtion = sum;
        }
        target += 1;
        counter += 1;
    }

    return min_consumtion;
}

fn calculate_exponential_fule_consumtion(input: &str) -> i32 {
    let numbers: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut min_consumtion = -1;
    let mut target = 0;
    let mut counter = 0;
    while counter < 1500 {
        // Move all to 0, 1, 2 check consumtion
        let mut sum = 0;
        for x in &numbers {
            let steps = i32::abs(x - target);
            let mut distance = 0;
            for n in 1..steps + 1 {
                distance += n;
            }
            sum += distance;
        }
        if sum < min_consumtion || min_consumtion == -1 {
            min_consumtion = sum;
        }
        target += 1;
        counter += 1;
    }

    return min_consumtion;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_fule_consumtion() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(calculate_fule_consumtion(input), 37);
    }
    #[test]
    fn test_calculate_exponential_fule_consumtion() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(calculate_exponential_fule_consumtion(input), 168);
    }
}
