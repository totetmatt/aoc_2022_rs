use std::collections::HashMap;
fn main() {
    

    let q = vec![1,2,3,4];
    let l = q.len();
    

    let q = q.iter().find(|x| x >= &&7).get_or_insert(1).unwrap_or_default();
    println!("{q:?}")
   
}
