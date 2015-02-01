// 0.90s on rust 1.0.0-alpha
fn main() {
  let mut primes = vec![];
  let mut x: u32 = 2;
  let mut len: u32 = 0;
  while len < 10000 {
    let mut is_prime = true;
    for &prime in primes.iter() {
      if x % prime == 0 {
        is_prime = false;
        break;
      }
    }
    if is_prime {
      primes.push(x);
      len = len + 1;
    }
    x = x + 1;
  }
  for &v in primes.iter() {
    print!("{} ", v);
  } 
}
