use std::{fs};
use regex::Regex;
#[derive(Debug)]

struct Rotation
{
    orientation: char,
    shift: i16,
}

#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut rotation_vec: Vec<Rotation> = vec![];
    let mut sum = 0;
    let mut dial = 50;
    let dial_numbers = 100;

    for line in input.lines()
    {
        let re = Regex::new(r"^([LR])(\d+)$").unwrap();
        for (capture, [col0, col1]) in re.captures_iter(line).map(|c| c.extract())
        {
            let value = Rotation { orientation: col0.parse::<char>().unwrap(), shift: col1.parse::<i16>().unwrap()};
            rotation_vec.push(value);
        }
    }


    for i in 0..rotation_vec.len()
    {
        // dbg!(&rotation_vec[i]);
        match  rotation_vec[i].orientation {
            'L' => { dial = (dial - rotation_vec[i].shift) % dial_numbers },
            'R' => { dial = (dial + rotation_vec[i].shift) % dial_numbers },
            _ => { panic!("") }
        }

        if dial == 0
        {
            sum += 1;
        }
    }
    sum
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;
    let mut dial: i16 = 50;
    let dial_numbers = 100;

    for line in input.lines()
    {
        let re = Regex::new(r"^([LR])(\d+)$").unwrap();
        for (capture, [col0, col1]) in re.captures_iter(line).map(|c| c.extract())
        {
            let rotation = Rotation { orientation: col0.parse::<char>().unwrap(), shift: col1.parse::<i16>().unwrap()};
            // dbg!(capture, dial, sum);
            
            let mut shift;

            if rotation.shift >= dial_numbers
            {
                shift = rotation.shift % dial_numbers;
                sum += (rotation.shift / dial_numbers) as u64;
            }
            else
            {
                shift = rotation.shift;
            }
            
            match  rotation.orientation
            {
                'L' =>
                { 
                    if dial != 0
                    {
                        dial -= shift;
                        
                        if dial < 0
                        {
                            dial += dial_numbers;
                            sum += 1;
                        }                
                    }
                    else
                    {
                        dial -= shift;
                        
                        if dial < 0
                        {
                            dial += dial_numbers;
                        }
                    }
                },
                'R' =>
                {
                    dial += shift;

                    while dial > dial_numbers
                    {
                        dial -= dial_numbers;
                        sum += 1;
                    }
                },
                _ => { panic!("") }
            }
            match dial
            {
                0 => { sum += 1 },
                100 =>
                {
                    dial -= dial_numbers;
                    sum += 1;
                }
                _ => {}
            }

            // dbg!(&dial, sum);
            println!("");
        }
    }
    sum
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/day_01/input.txt");

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
        let input = fs::read_to_string("input/day_01/test_01.txt").unwrap();
        assert_eq!(part_1(input.as_str()), 3);
    }

    #[test]
    fn test_02()
    {
        let input = fs::read_to_string("input/day_01/test_01.txt").unwrap();
        assert_eq!(part_2(input.as_str()), 6);
    }
}