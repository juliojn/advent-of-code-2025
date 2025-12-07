use std::fs;
use std::ops::Range;
use regex::Regex;

enum Oper
{
    ADD,
    MUL,
}

#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<u64>> = Vec::new();
    let mut oper_vec= vec![];

    for line in input.lines()
    {
        let re = Regex::new(r"(\d+)").unwrap();
        let caps: Vec<u64> = re.find_iter(line).map(|c| c.as_str().parse::<u64>().unwrap()).collect();
        if !caps.is_empty()
        {
            matrix.push(caps.clone());
        }

        let re = Regex::new(r"([\+\*])").unwrap();
        let caps: Vec<char> = re.find_iter(line).map(|c| c.as_str().parse::<char>().unwrap()).collect();
        if !caps.is_empty()
        {
            oper_vec = caps.clone();
        }
    }

    // dbg!(&matrix, &oper_vec);

    for i in 0..matrix.len()
    {
        assert_eq!(oper_vec.len(), matrix[i].len());
    }

    let mut result_vec = vec![0;oper_vec.len()];

    for j in 0..oper_vec.len()
    {
        let oper;

        match oper_vec[j]
        {
            '+' => { oper = Oper::ADD; result_vec[j] = 0 },
            '*' => { oper = Oper::MUL; result_vec[j] = 1 },
            _ =>   { panic!() }
        }

        for i in 0..matrix.len()
        {
            match oper
            {
                Oper::ADD => { result_vec[j] += matrix[i][j] }
                Oper::MUL => { result_vec[j] *= matrix[i][j] }
            }
        }

        sum += result_vec[j];
    }

    sum
}

fn get_digit(n: u64, d: u8) -> u8
{
    ((n % u64::pow(10, (d+1).into())) / u64::pow(10, d.into())) as u8
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut oper_vec= vec![];

    for line in input.lines()
    {
        let re = Regex::new(r"^(\d+\s+)$").unwrap();
    let caps: Vec<char> = re.captures_iter(line).map(|c| c.as_str()).collect();
        if !caps.is_empty()
        {
            matrix.push(caps.clone());
        }

        let re = Regex::new(r"([\+\*])").unwrap();
        let caps: Vec<char> = re.find_iter(line).map(|c| c.as_str().parse::<char>().unwrap()).collect();
        if !caps.is_empty()
        {
            oper_vec = caps.clone();
        }
    }
    dbg!(&matrix);

    sum
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/day_06/input.txt");
    // let input = fs::read_to_string("input/day_06/test_01.txt");

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
        let input = fs::read_to_string("input/day_06/test_01.txt").unwrap();
        assert_eq!(part_1(&input), 4277556);
    }

    #[test]
    fn test_02()
    {
        let input = fs::read_to_string("input/day_06/test_01.txt").unwrap();
        assert_eq!(part_2(&input), 3263827);
    }
}