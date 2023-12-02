use std::collections::HashMap;

pub fn print() {
    let data = include_str!("../input2.txt");
    println!("Day 2: {}, {}", part1(data), part2(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}

fn part1(data: &str) -> i32 {
    data.lines()
        .enumerate()
        .map(|(i, line)| if possible(line) { i as i32 + 1 } else { 0 })
        .sum()
}

fn possible(data: &str) -> bool {
    let data = data.split(':').last().unwrap();
    data.split(';').all(|game| {
        game.split(',').all(|cubes| {
            let mut i = cubes.trim().split(' ');
            let count = i.next().unwrap().parse().unwrap();
            let color = i.next().unwrap();
            valid_color_count(color, count)
        })
    })
}

fn valid_color_count(color: &str, count: i32) -> bool {
    match color {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => false,
    }
}

fn part2(data: &str) -> i32 {
    data.lines().map(power).sum()
}

fn power(data: &str) -> i32 {
    let data = data.split(':').last().unwrap();
    let mut colors = HashMap::new();
    data.split(';').for_each(|game| {
        game.split(',').for_each(|cubes| {
            let mut i = cubes.trim().split(' ');
            let count: i32 = i.next().unwrap().parse().unwrap();
            let color = i.next().unwrap();
            let entry = colors.entry(color).or_insert(count);
            *entry = count.max(*entry);
        });
    });
    colors.values().product()
}
