use std::collections::HashMap;
fn main() -> Result<(), std::io::Error> {


    let mut vec = vec![1, 2, 3];
    
 
    let mut vec2 = vec!(10,11,12);
 
    vec.append(&mut vec2);
    println!("{vec:?}");
    println!("{vec2:?}");
    Ok(())
}