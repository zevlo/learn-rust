fn main() {
    let v = vec![1, 1, 5];
    let cond1 = all_elements_less_than_k(&v, 6);
    let cond2 = sum_greater_than_s(&v, 6);       
    println!("{}", cond1 && cond2);
}


pub fn all_elements_less_than_k(v: &Vec<i32>, k: i32) -> bool {
    for i in 0..v.len() {
        if v[i] >= k {
            return false;
        }
    }
    true
}

pub fn sum_greater_than_s(v: &Vec<i32>, s: i32) -> bool {
    let mut sum = 0;
    for i in 0..v.len() {
        sum = sum + v[i];
    }
    sum > s
}
