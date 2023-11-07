
pub fn flimit(input: u64) -> u64 {
  let rqfloat: f64 = 1.0 + (input as f64).sqrt();
  return rqfloat as u64;
}

pub fn verprime(input: u64) -> bool {
  if input == 0 {return false;}
  if input == 1 {return false;}
  if input == 2 {return true;}
  if input == 3 {return true;}
  for i in 2..=flimit(input) {if input % i == 0 {return false;}}
  return true;
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
  while b != 0 {
    let tmp = a;
    a = b;
    b = tmp % b;
  }
  return a;
}