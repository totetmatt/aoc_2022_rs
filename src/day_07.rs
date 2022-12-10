use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
fn main() -> Result<(), std::io::Error> {
    let input_file: &str = "./inputs/day_07/test";
    {
        let mut fs : HashMap<(String,Vec<String>),i32> = HashMap::new();
        fs.insert((("d".to_string()), vec!("/".to_string())), 0i32);
        
        let mut context: Vec<String>= Vec::new();
        
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
    
            match line {
             
                Ok(l)   => {
                    let q: Vec<&str> = l.split(" ").collect();
                        match q {
                            _ if q[0] == "$" => match q[1] {
                                
                                "cd" => {match q[2] {
                                    ".." => {
                                        context.pop(); 
                                        () },
                                    dir => context.push(dir.to_string()) ,
                                }
                             
                            },
                               
                                _ => ()
                            },
                            _ if q[0] == "dir" => {
                                let mut d = context.to_vec();
                                d.push(q[1].to_string());
                                fs.insert(("d".to_string(), d), 0i32);
                                ()
                            },
                            _ => {
                                let mut d = context.to_vec();
                                d.push(q[1].to_string());
                                fs.insert(("f".to_string(), d),    q[0].parse::<i32>().unwrap());
                                ()
                            }
                        }
                         
                    ()
                },
                Err(_)          => ()
            }
        }

        let mut k : Vec<&(String,Vec<String>)> = fs.keys()
                                                    .into_iter()
                                                    .filter(|(x,_)| x== "d")
                                                    .collect();
        k.sort_by(|(_,a),(_,b)|b.len().partial_cmp(&a.len()).unwrap());
    
        
      
        k.iter()
        .for_each(
            | z | {
            let (tx, x) = z;
            let q: i32 = fs.iter()
            .filter(
                |((_,d),_)| &d[0..d.len()-1] == x.as_slice()
            )
            .map(|((_,_),y)| y )
            .sum();
            fs.entry((tx.to_owned(),x.to_owned())).and_modify(|x| *x=q);
        }
        );
    }
    Ok(())
}
