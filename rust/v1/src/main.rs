// compute the N fibonacci_number using array

fn main() {
  const N: usize = 10 + 2;
  let mut numbers: [i32; 12] = [0; 12];
  numbers[1] = 1;
  
  // note: (a..b) is inclusive (a) and exclusive (b)
  for i in (2..N) { 
    numbers[i] = numbers[i-1] + numbers[i-2];
    println!("{}", numbers[i]);
  }
  println!("");
  println!("{}", numbers[N-1]);
}
