fn main() {
    let mut s = String::from("hello");

    // if this assignment were not to a mutable reference 
    // the ownership is not borrowed, and there is no compiler error
    // let r1 = &mut s;
    
    // Can we only borrow mutable references?
    // let r2 = &mut s;
    
    let r1 = &mut s;
    let r2 = &mut s;

    let s2 = dangle();

    println!("{} {}", r1, r2);
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
