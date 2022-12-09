use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
fn main() -> Result<(), std::io::Error> {
    let input_file: &str = "./inputs/day_08/test";
   
     
    let file = &File::open(input_file)?;
    let reader = io::BufReader::new(file);
    let map: Vec<Vec<u8>> =  reader.lines().into_iter().map( 
        |line|
            line.unwrap().chars().map(|x| x.to_string().parse::<u8>().unwrap()).collect()
    ).collect();

    let width = map.first().unwrap().len();
    let height = map.len();
    let line_scan: HashSet<(usize, usize)>= map.iter().enumerate().map(
        | (y,line)|{
        
            let mut valid : HashSet<(usize,usize)> = HashSet::from([(y,0), (y,line.len()-1)]);
            let mut max : u8 = line.first().unwrap().to_owned();
            for x in 1..line.len() {
                let current = line .get(x).unwrap().to_owned();
                if current> max {
                    valid.insert((y,x));
                    max = current;
                }
            };
            let mut max : u8 = line.last().unwrap().to_owned();
            for x in (0..line.len()-1).rev() {
                let current = line .get(x).unwrap().to_owned();
                if current> max {
                    valid.insert((y,x));
                    max = current;
                }
            };
            
           valid.to_owned()
        }
    )
    .reduce(
        |a,b| 
        a.union(&b).map(|x| x.to_owned()).collect()
    ).unwrap();
    

    let col_scan : HashSet<(usize, usize)> = (0..width)
    .map(
        |x|
         map
         .iter()
         .map(|y| y.get(x).unwrap().to_owned())
         .collect::<Vec<u8>>().to_owned()
        )
        .enumerate().map(
            | (x,line)|{
            
                let mut valid : HashSet<(usize,usize)> = HashSet::from([(0,x), (line.len()-1,x)]);
                let mut max : u8 = line.first().unwrap().to_owned();
                for y in 1..line.len() {
                    let current = line .get(y).unwrap().to_owned();
                    if current> max {
                        valid.insert((y,x));
                        max = current;
                    }
                };
                let mut max : u8 = line.last().unwrap().to_owned();
                for y in (0..line.len()-1).rev() {
                    let current = line .get(y).unwrap().to_owned();
                    if current> max {
                        valid.insert((y,x));
                        max = current;
                    }
                };
                
               valid.to_owned()
            }
        )
        .reduce(
            |a,b| 
            a.union(&b).map(|x| x.to_owned()).collect()
        ).unwrap();
 

    let nb_visible = line_scan.union(&col_scan).count();

    println!("{nb_visible:?}");

    // Part 2 


    let part2 = (0..height)
    .for_each(|y| 
                (0..width)
                .for_each(|x| {
                    let line = map.get(y).unwrap();

         
                    let col : Vec<u8> = map.iter().map(|h| h.get(x).unwrap().to_owned()).collect();
                    let current = line.get(x).unwrap();
                    
                   

                    println!("( x={x}, y={y} ) {current:?} {line:?} {col:?}");
                    })
        );

    Ok(())
}