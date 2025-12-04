use std::fs;
use regex::Regex;

fn get_collision(matrix: &Vec<Vec<char>>, i: usize, j: usize, char_to_find: char) -> u8
{
    let mut collision = 0;
    let mut pos_incr = vec![];
    for i in -1..=1
    {
        for j in -1..=1
        {
            if i != 0 || j != 0
            {
                pos_incr.push((i,j));
            }
        }
    }

    for pos in pos_incr
    {
        let new_i = i as i32 + pos.0;
        let new_j = j as i32 + pos.1;
        if 0 <= new_i && new_i < matrix.len() as i32
        {
            if 0 <= new_j && new_j < matrix[i].len() as i32
            {
                if matrix[new_i as usize][new_j as usize] == char_to_find
                {
                    collision += 1;
                }
            }
        }
    }

    
    // for i_incr in -1..=1 as isize
    // {
    //     match matrix.get(i + i_incr as usize)
    //     {
    //         Some(v) =>
    //         {
    //             for j_incr in -1..=1 as isize
    //             {
    //                 match v.get(j + j_incr as usize)
    //                 {
    //                     Some(c) =>
    //                     {
    //                         if *c == char_to_find
    //                         {
    //                             collision += 1;
    //                         }
    //                     }
    //                     None => {}
    //                 }
    //             }
    //         }
    //         None => {}
    //     }
    // }
    
    collision
}

#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>>= vec![];
    let max_collision = 4;

    for line in input.lines()
    {
        let re = Regex::new(r"(\S)").unwrap();
        let caps: Vec<char> = re.find_iter(line).map(|c| c.as_str().parse::<char>().unwrap()).collect();
        matrix.push(caps);
    }

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == '@'  
            {
                if get_collision(&matrix, i, j, '@') < max_collision
                {
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>>= vec![];
    let max_collision = 4;

    for line in input.lines()
    {
        let re = Regex::new(r"(\S)").unwrap();
        let caps: Vec<char> = re.find_iter(line).map(|c| c.as_str().parse::<char>().unwrap()).collect();
        matrix.push(caps);
    }

    let mut next_matrix = vec![vec!['.';matrix[0].len()];matrix.len()];
    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            next_matrix[i][j] = matrix[i][j]
        }
    }

    loop
    {
        let mut removed = 0;
        
        for i in 0..matrix.len()
        {
            for j in 0..matrix[i].len()
            {
                matrix[i][j] = next_matrix[i][j]
            }
        }

        for i in 0..matrix.len()
        {
            for j in 0..matrix[i].len()
            {
                match matrix[i][j]
                {
                    '@' =>
                    {
                        if get_collision(&matrix, i, j, '@') < max_collision
                        {
                            next_matrix[i][j] = 'x';
                            removed += 1;
                            sum += 1;
                        }
                    },
                    c => { next_matrix[i][j] = c }
                }
            }
        }

        if removed == 0
        {
            break;
        }
    }

    sum
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/day_04/input.txt");

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
        let input = fs::read_to_string("input/day_04/test_01.txt").unwrap();
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn test_02()
    {
        let input = fs::read_to_string("input/day_04/test_01.txt").unwrap();
        assert_eq!(part_2(&input), 43);
    }
}