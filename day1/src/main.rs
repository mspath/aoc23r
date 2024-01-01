fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("sum of calibration values: {}", result_breakfast);
}

fn breakfast(input: &str) -> u32 {
    let calibrations: Vec<u32> = input
        .lines()
        .map(|calibration| {
            let mut digits = calibration.chars().filter(|c| c.is_digit(10));
            let first = digits
                .next()
                .expect("the calibration should contain one digit");
            let num = match digits.last() {
                Some(last) => format!("{}{}", first, last),
                None => format!("{}{}", first, first),
            };
            num.parse::<u32>().unwrap()
        }).collect();
    println!("calibrations:\n{:?}", calibrations);
    calibrations.iter().sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 55172);
    }

}