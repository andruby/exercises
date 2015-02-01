// 0.90s on rust 1.0.0-alpha
fn main() {
  let mut primes: [u32; 10000] = [0; 10000];
  let mut x = 2;
  let mut len = 0;
  while len < 10000 {
    let mut is_prime = true;
    for i in range(0, len) {
      if x % primes[i] == 0 {
        is_prime = false;
        break;
      }
    }
    if is_prime {
      primes[len] = x;
      len = len + 1;
    }
    x = x+1;
  }
  for &v in primes.iter() {
    print!("{} ", v);
  } 
}
