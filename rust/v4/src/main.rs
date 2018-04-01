// compute the N fibonacci_number using vec, slice, and function

fn main() {
  const N: i32 = 10;
  let mut numbers = vec![0, 1];
  
  // note: (a..b) is inclusive (a) and exclusive (b)
  for _ in 0..N {
    let nn = numbers.len();
    
    // need to split it into two lines to avoid borrow checker
    // error: cannot borrow `numbers` as mutable more than once at a time
    numbers.push(compute_next(&mut numbers[nn-2..nn]));
    
    // note: [a..b] is inclusive (a) and exclusive (b)
    //let nex = compute_next(&mut numbers[nn-2..nn]);
    //numbers.push(nex);
  }
  
  println!("");
  println!("{:?}", numbers);
}

fn compute_next(numbers: &mut [i32]) -> i32{
  println!("{:?}", numbers);
  numbers[0] + numbers[1]
}