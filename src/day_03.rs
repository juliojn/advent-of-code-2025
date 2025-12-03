use std::fs;
use regex::Regex;

fn get_max_value(vec: &Vec<u8>, start_idx: usize, vec_size: usize) -> (u8, usize)
{
    let mut max_value = vec[start_idx];
    let mut max_value_idx = start_idx;
    let mut idx = start_idx + 1;

    while idx != vec_size
    {
        if vec[idx] > max_value
        {
            max_value = vec[idx];
            max_value_idx = idx;
        }

        idx += 1;
    }

    (max_value, max_value_idx)
}

#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut sum = 0;

    for line in input.lines()
    {
        let re = Regex::new(r"(\d)").unwrap();
        // let it = re.captures_iter(line).map(|c| c.extract());
        let caps: Vec<u8> = re.find_iter(line).map(|c| c.as_str().parse::<u8>().unwrap()).collect();
        
        let upper_digit;
        let upper_digit_idx;
        let lower_digit;

        (upper_digit, upper_digit_idx) = get_max_value(&caps, 0, caps.len() - 1);
        (lower_digit, _) = get_max_value(&caps, upper_digit_idx + 1, caps.len());

        // dbg!(upper_digit, lower_digit);
        // println!();
        sum += (upper_digit * 10 + lower_digit) as u64;
    }

    sum
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;

    for line in input.lines()
    {
        let re = Regex::new(r"(\d)").unwrap();
        // let it = re.captures_iter(line).map(|c| c.extract());
        let caps: Vec<u8> = re.find_iter(line).map(|c| c.as_str().parse::<u8>().unwrap()).collect();
        let number_of_digits = 12;
        let mut digits = vec![0 as u8; number_of_digits];
        let mut result = 0;
        let mut idx = 0;

        for i in 0..number_of_digits
        {
            (digits[i], idx) = get_max_value(&caps, idx, caps.len() - ((number_of_digits - i) - 1));
            idx += 1;
            dbg!(digits[i]);
        }

        for i in 0..number_of_digits
        {
            result += digits[i] as u64 * u64::pow(10,(number_of_digits - i) as u32 - 1);
        }

        println!();
        sum += result
    }

    sum
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/day_03/input.txt");

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
        let input = fs::read_to_string("input/day_03/test_01.txt").unwrap();
        assert_eq!(part_1(&input), 357);
    }
    
    #[test]
    fn test_02()
    {
        let input = fs::read_to_string("input/day_03/test_01.txt").unwrap();
        assert_eq!(part_2(&input), 3121910778619);
    }
}