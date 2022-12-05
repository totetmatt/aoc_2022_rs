use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let input_file: &str = "./inputs/day_04/input";
    {
        // Part 1
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        let res: i32 = reader
            .lines()
            .map(|line| match line {
                Ok(l) => {
                    let t: Vec<&str> = l.split(',').collect();
                    let p: Vec<Vec<i32>> = t
                        .iter()
                        .map(|x| {
                            x.split('-')
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>()
                        })
                        .collect();
                    let (a, b) = (p.first().unwrap(), p.last().unwrap());
                    let ar = a.last().unwrap() - a.first().unwrap();
                    let br = b.last().unwrap() - b.first().unwrap();

                    let (a, b) = if br >= ar { (b, a) } else { (a, b) };

                    if a.first().unwrap() <= b.first().unwrap()
                        && a.last().unwrap() >= b.last().unwrap()
                    {
                        1i32
                    } else {
                        0i32
                    }
                }
                Err(_) => 0i32,
            })
            .sum();
        println!("part 1 : {res:?}");
    }
    {
        // Part 2
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        let res: i32 = reader
            .lines()
            .map(|line| match line {
                Ok(l) => {
                    let t: Vec<&str> = l.split(',').collect();
                    let p: Vec<Vec<i32>> = t
                        .iter()
                        .map(|x| {
                            x.split('-')
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>()
                        })
                        .collect();
                    let (a, b) = (p.first().unwrap(), p.last().unwrap());
                    let ar = a.last().unwrap() - a.first().unwrap();
                    let br = b.last().unwrap() - b.first().unwrap();

                    let (a, b) = if br >= ar { (b, a) } else { (a, b) };

                    if a.last().unwrap() < b.first().unwrap()
                        || a.first().unwrap() > b.last().unwrap()
                    {
                        0i32
                    } else {
                        1i32
                    }
                }
                Err(_) => 0i32,
            })
            .sum();
        println!("part 2 : {res:?}");
    }

    Ok(())
}
