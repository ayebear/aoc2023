pub fn print() {
    let data = include_str!("../input1.txt");
    println!("Day 1: {}, {}", part1(data), part2(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}

pub fn part1(data: &str) -> i32 {
    data.lines().map(parse1).sum()
}

fn parse1(line: &str) -> i32 {
    let first = line.chars().find(|c| c.is_numeric()).expect("digit");
    let last = line.chars().rfind(|c| c.is_numeric()).expect("digit");
    format!("{first}{last}").parse().unwrap()
}

pub fn part2(data: &str) -> i32 {
    data.lines().map(parse2).sum()
}

fn parse2(line: &str) -> i32 {
    let mut iter = line
        .char_indices()
        .flat_map(|(i, _c)| get_digit(&line[i..]));
    let first = iter.next().expect("first digit in string");
    let last = iter.last().unwrap_or(first);
    format!("{first}{last}").parse().unwrap()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digit(s: &str) -> Option<i32> {
    s.chars()
        .next()
        .and_then(|c| c.to_string().parse().ok())
        .or_else(|| {
            DIGITS.iter().enumerate().find_map(|(i, &d)| {
                if s.starts_with(d) {
                    return Some(i as i32 + 1);
                };
                None
            })
        })
}
