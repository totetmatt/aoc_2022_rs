use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn range_at(&self, y: i64) -> Option<Bound> {
        let r = self.z - (self.y - y).abs();
        if r < 0 {
            None
        } else {
            Some(Bound {
                start: self.x - r,
                end: self.x + r,
            })
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Bound {
    start: i64,
    end: i64,
}

fn merge(low: &Bound, other: &Bound) -> Bound {
    if other.start - 1i64 <= low.end {
        Bound {
            start: low.start,
            end: std::cmp::max(low.end, other.end),
        }
    } else {
        low.to_owned()
    }
}
fn main() {
    let input_file: &str = "./inputs/day_15/input";
    let file = &File::open(input_file).unwrap();
    let reader = io::BufReader::new(file);

    let mut sensors: HashSet<Vec3> = HashSet::new();
    let mut beacons: HashSet<Vec2> = HashSet::new();

    let input = reader.lines();
    for line in input {
        let line = line.unwrap();
        let line: String = line
            .chars()
            .filter(|x| x.is_numeric() || *x == ',' || *x == ':' || *x == '-')
            .collect();

        let line = line
            .split(":")
            .map(|x| {
                let tmp = x
                    .to_owned()
                    .split(",")
                    .map(|y| y.to_owned().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                Vec2 {
                    x: *tmp.first().unwrap(),
                    y: *tmp.last().unwrap(),
                }
            })
            .collect::<Vec<Vec2>>();
        let beacon = line.last().unwrap();
        let signal = line.first().unwrap();
        let range = (signal.x - beacon.x).abs() + (signal.y - beacon.y).abs();
        beacons.insert(beacon.to_owned());
        sensors.insert(Vec3 {
            x: signal.x,
            y: signal.y,
            z: range,
        });
    }

    {
        let mut ranges = sensors
            .iter()
            .flat_map(|s| s.range_at(10))
            .collect::<Vec<Bound>>();
        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        let res = ranges
            .into_iter()
            .reduce(|acc, el| merge(&acc, &el))
            .unwrap();
        let size = res.end - res.start;
        println!("Part 1 : {res:?} {size:?}");
    }
    let limit: i64 = 4000000;

    for y in 0..limit + 1 {
        let mut ranges = sensors
            .iter()
            .flat_map(|s| s.range_at(y))
            .map(|x| Bound {
                start: std::cmp::max(x.start, 0),
                end: std::cmp::min(limit, x.end),
            })
            .collect::<Vec<Bound>>();
        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        let res = ranges
            .into_iter()
            .reduce(|acc, el| merge(&acc, &el))
            .unwrap();

        if res.end != limit {
            let val = (res.end + 1) * 4000000 + y;
            println!("Part 2: {res:?} {y} : {val}");
            break;
        }
    }
}
