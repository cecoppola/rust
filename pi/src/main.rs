use std::env;
use std::collections::VecDeque;
use std::time::{Instant};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fs::File;
use std::io::Write;
mod functions;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let e1: u32 = args[1].parse::<u32>().unwrap();
  let ptot = AtomicUsize::new(0);
  let k1l: u64 = 210;
  let mut fset = VecDeque::<u8>::new();
  for i in 0..k1l { if functions::gcd(i,k1l) == 1 {fset.push_back(i as u8);} }  
  let rangediv: u64 = rayon::current_num_threads() as u64;
  println!("Using {} threads.",rangediv);
  let mut file1 = File::create("wheeltimes.txt")?;
  for exp in 10..e1 {
    let start = Instant::now();
    let n1 = 2_u64.pow(exp);    
    let wdist: u64 = (n1/k1l)/rangediv;
    ptot.store(0, Ordering::SeqCst);
    if wdist != 0 {
      for i in 1..=k1l/2 { if functions::verprime(i as u64) { if k1l % i == 0 { ptot.fetch_add(1, Ordering::SeqCst); } } }
      if functions::verprime(k1l) {ptot.fetch_add(1, Ordering::SeqCst);} 
    }
    rayon::scope(|s| {
      let fset = &fset;
      let ptot = &ptot;
      s.spawn_broadcast(|_,comm| {
        let wstart: u64 = wdist*(comm.index() as u64);
        for i in wstart..wstart + wdist {
          let mut tcount: usize = 0;
          for j in 0..fset.len() { if functions::verprime(i*k1l + (fset[j] as u64)) { tcount+=1; } }
          ptot.fetch_add(tcount, Ordering::SeqCst);
        }
      });
    });
    for i in k1l*rangediv*wdist..n1 { if functions::verprime(i) { ptot.fetch_add(1, Ordering::SeqCst); } }
    let ncount: u32 = ptot.load(Ordering::Acquire).try_into().unwrap();
    let timesum = start.elapsed().as_millis() as f32;
    println!("{exp:>3} \t x = {n1:>11} \t pi(x) = {ncount:>10} \t Time = {: >7}ms", timesum);
    write!(&mut file1, "{exp:>3} \t x = {n1:>11} \t pi(x) = {ncount:>10} \t Time = {: >7}ms \n", timesum)?;
  }
  Ok(())
}