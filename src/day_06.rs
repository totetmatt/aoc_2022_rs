use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
fn main() -> Result<(), std::io::Error> {
    let input_file: &str = "./inputs/day_06/input";
    {
        let file = &File::open(input_file)?;
        let mut reader = io::BufReader::new(file);
        let mut s: String = String::new();
        reader
            .read_line(&mut s)
            .and_then(|_| {
                for j in vec![4_usize, 14] {
                    for i in j..s.len() {
                        if j == HashSet::<char>::from_iter((&s[i - j..i]).chars()).len() {
                            println!("{i}");
                            break;
                        }
                    }
                }
                Ok(())
            })
            .expect("Error");
    }
    Ok(())
}
