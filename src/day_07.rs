use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
fn main() -> Result<(), std::io::Error> {
    let input_file: &str = "./inputs/day_07/test";
    {
        let mut fs : HashMap<(String,Vec<String>),i32> = HashMap::new();
        fs.insert((("d".to_string()), vec!("/".to_string())), 0i32);
        
        let mut context: Vec<String> = Vec::new();

        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(l)   => {
                    let q: Vec<&str> = l.split(" ").collect();
                        match q {
                            _ if q[0] == "$" => match q[1] {
                                "cd" => {match q[2] {
                                    ".." => {context.pop(); () },
                                    dir => context.push(dir.to_string()) ,
                                }
                                println!("{context:?}")
                            },
                                "ls" => println!("LS"),
                                _ => ()
                            },
                            _ => ()
                        }
                         
                    ()
                },
                Err(_)          => ()
            }
        }

        println!("{fs:?}");
    }
    Ok(())
}
