use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Vec2 {
    pub x:i32,
    pub y:i32
}
impl Vec2 {
    fn adjacents(&self) -> Vec<Vec2> {
        vec!(
            Vec2{x:self.x+1,y:self.y},
            Vec2{x:self.x,y:self.y+1},
            Vec2{x:self.x-1,y:self.y},
            Vec2{x:self.x,y:self.y-1}
        )
    }
}
#[derive(Eq, PartialEq, Hash, Debug)]
struct Tile {
    pub c:char
}
impl Tile {
    fn can_go_to(&self, other:&Tile) -> bool {
        match &self.c {
            'S' => other.c == 'a' || other.c == 'b',
            'z' | 'y' if other.c == 'E' || other.c == 'S' => true,
             _ if other.c != 'E' => other.c <= char::from_u32(self.c as u32 +1).unwrap(),
             _ => false
        }
    }
}

fn solve(carte : &HashMap<Vec2,Tile>, start: &Vec2) -> Option<i32> {

    let (end,_) = carte.iter().find(| (_,x)| x.c == 'E').expect("End Should be here");
    let mut to_visit : Vec<(Vec2,i32)> = vec!((start.clone(),0));

    let mut visited : HashMap<Vec2,i32> = HashMap::new();
    loop {
       match to_visit.pop() {
        Some((v,s)) => {
            visited.insert(v.clone(),s);
       
            let current = carte.get(&v).unwrap();
            let nexts = v.adjacents();
         
            let q:Vec<(&Vec2,&Tile)>=  nexts.iter()
            .flat_map(|x| 
                (&carte)
                .get(&x)
                .and_then(|t| Some((x,t))) 
            )
            .filter(|(v,t)| current.can_go_to(t) 
            && visited.get(v).is_none()
            && to_visit.iter().find( |(n,_)|*n== **v).is_none()
        ).collect();
            q.into_iter().for_each( 
                |(v,_)| to_visit.insert(0,(v.to_owned(),s+1))
            );

            to_visit.sort_by(|(_,a),(_,b)| b.cmp(a) );
         
           
        },
        _ => break
       }
    }
    visited.get(end).map(|x| x.to_owned())
}
fn main() {
    let input_file: &str = "./inputs/day_12/input";
    let file = &File::open(input_file).unwrap();
    let reader = io::BufReader::new(file);

    let mut carte : HashMap<Vec2,Tile> = HashMap::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x,c) in line.chars().enumerate(){
            let v = Vec2{x:x as i32,y:y as i32};
           carte.insert(v,Tile{c});

        }
    }

    let (start, _)= carte.iter().find(| (_,x)| x.c == 'S').expect("Start shold be here");
   let part1 = solve(&carte, start);
   println!("Part 1: {part1:?}");

   let res:i32 = carte.iter()
   .filter(|(_,t)| t.c == 'a' )
   .map(|(v,_)| v)
   .flat_map(|x|  solve(&carte, x))
   .min().unwrap();


    println!("Part 2: {res:?}");
   

}