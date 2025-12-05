use std::collections::HashSet;
use std::fs;
use std::cmp;
use regex::Regex;

#[allow(unused_variables)]
fn part_1(input: &str) -> u64
{
    let mut sum = 0;
    let mut range_vec = vec![];
    let mut id_vec = vec![];
    let mut reading_ranges = true;

    for line in input.lines()
    {
        if line.is_empty()
        {
            reading_ranges = false;
        }

        match reading_ranges
        {
            true =>
            {
                let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
                for (_, [col0, col1]) in re.captures_iter(line).map(|c| c.extract())
                {
                    let range = (col0.parse::<u64>().unwrap(),col1.parse::<u64>().unwrap());
                    range_vec.push(range);
                }
            }
            false =>
            {
                let re = Regex::new(r"^(\d+)$").unwrap();
                for (_, [col0]) in re.captures_iter(line).map(|c| c.extract())
                {
                    let id = col0.parse::<u64>().unwrap();
                    id_vec.push(id);
                }
            }
        }
    }

    id_vec.sort();
    let mut fresh_vec = vec![false; id_vec.len()];

    for range in range_vec
    {
        let lower_id;
        let upper_id;

        let mut idx_0 = 0;
        let mut idx_1 = id_vec.len();

        while idx_0 < idx_1 - 1
        {
            let actual_pos = (idx_1 + idx_0) / 2;
            let actual_value = id_vec[actual_pos];
            if actual_value < range.0
            {
                idx_0 = actual_pos;
            }
            else if actual_value > range.0
            {
                idx_1 = actual_pos;
            }
            else    // actual_value == range.0
            {
                idx_0 = actual_pos;
                idx_1 = idx_0;
            }
        }

        lower_id = idx_0;

        let mut idx_0 = 0;
        let mut idx_1 = id_vec.len();

        while idx_0 < idx_1 - 1
        {
            let actual_pos = (idx_1 + idx_0) / 2;
            let actual_value = id_vec[actual_pos];
            if actual_value < range.1
            {
                idx_0 = actual_pos;
            }
            else if actual_value > range.1
            {
                idx_1 = actual_pos;
            }
            else    // actual_value == range.0
            {
                idx_0 = actual_pos;
                idx_1 = idx_0;
            }
        }

        upper_id = idx_1;

        for idx in lower_id..=upper_id
        {
            if range.0 <= id_vec[idx] && id_vec[idx] <= range.1
            {
                fresh_vec[idx] = true;
            }
        }
    }

    for i in 0..fresh_vec.len()
    {
        if fresh_vec[i]
        {
            sum += 1;
        }
    }

    sum
}

pub trait Combine
{
    fn combine_range(&self, range: (u64,u64)) -> Option<(u64,u64)>;
}

impl Combine for (u64,u64)
{
    fn combine_range(&self, range: (u64,u64)) -> Option<(u64,u64)>
    {
        let mut combined_range: (u64,u64) = (0,0);
        let mut can_combine = false;

        if range.0 <= self.0 && self.0 <= range.1
        {
            can_combine = true;
        }

        if range.0 <= self.1 && self.1 <= range.1
        {
            can_combine = true;
        }

        if can_combine
        {
            combined_range.0 = cmp::min(self.0,range.0) ;
            combined_range.1 = cmp::max(self.1,range.1);
            return Some((combined_range.0, combined_range.1));
        }

        None
    }
}

#[allow(unused_variables)]
fn part_2(input: &str) -> u64
{
    let mut sum = 0;
    let mut range_vec = vec![];
    let mut reading_ranges = true;

    for line in input.lines()
    {
        if line.is_empty()
        {
            reading_ranges = false;
        }

        match reading_ranges
        {
            true =>
            {
                let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
                for (_, [col0, col1]) in re.captures_iter(line).map(|c| c.extract())
                {
                    let range = (col0.parse::<u64>().unwrap(),col1.parse::<u64>().unwrap());
                    range_vec.push(range);
                }
            }
            false => {}
        }
    }

    let mut modified = true;

    while modified
    {
        modified = false;

        for i in 0..range_vec.len()
        {
            for j in 0..range_vec.len()
            {
                let x =  range_vec[i];
                let y = range_vec[j];
                if x != y
                {
                    match (x).combine_range(y)
                    {
                        Some(r) =>
                        {
                            modified = true;
                            range_vec[i] = r;
                            range_vec[j] = r;
                        },
                        None => {}
                    }
                }
            }
        }
    }

    let mut range_set = HashSet::new();

    for it in range_vec.iter()
    {
        range_set.insert(it);
    }
    for it in range_set.iter()
    {
        sum += (it.1 - it.0) + 1;
    }

    // dbg!(range_vec);

    sum
}

#[allow(unused_variables)]
pub fn main()
{
    let input = fs::read_to_string("input/day_05/input.txt");
    // let input = fs::read_to_string("input/day_05/test_01.txt");
    let x = (1 as u64,10 as u64);

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
        let input = fs::read_to_string("input/day_05/test_01.txt").unwrap();
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn test_02()
    {
        let input = fs::read_to_string("input/day_05/test_01.txt").unwrap();
        assert_eq!(part_2(&input), 14);
    }
}