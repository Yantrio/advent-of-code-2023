#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<String>) -> u32 {
    // take the first and the last element of each line
    // and sum them up
    input
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|l| l.first().unwrap() * 10 + l.last().unwrap())
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| extract_spelled_out_numbers(line))
        .map(|numbers| numbers.first().unwrap() * 10 + numbers.last().unwrap())
        .sum()
}

const DIGIT_MAP: [(&str, u32); 10] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("ten", 10),
];

fn extract_spelled_out_numbers(input: &str) -> Vec<u32> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut numbers = Vec::new();

    // iterate over each character and see if it's the start of a word (ie, "one", "two", "three", etc)
    for i in 0..chars.len() {
        if !chars[i].is_digit(10) {
            // if it's not a digit, then it's possibly the start of a word
            for (word, value) in DIGIT_MAP.iter() {
                if chars[i..].starts_with(word.chars().collect::<Vec<char>>().as_slice()) {
                    // if it is, then we can return the value of the word
                    numbers.push(*value);
                }
            }
        } else {
            // if it is a digit, then we can just return the digit
            numbers.push(chars[i].to_digit(10).unwrap());
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(solve_part1(&input_generator(SAMPLE_INPUT)), 142);
    }

    const SAMPLE_INPUT_2: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&input_generator(SAMPLE_INPUT_2)), 281);
    }

    #[test]
    fn extract_spelled_out_numbers_1() {
        assert_eq!(extract_spelled_out_numbers("one"), vec![1]);
        assert_eq!(extract_spelled_out_numbers("two1nine"), vec![2, 1, 9]);
        assert_eq!(extract_spelled_out_numbers("xtwone3four"), vec![2, 1, 3, 4]);
    }
}
