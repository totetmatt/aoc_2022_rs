use std::fs::File;
use std::io::{self, BufRead};

/*
Rock Paper Scissor

ABC
XYZ --> 3

ABC
ZXY --> 0

ABC
YZX --> 6
*/
fn main() -> Result<(), std::io::Error> {
    let abc = String::from("ABC");
    let xyz = String::from("XYZ");
    let input_file: &str = "./inputs/day_02/input";
    {
        // Part 1
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        let res: i32 = reader
            .lines()
            .map(|line| match line {
                Ok(l) => {
                    let opponent = l.chars().nth(0).unwrap();
                    let me = l.chars().nth(2).unwrap();
                    let a: i32 = abc.find(opponent).unwrap() as i32;
                    let x: i32 = xyz.find(me).unwrap() as i32;
                    x + 1i32
                        + match a - x {
                            -1 | 2 => 6i32,
                            0 => 3i32,
                            _ => 0i32,
                        }
                }
                Err(_) => 0i32,
            })
            .sum();
        println!("part 1 : {res}");
    }

    {
        // Part 2
        let res = &String::from("ABCABCABC");
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        let result: i32 = reader
            .lines()
            .map(|line| match line {
                Ok(l) => {
                    let opponent = l.chars().nth(0).unwrap();
                    let result = l.chars().nth(2).unwrap();
                    let (idx, score): (i32, i32) = match result {
                        'X' => (-1, 0),
                        'Y' => (0, 3),
                        'Z' => (1, 6),
                        _ => (0, 0),
                    };

                    res.find(
                        res.chars()
                            .nth(res.find(opponent).unwrap() + (3 + idx) as usize)
                            .unwrap(),
                    )
                    .unwrap() as i32
                        + score
                        + 1
                }
                Err(_) => 0i32,
            })
            .sum();
        println!("part 2 : {result}");
    }

    Ok(())
}
