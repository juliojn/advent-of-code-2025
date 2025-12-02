use std::fs;
use regex::Regex;

#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut sum = 0;

    for line in input.lines()
    {
        let re = Regex::new(r"^(\d+)\s+(\d+)$").unwrap();
        for (_, [col0, col1]) in re.captures_iter(line).map(|c| c.extract())
        {
            sum += col0.parse::<u64>().unwrap() * col1.parse::<u64>().unwrap();
        }
    }

    sum
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    0
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/template/input.txt");

    match input
    {
        Ok(input) =>
        {
            let result = part_1(&input);
            println!("part 1: {result}");
            let result = part_2(&input);
            println!("part 2: {result}");
        },
        Err(error) =>
        {
            panic!("{error}");
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01()
    {
        let input = fs::read_to_string("input/template/test_01.txt").unwrap();
        assert_eq!(part_1(&input), 1);
    }
}