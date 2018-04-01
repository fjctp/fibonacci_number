// compute the N fibonacci_number using array, slice, and function

fn main() {
  const N: usize = 10;
  let mut numbers: [i32; 10+2] = [0; 10+2];
  numbers[1] = 1;
  
  // note: (a..b) is inclusive (a) and exclusive (b)
  for i in (2..N+2) {
    add_next(&mut numbers[i-2..i+1]);
  }
  
  println!("");
  println!("{:?}", numbers);
}

fn add_next(numbers: &mut [i32]){
  numbers[2] = numbers[0] + numbers[1];
}