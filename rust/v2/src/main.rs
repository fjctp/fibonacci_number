// compute the N fibonacci_number using Vec

fn main() {
  const N: usize = 10;
  let mut numbers = vec![0, 1];
  
  // note: (a..b) is inclusive (a) and exclusive (b)
  for i in (0..N) { 
    let nn = numbers.len();
    let next = numbers[nn-1] + numbers[nn-2];
    println!("{}, {}", i+1, next);
    numbers.push(next);
  }
  
  println!("");
  println!("{:?}", numbers);
}
