use std::collections::HashMap;
fn main() {
    

    let q = vec![1,2,3];
    let l = q.len();
    println!("{l:?}");
    (0..q.len()-1).rev().for_each(|x| {
        let qq = q.get(x);
        println!("{x} {qq:?}")});
}
