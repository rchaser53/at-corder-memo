use proconio::input;

fn main() {
  input! {
    n: usize,
  }
  let mut result = 0;
  
  for i in 1..n {
    result += i % (i+1);
  }

  println!("{}", result); 
}

fn main() {
  proconio::input!{
    n: usize,
  }
  println!("{}", (n-1) * n / 2);
}