fn main() {
    let v = vec![1, 2, 3];    
    let result = is_sorted(v);
    println!("{}", result);
}

pub fn is_sorted(v: Vec<i32>) -> bool {
    if v.len() <= 1 { // A vector with 0 or 1 elements is always sorted
        return true;
    }
    
    for i in 0..v.len()-1 { // Loop up to the second-to-last element
        if v[i] > v[i + 1] { // Check if the current element is greater than the next
            return false; // If it is, the vector is not sorted
        }
    }
    true // If the loop completes, the vector is sorted
}
