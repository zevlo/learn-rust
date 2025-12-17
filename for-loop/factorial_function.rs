fn main() {
  let n = 5;
  let result = factorial(n);
  println!("{}", result);
}

pub fn factorial(n: u32) -> u32 {
  if n == 0 {
        return 1;
    }
    
    let mut result = 1;
    for i in 1..(n+1) {
        result *= i;
    }
    result
}
