use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
#[derive(Debug, Eq, PartialEq, Clone)]
enum Elem {
    Value(String),
    SubList(String),
}
fn is_a_list(s: &String) -> bool {
    let last = s.chars().last();
    if last.is_none() {
        false
    } else {
        last.unwrap() == ']'
    }
}
fn compare(left: &Vec<Elem>, right: &Vec<Elem>) -> Option<bool> {
    let max = if left.len() < right.len() {
        right.len()
    } else {
        left.len()
    };
    for idx in 0..max {
        let l = left.get(idx);
        let r = right.get(idx);

        match (l, r) {
            (Some(Elem::Value(l)), Some(_)) if l == "" => return Some(false),
            (Some(_), Some(Elem::Value(r))) if r == "" => return Some(true),
            (Some(Elem::Value(l)), Some(Elem::Value(r))) => {
                if l != r {
                    return Some(l < r);
                }
            }
            (Some(Elem::SubList(l)), Some(Elem::SubList(r))) => match compare(&parse(l), &parse(r))
            {
                Some(c) => return Some(c),
                _ => (),
            },
            (Some(Elem::SubList(l)), Some(Elem::Value(r))) => {
                match compare(&parse(l), &parse(&String::from("[".to_owned() + r + "]"))) {
                    Some(c) => return Some(c),
                    _ => (),
                }
            }
            (Some(Elem::Value(l)), Some(Elem::SubList(r))) => {
                match compare(&parse(&String::from("[".to_owned() + l + "]")), &parse(r)) {
                    Some(c) => return Some(c),
                    _ => (),
                }
            }

            (_, None) => return Some(false),
            (None, _) => return Some(true),
        }
    }

    None
}
fn parse(s: &String) -> Vec<Elem> {
    let mut tmp: String = String::new();
    let mut list_counter: u32 = 0u32;
    let mut list: Vec<Elem> = Vec::new();
    for c in s.chars() {
        match c {
            '[' => {
                list_counter += 1;
                if list_counter > 1 {
                    (&mut tmp).push(c)
                }
            }
            ']' => {
                if list_counter > 1 {
                    (&mut tmp).push(c)
                }
                list_counter -= 1;
            }
            ',' => {
                if list_counter == 1 {
                    (&mut list).push(if is_a_list(&tmp) {
                        Elem::SubList(String::from_iter(tmp.drain(..)))
                    } else {
                        Elem::Value(String::from_iter(tmp.drain(..)))
                    })
                } else {
                    (&mut tmp).push(c)
                }
            }
            _ => (&mut tmp).push(c),
        }
    }
    (&mut list).push(if is_a_list(&tmp) {
        Elem::SubList(String::from_iter(tmp.drain(..)))
    } else {
        Elem::Value(String::from_iter(tmp.drain(..)))
    });
    list
}
fn main() {
    let input_file: &str = "./inputs/day_13/test";
    let file = &File::open(input_file).unwrap();
    let reader = io::BufReader::new(file);

    let mut input: Vec<Vec<Elem>> = reader
        .lines()
        .flat_map(|x| x)
        .filter(|x| x != "")
        .map(|x| parse(&x))
        .collect();
    let q: i32 = (0..input.len())
        .step_by(2)
        .filter(|x| {
            let left = &input[*x];
            let right = &input[x + 1];

            let result = compare(left, right);
            let idx = (x / 2) + 1;

            if result.is_none() {
                false
            } else {
                result.unwrap()
            }
        })
        .map(|x| ((x / 2) + 1) as i32)
        .sum();
    println!("Part 1 : {q}");

    let a = parse(&String::from("[[2]]"));
    let b = parse(&String::from("[[6]]"));
    input.push(a.clone());
    input.push(b.clone());

    input.sort_by(|a, b| {
        if compare(a, b).unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let p: i32 = input
        .iter()
        .enumerate()
        .filter(|(_, x)| &a == *x || &b == *x)
        .map(|(idx, _)| {
            println!("{idx}");
            (idx + 1) as i32
        })
        .product();
    println!("Part 2: {p:?}");
}
