use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    
    println!("{}", set.contains(&10));
    println!("{}", set.contains(&11));
} 
