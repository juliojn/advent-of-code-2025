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

#[allow(unused)]
fn cuantic_tachyon_recursive(matrix: &Vec<Vec<char>>, line: usize, i: usize) -> u64
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
            sum = cuantic_tachyon_recursive(matrix, line+1, i);
        },
        '^' =>
        {
            sum = cuantic_tachyon_recursive(matrix, line+1, i+1) +
                   cuantic_tachyon_recursive(matrix, line+1, i-1);
        },
        _ => { }
    }

    sum
}

#[allow(unused)]
fn cuantic_tachyon_iterative(matrix: &Vec<Vec<char>>, start_line: usize, start_i: usize) -> u64
{
    let mut sum = 0;
    let mut pos_vec = Vec::new();

    pos_vec.push((start_line, start_i));

    while !pos_vec.is_empty()
    {
        let pos = pos_vec.pop().unwrap();
        let line = pos.0;
        let i = pos.1;

        if line == matrix.len() - 1
        {
            sum += 1;
        }
        else if i < matrix[line+1].len()
        {
            match matrix[line+1][i]
            {
                '.' =>
                {
                    pos_vec.push((line+1, i));
                },
                '^' =>
                {
                    pos_vec.push((line+1, i+1));
                    pos_vec.push((line+1, i-1));
                },
                _ => { }
            }
        }
    }

    sum
}

#[allow(unused)]
fn cuantic_tachyon_cache(matrix: &Vec<Vec<char>>, line: usize, i: usize, cache: &mut Vec<Vec<u64>>) -> ()
{
    if i >= matrix[line].len()
    {
        return;
    }

    if line == matrix.len() - 1
    {
        cache[line][i] = 1;
    }

    if cache[line][i] != 0
    {
        return;
    }

    match matrix[line+1][i]
    {
        '.' =>
        {
            cuantic_tachyon_cache(matrix, line+1, i, cache);
            cache[line][i] = cache[line+1][i];
        },
        '^' =>
        {
            cuantic_tachyon_cache(matrix, line+1, i+1, cache);
            cuantic_tachyon_cache(matrix, line+1, i-1, cache);
            cache[line][i] = cache[line+1][i+1] + cache[line+1][i-1];
        },
        _ => { }
    }
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut cache: Vec<Vec<u64>> = Vec::new();

    for line in input.lines()
    {
        matrix.push(line.chars().collect());
    }


    let start_vec = find_char(&matrix, 0, 'S');

    for it in start_vec.iter()
    {
        cache = vec![vec![0 ;matrix[0].len()];matrix.len()];
        cuantic_tachyon_cache(&matrix, 1, *it, &mut cache);
        sum += cache[1][*it];
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