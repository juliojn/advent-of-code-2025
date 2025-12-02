use std::fs;
use regex::Regex;

fn check_invalid(number: u64) -> bool
{
    let digits = number.ilog10() +1;

    if digits % 2 == 1
    {
        return false;
    }

    let high_half = number / (u64::pow(10, digits/2));
    let low_half = number % (u64::pow(10, digits/2));
    // dbg!(number, digits, high_half, low_half);

    high_half == low_half    
}

fn get_next_invalid(number: u64, max: u64) -> Option<u64>
{
    let mut next_number = number + 1;
    
    while !check_invalid(next_number) && next_number <= max
    {
        next_number += 1;
    }

    if next_number <= max
    {
        return Some(next_number);
    }
    else
    {
        return None;    
    }
}

fn check_invalid_part_2(number: u64) -> bool
{
    let digits = number.ilog10() + 1;
    let mut is_invalid = false;

    // number of digits of each partition
    for i in 1..=digits/2
    {
        if !is_invalid && digits % i as u32 == 0
        {
            let parts_vec_size = (digits/i) as usize;
            let mut parts_vec = vec![0; parts_vec_size];

            for j in 0..parts_vec_size as u32
            {
                let divisor_0 = u64::pow(10, i * (j+1));
                let divisor_1 = u64::pow(10, i * j);
                parts_vec[j as usize] =  (number % divisor_0) / divisor_1;
            }

            let mut has_same_number = true;

            for j in 1..parts_vec_size
            {
                if parts_vec[j as usize] != parts_vec[0]
                {
                    has_same_number = false;
                }
            }

            if has_same_number
            {
                is_invalid = true;
            }
        }
    }

    is_invalid
}


fn get_next_invalid_part_2(number: u64, max: u64) -> Option<u64>
{
    let mut next_number = number + 1;
    
    while !check_invalid_part_2(next_number) && next_number <= max
    {
        next_number += 1;
    }

    if next_number <= max
    {
        return Some(next_number);
    }
    else
    {
        return None;    
    }
}


#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut sum = 0;

    for line in input.lines()
    {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        for (_, [col0, col1]) in re.captures_iter(line).map(|c| c.extract())
        {
            let low_limit = col0.parse::<u64>().unwrap();
            let high_limit = col1.parse::<u64>().unwrap();
            // dbg!(low_limit, high_limit);

            let mut number = low_limit;
            let mut max_reached= false;

            if check_invalid(number)
            {
                // dbg!(number);
                sum += number;
            }

            while !max_reached
            {
                match get_next_invalid(number, high_limit)
                {
                    Some(n) =>
                    {
                        // dbg!(n);
                        number = n;
                        sum += n;
                    }
                    None => { max_reached = true }
                }
            }

        }
    }

    sum as u64
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;

    for line in input.lines()
    {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        for (_, [col0, col1]) in re.captures_iter(line).map(|c| c.extract())
        {
            let low_limit = col0.parse::<u64>().unwrap();
            let high_limit = col1.parse::<u64>().unwrap();
            dbg!(low_limit, high_limit);

            let mut number = low_limit;
            let mut max_reached= false;

            if check_invalid_part_2(number)
            {
                dbg!(number);
                sum += number;
            }

            while !max_reached
            {
                match get_next_invalid_part_2(number, high_limit)
                {
                    Some(n) =>
                    {
                        dbg!(n);
                        number = n;
                        sum += n;
                    }
                    None => { max_reached = true }
                }
            }

        }
    }

    sum as u64
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/day_02/input.txt");

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
        let input = fs::read_to_string("input/day_02/test_01.txt").unwrap();
        assert_eq!(part_1(&input), 1227775554);
    }

    #[test]
    fn test_02()
    {
        let input = fs::read_to_string("input/day_02/test_01.txt").unwrap();
        assert_eq!(part_2(&input), 4174379265);
    }
}