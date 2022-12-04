use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn value(c: &char) -> i32 {
    if c.is_lowercase() {
        (*c as u32 - 96) as i32
    } else {
        (*c as u32 - 38) as i32
    }
}
fn main() -> Result<(), std::io::Error> {
    let input_file: &str = "./inputs/day_03/input";
    {
        // Part 1
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        let res: i32 = reader
            .lines()
            .map(|line| match line {
                Ok(l) => {
                    let mid = l.len() / 2;

                    let (left, right) = l.split_at(mid);

                    let mut left: Vec<char> = left.chars().collect();
                    left.sort();

                    let mut right: Vec<char> = right.chars().collect();
                    right.sort();
                    let mut left_it = left.iter();
                    let mut right_it = right.iter();

                    let mut left_char = left_it.next().unwrap();
                    let mut right_char = right_it.next().unwrap();
                    while left_char != right_char {
                        if left_char < right_char {
                            left_char = left_it.next().unwrap();
                        } else {
                            right_char = right_it.next().unwrap();
                        }
                    }
                    value(left_char)
                }
                Err(_) => 0i32,
            })
            .sum();
        println!("part 1 : {res}");
    }

    {
        // Part 2
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        let mut res = reader.lines().into_iter();
        let mut sum = 0i32;
        while let Some(Ok(first)) = res.next() {
            let second = res.next().unwrap().unwrap();
            let third = res.next().unwrap().unwrap();

            let first_set: HashSet<char> = HashSet::from_iter(first.chars().into_iter());
            let second_set: HashSet<char> = HashSet::from_iter(second.chars().into_iter());
            let third_set: HashSet<char> = HashSet::from_iter(third.chars().into_iter());

            let intersection: HashSet<char> =
                first_set.intersection(&second_set).map(|x| *x).collect();
            let intersection: Vec<char> =
                intersection.intersection(&third_set).map(|x| *x).collect();
            let c = intersection.first().unwrap();
            sum += value(c);
        }
        println!("part 2 : {sum}");
    }

    Ok(())
}
