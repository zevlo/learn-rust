fn main() {
	let v = vec![1,2,3,2,1];
	
	let result = is_palindrome(v);
	println!("{}", result);
}

pub fn is_palindrome(v: Vec<i32>) -> bool {
    let mut left = 0;
    let mut right = v.len().saturating_sub(1);

    while left < right {
        if v[left] != v[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
