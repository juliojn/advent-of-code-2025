use std::fs;

fn find_char(matrix: &Vec<Vec<char>>, line: usize, c: char) -> Vec<usize>
{
    let mut pos_vec: Vec<usize> = Vec::new();

    if line >= matrix.len()
    {
        panic!()
    }

    for i in 0..matrix[line].len()
    {
        if matrix[line][i] == c
        {
            pos_vec.push(i);
        }
    }

    pos_vec
}

#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines()
    {
        matrix.push(line.chars().collect());
    }

    let start_vec = find_char(&matrix, 0, 'S');

    for it in start_vec.iter()
    {
        matrix[1][*it] = '|';
    }

    for line in 1..matrix.len() - 1
    {
        let tachyon_vec = find_char(&matrix, line, '|');

        for it in tachyon_vec.iter()
        {
            // if matrix[line+1][*it] == '^'
            match matrix[line+1][*it]
            {
                '^' =>
                {
                    sum +=1;

                    if *it != 0
                    {
                        matrix[line+1][*it-1] = '|'
                    }

                    if *it < matrix[line+1].len() - 1
                    {
                        matrix[line+1][*it+1] = '|'
                    }
                }
                '.' => { matrix[line+1][*it] = '|'; }
                _ => {}
            }
        }
    }

    sum
}

fn cuantic_tachyon(matrix: &Vec<Vec<char>>, line: usize, i: usize) -> u64
{
    let mut sum = 0;

    if i >= matrix[line].len()
    {
        return  0;
    }

    if line == matrix.len() - 1
    {
        return 1;
    }

    match matrix[line+1][i]
    {
        '.' =>
        {
            sum = cuantic_tachyon(matrix, line+1, i);
        },
        '^' =>
        {
            sum = cuantic_tachyon(matrix, line+1, i+1) +
                   cuantic_tachyon(matrix, line+1, i-1);
        },
        _ => { }
    }

    sum
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines()
    {
        matrix.push(line.chars().collect());
    }

    let start_vec = find_char(&matrix, 0, 'S');

    for it in start_vec.iter()
    {
        // Recursion is too slow for the main input
        sum += cuantic_tachyon(&matrix, 1, *it);
    }

    sum
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/day_07/input.txt");

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
        let input = fs::read_to_string("input/day_07/test_01.txt").unwrap();
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn test_02()
    {
        let input = fs::read_to_string("input/day_07/test_01.txt").unwrap();
        assert_eq!(part_2(&input), 40);
    }
}