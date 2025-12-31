fn main() {
    let v = vec![1, 2, 3];
    let result_ok = double_at(&v, 1);
    println!("Double index 1: {:?}", result_ok);

    let result_none = double_at(&v, 5);
    println!("Double index 5: {:?}", result_none);

    let result_empty = double_at(&Vec::<i32>::new(), 0);
    println!("Double index 0 (empty): {:?}", result_empty);
}

pub fn double_at(v: &Vec<i32>, idx: usize) -> Option<Vec<i32>> {
    if idx < v.len() {
        let mut new_v = v.clone();
        new_v[idx] = new_v[idx] * 2;
        Some(new_v)
    } else {
        None
    }
}
