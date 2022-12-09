fn another_function(value: u32) {
  println!("The value is {value}.")
}

fn main() {
  another_function(32)
  // Error
  // another_function(32, 87)
}
