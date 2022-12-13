use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input_file: &str = "./inputs/day_10/input";
    let file = &File::open(input_file).unwrap();
    let reader = io::BufReader::new(file);
    let mut states: Box<Vec<i32>> = Box::from(vec![1]);

    reader.lines().for_each(|line| {
        let l = line.unwrap();
        let op: Vec<&str> = l.split_whitespace().collect();
        let last = (states).last().unwrap();
        match op[..] {
            ["noop"] => (states).push(*last),
            ["addx", b] => (states).append(&mut vec![*last, *last + b.parse::<i32>().unwrap()]),
            _ => (),
        }
    });

    let part1: i32 = (19..220)
        .step_by(40)
        .map(|x| (x + 1 as i32) * states.get(x as usize).unwrap())
        .sum();

    println!("Part 1 {part1:?}");

    states.iter().enumerate().for_each(|(idx, x)| {
        let c = (idx % 40) as i32;
        if idx > 1 && c == 0 {
            println!()
        };
        if c >= (*x - 1) && c <= (*x + 1) {
            print!("⬜️")
        } else {
            print!("⬛️")
        };
    });
}
