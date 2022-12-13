use std::collections::HashMap;
fn main() {
    let a = 'c';
    let b = char::from_u32(a as u32 + 1).unwrap();

    let c = a < b;
    println!("{b:?}")
}
