use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
fn main() -> Result<(), std::io::Error> {

    
    let input_file: &str = "./inputs/day_05/input";
    {    let mut warehouse:HashMap<usize,Vec<char>> = HashMap::new();
        // Part 1
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines()  {
            match line {
                Ok(l) if l.contains('[')=> {
                    l.char_indices()
                    .filter(|x| x.1.is_alphabetic())
                    .map( | (i,c) | (1+(i-1)/4,c ))
                    .for_each(|(i,c)| {
                    
                       if let Some(d) = warehouse.get_mut(&i) {
                            d.insert(0, c)
                       } else {
                            warehouse.insert(i, vec!(c));
                       };
                    });
                
                },
                Ok(l) if l.contains("move")=> {
                    let p: Vec<usize>= l.split(" ").flat_map(|x| x.parse::<usize>()).collect();
                    let (m,f,t) = (p[0],p[1],p[2]);

                    for _ in 0..m {
                        let from = warehouse.get_mut(&f).unwrap();
                        let v = from.pop().unwrap();
                        let to = warehouse.get_mut(&t).unwrap();
                        to.push(v);
                    }
                } ,
                _ => ()
            }
        }
            
        
        for i in 1 .. warehouse.keys().len()+1 {
            let c = warehouse.get(&i).and_then(|x| x.last()).unwrap();
            print!("{c}");
        }
    }
        println!("");
    {    let mut warehouse:HashMap<usize,Vec<char>> = HashMap::new();
        // Part 2
        let file = &File::open(input_file)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines()  {
            match line {
                Ok(l) if l.contains('[')=> {
                    l.char_indices()
                    .filter(|x| x.1.is_alphabetic())
                    .map( | (i,c) | (1+(i-1)/4,c ))
                    .for_each(|(i,c)| {
                    
                       if let Some(d) = warehouse.get_mut(&i) {
                            d.insert(0, c)
                       } else {
                            warehouse.insert(i, vec!(c));
                       };
                    });
                
                },
                Ok(l) if l.contains("move")=> {
                    let p: Vec<usize>= l.split(" ").flat_map(|x| x.parse::<usize>()).collect();
                    let (m,f,t) = (p[0],p[1],p[2]);


                    let from = warehouse.get_mut(&f).unwrap();
                    let mut tmp : Vec<char> = Vec::new();
                    
                    for _ in 0..m {
                        
                        let v = from.pop().unwrap();
                        tmp.insert(0, v);
                    }
                    let to = warehouse.get_mut(&t).unwrap();
                    to.append(&mut tmp);
                } ,
                _ => ()
            }
        }
            
        
        for i in 1 .. warehouse.keys().len()+1 {
            let c = warehouse.get(&i).and_then(|x| x.last()).unwrap();
            print!("{c}");
        }
    }
  

    Ok(())
}
