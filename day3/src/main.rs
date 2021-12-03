use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<_>>();

    let result_part_one = calculate_power_consumtion(lines.clone());
    let result_oxygen = oxygen_generator_rating(lines.clone(), 0);
    let result_co2 = co2_scrubber_rating(lines.clone(), 0);
    let result_part_two = result_oxygen * result_co2;
    println!("Result - part one: {}", result_part_one);
    println!("Result - part two: {}", result_part_two);
}

fn calculate_power_consumtion(input: Vec<&str>) -> i32 {
    let mut gamma: Vec<i32> = [].to_vec();
    let mut epsilon: Vec<i32> = [].to_vec();
    let mut tmp: Vec<i32> = [].to_vec();
    for x in input {
        let mut counter = 0;
        let bits = x.split("");
        for b in bits {
            if b == "" {
                continue;
            }
            if tmp.len() == counter {
                tmp.push(0);
            }
            if b == "1" {
                tmp[counter] = tmp[counter] + 1
            } else {
                tmp[counter] = tmp[counter] - 1
            }
            counter += 1;
        }
    }
    for x in tmp {
        if x > 0 {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }
    let gamma_list: Vec<String> = gamma.iter().map(|n| n.to_string()).collect();
    let epsilon_list: Vec<String> = epsilon.iter().map(|n| n.to_string()).collect();
    let gamma_binary = gamma_list.join("");
    let epsilon_binary = epsilon_list.join("");
    let gamma_int = i32::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon_int = i32::from_str_radix(&epsilon_binary, 2).unwrap();
    return gamma_int * epsilon_int;
}

fn convert_binary_to_int(input: &str) -> i32 {
    let int = i32::from_str_radix(&input, 2).unwrap();
    return int;
}

fn most_common_binary(input: Vec<&str>, bit: usize, priority: usize) -> i32 {
    let mut tmp: Vec<i32> = [].to_vec();
    let mut result: Vec<i32> = [].to_vec();
    for x in input {
        let mut counter = 0;
        let bits = x.split("");
        for b in bits {
            if b == "" {
                continue;
            }
            if tmp.len() == counter {
                tmp.push(0);
            }
            if b == "1" {
                tmp[counter] = tmp[counter] + 1
            } else {
                tmp[counter] = tmp[counter] - 1
            }
            counter += 1;
        }
    }
    for x in tmp {
        if x == 0 && priority == 1 {
            result.push(1)
        } else if x > 0 {
            result.push(1);
        } else {
            result.push(0);
        }
    }
    return result[bit];
}

fn least_common_binary(input: Vec<&str>, bit: usize, priority: usize) -> i32 {
    let mut tmp: Vec<i32> = [].to_vec();
    let mut result: Vec<i32> = [].to_vec();
    for x in input {
        let mut counter = 0;
        let bits = x.split("");
        for b in bits {
            if b == "" {
                continue;
            }
            if tmp.len() == counter {
                tmp.push(0);
            }
            if b == "1" {
                tmp[counter] = tmp[counter] + 1
            } else {
                tmp[counter] = tmp[counter] - 1
            }
            counter += 1;
        }
    }
    for x in tmp {
        if x == 0 && priority == 0 {
            result.push(0)
        } else if x > 0 {
            result.push(0);
        } else {
            result.push(1);
        }
    }
    return result[bit];
}

fn filter_out_binarys(input: Vec<&str>, bit: usize, binary: i32) -> Vec<&str> {
    return input
        .into_iter()
        .filter(|x| x.chars().nth(bit).unwrap().to_string() == binary.to_string())
        .collect();
}

fn oxygen_generator_rating(input: Vec<&str>, bit: usize) -> i32 {
    if input.len() == 1 {
        return convert_binary_to_int(input[0]);
    }
    // Check most of 1/0 in bit one
    let binary = most_common_binary(input.clone(), bit, 1);
    // Filter out the other lines
    let new_input = filter_out_binarys(input.clone(), bit, binary);
    // Run method again until we have one result
    return oxygen_generator_rating(new_input, bit + 1);
}

fn co2_scrubber_rating(input: Vec<&str>, bit: usize) -> i32 {
    if input.len() == 1 {
        return convert_binary_to_int(input[0]);
    }
    // Check most of 1/0 in bit one
    let binary = least_common_binary(input.clone(), bit, 0);
    // Filter out the other lines
    let new_input = filter_out_binarys(input.clone(), bit, binary);
    // Run method again until we have one result
    return co2_scrubber_rating(new_input, bit + 1);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_power_consumtion() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(calculate_power_consumtion(input), 198)
    }
    #[test]
    fn test_convert_binary_to_int() {
        let input = "10111";
        assert_eq!(convert_binary_to_int(input), 23)
    }
    #[test]
    fn test_most_common_binary() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let input_two = vec!["10110", "10111"];
        assert_eq!(most_common_binary(input.clone(), 0, 0), 1);
        assert_eq!(most_common_binary(input.clone(), 1, 0), 0);
        assert_eq!(most_common_binary(input_two.clone(), 4, 0), 0);
        assert_eq!(most_common_binary(input_two.clone(), 4, 1), 1);
    }
    #[test]
    fn test_least_common_binary() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let input_two = vec!["10110", "10111"];
        // let input_three = vec!["01111", "01010"];
        assert_eq!(least_common_binary(input.clone(), 0, 0), 0);
        assert_eq!(least_common_binary(input.clone(), 1, 0), 1);
        assert_eq!(least_common_binary(input_two.clone(), 4, 0), 0);
        assert_eq!(least_common_binary(input_two.clone(), 4, 1), 1);
    }
    #[test]
    fn test_filter_out_binarys() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(
            filter_out_binarys(input.clone(), 0, 0),
            ["00100", "01111", "00111", "00010", "01010",]
        );
        assert_eq!(
            filter_out_binarys(input.clone(), 0, 1),
            ["11110", "10110", "10111", "10101", "11100", "10000", "11001"]
        );
    }
    #[test]
    fn test_oxygen_generator_rating() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(oxygen_generator_rating(input, 0), 23)
    }
    #[test]
    fn test_co2_scrubber_rating() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(co2_scrubber_rating(input, 0), 10)
    }
}
