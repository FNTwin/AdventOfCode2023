fn parse_input(input: &str) -> u32 {
    input
        .lines()
        .filter(|line: &&str| !line.is_empty())
        .map(|line: &str| line.to_string())
        .map(|line: String| {
            line.chars()
                .filter_map(|c: char| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec: Vec<u32>| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

fn main() {
    let input: &str = include_str!("../input.txt");
    println!("{}", parse_input(input));
}
