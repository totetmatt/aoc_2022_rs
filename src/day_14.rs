use std::cmp::{max, min};
use std::collections::btree_set::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
#[derive(Debug, Eq, PartialEq, Hash)]
struct Vec2 {
    pub x: i32,
    pub y: i32,
}

enum Tile {
    Rock,
    Sand,
}
fn print_map_tile(map: &HashMap<Vec2, Tile>, part: String, nb: i32) {
    let mut file = File::create(format!("outimg/{part}/out.{nb:05}.ppm")).unwrap();

    let x_min = map.keys().map(|k| k.x).min().unwrap();
    let y_min = 0;
    let x_max = map.keys().map(|k| k.x).max().unwrap();
    let y_max = map.keys().map(|k| k.y).max().unwrap() + 2;
    let w = (x_max - x_min) + 1;
    let h = y_max + 1;
    file.write_all(b"P3\n").unwrap();
    file.write_all(format!("{w} {h}\n").as_bytes()).unwrap();
    file.write_all(b"255\n").unwrap();
    for (ch, y) in (y_min..y_max + 1).enumerate() {
        for (cw, x) in (x_min..x_max + 1).enumerate() {
            let c = cw + ch;
            match map.get(&Vec2 { x, y }) {
                Some(Tile::Sand) => file.write_all(b"255 255 0\n").unwrap(), //print!("🟨"),
                Some(Tile::Rock) => file.write_all(b"255 255 255\n").unwrap(), //print!("⬜"),
                _ if x == 500 && y == 0 => file.write_all(b"0 255 0\n").unwrap(), //print!("🌟"),

                _ => file.write_all(b"0 0 0\n").unwrap(), //print!("⬛")
            }
        }
        //println!("")
    }
}

fn fall(start: &Vec2, map: &HashMap<i32, Box<BTreeSet<i32>>>) -> Option<Vec2> {
    match map.get(&start.x) {
        Some(x) => match x.iter().find(|y| y >= &&start.y) {
            Some(y) => {
                let f = Vec2 { x: start.x, y: *y };

                let v_left = Vec2 { x: f.x - 1, y: f.y };

                let is_left_taken = map
                    .get(&(v_left.x))
                    .map(|z| z.contains(&v_left.y))
                    .unwrap_or(false);

                if !is_left_taken {
                    return fall(&v_left, map);
                }

                let v_right = Vec2 { x: f.x + 1, y: f.y };
                let is_right_taken = map
                    .get(&(v_right.x))
                    .map(|z| z.contains(&v_right.y))
                    .unwrap_or(false);

                if !is_right_taken {
                    return fall(&v_right, map);
                }

                return Some(Vec2 {
                    x: start.x,
                    y: *y - 1,
                });
            }
            _ => None,
        },
        _ => None,
    }
}

fn fall2(start: &Vec2, y_max: i32, map: &HashMap<i32, Box<BTreeSet<i32>>>) -> Option<Vec2> {
    match map.get(&start.x) {
        Some(x) => match x.iter().find(|y| y >= &&start.y) {
            Some(y) => {
                let f = Vec2 { x: start.x, y: *y };

                let v_left = Vec2 { x: f.x - 1, y: f.y };

                let is_left_taken = map
                    .get(&(v_left.x))
                    .map(|z| z.contains(&v_left.y))
                    .unwrap_or(false);

                if !is_left_taken {
                    return fall2(&v_left, y_max, map);
                }

                let v_right = Vec2 { x: f.x + 1, y: f.y };
                let is_right_taken = map
                    .get(&(v_right.x))
                    .map(|z| z.contains(&v_right.y))
                    .unwrap_or(false);

                if !is_right_taken {
                    return fall2(&v_right, y_max, map);
                }

                return Some(Vec2 {
                    x: start.x,
                    y: *y - 1,
                });
            }
            _ => Some(Vec2 {
                x: start.x,
                y: y_max - 1,
            }),
        },
        _ => Some(Vec2 {
            x: start.x,
            y: y_max - 1,
        }),
    }
}
fn main() {
    let input_file: &str = "./inputs/day_14/input";
    let file = &File::open(input_file).unwrap();
    let reader = io::BufReader::new(file);

    let input: HashSet<Vec2> = reader
        .lines()
        .flat_map(|line| {
            let line = line.unwrap();
            let elements: Vec<&str> = line.split("->").collect();
            let elements: Vec<Vec2> = elements
                .iter()
                .map(|x| {
                    let v: Vec<&str> = x.trim().split(",").collect();
                    Vec2 {
                        x: v.first().unwrap().parse::<i32>().unwrap(),
                        y: v.last().unwrap().parse::<i32>().unwrap(),
                    }
                })
                .collect();

            elements
                .as_slice()
                .windows(2)
                .flat_map(|a| {
                    let l = &a[0];
                    let r = &a[1];
                    (min(l.x, r.x)..max(l.x, r.x) + 1).flat_map(|x| {
                        (min(l.y, r.y)..max(l.y, r.y) + 1).map(move |y| Vec2 { x, y })
                    })
                })
                .collect::<HashSet<Vec2>>()
        })
        .collect();

    let mut map: HashMap<i32, Box<BTreeSet<i32>>> = HashMap::new();
    for i in input {
        let bmap = &mut map;
        match bmap.get_mut(&i.x) {
            Some(v) => v.insert(i.y),
            _ => bmap.insert(i.x, Box::from(BTreeSet::from([i.y]))).is_none(),
        };
    }

    {
        // Part 1
        let mut map = map.clone();
        //////// Completly useless, only for graphical representation
        let mut graphical_map: HashMap<Vec2, Tile> = HashMap::new();
        map.iter().for_each(|(x, vy)| {
            vy.iter().for_each(|y| {
                graphical_map.insert(Vec2 { x: *x, y: *y }, Tile::Rock);
            });
        });
        //////// Completly useless, only for graphical representation

        let mut i = 0;
        loop {
            let start = Vec2 { x: 500, y: 0 };

            let f = fall(&start, &map);
            print_map_tile(&graphical_map, "part1".to_string(), i);
            if f.is_none() {
                break;
            }
            i += 1;

            match f {
                Some(v) => {
                    map.get_mut(&v.x).map(|y| y.insert(v.y));
                    graphical_map.insert(Vec2 { x: v.x, y: v.y }, Tile::Sand)
                }
                _ => None,
            };
            //  >println!("{map:?}");
            // print_map(&map);
        }

        //println!{"part 1 : {i}"};
    }
    {
        // Part 2
        let mut map = map.clone();
        //////// Completly useless, only for graphical representation
        let mut graphical_map: HashMap<Vec2, Tile> = HashMap::new();
        map.iter().for_each(|(x, vy)| {
            vy.iter().for_each(|y| {
                graphical_map.insert(Vec2 { x: *x, y: *y }, Tile::Rock);
            });
        });
        //////// Completly useless, only for graphical representation
        let y_max = map.values().flat_map(|y| y.iter().max()).max().unwrap() + 2;

        let mut i = 0;
        let start = Vec2 { x: 500, y: 0 };
        loop {
            let f = fall2(&start, y_max, &map);
            print_map_tile(&graphical_map, "part2".to_string(), i);
            i += 1;
            if f.as_ref().unwrap() == &start {
                break;
            }

            match f {
                Some(v) => match map.get_mut(&v.x) {
                    Some(y) => {
                        y.insert(v.y);
                        graphical_map.insert(Vec2 { x: v.x, y: v.y }, Tile::Sand);
                        ()
                    }
                    _ => {
                        map.insert(v.x, Box::from(BTreeSet::from([v.y])));
                        graphical_map.insert(Vec2 { x: v.x, y: v.y }, Tile::Sand);
                        ()
                    }
                },
                _ => (),
            };
        }

        // println!{"part 2 : {i}"};
    }
}
