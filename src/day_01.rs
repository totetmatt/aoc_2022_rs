use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Reverse;
fn main() {
    let mut elves=vec![0];
    if let Ok(lines) = read_lines("./inputs/day_01/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                match ip.parse::<i64>() {
                    Ok(v) => 
                        {
                            let last = elves.last_mut().unwrap() ;
                            *last += v
                        },
                    _ => elves.push(0),
                }
            }
        }
    }
    elves.sort_by_key(|w| Reverse(*w));
    let max = elves.first().unwrap();
    println!("{max:?}");
    let top3 = elves[0..3].iter();
    let maxes : i64 = top3.sum();
    println!("{maxes}")

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}