fn main() {
  let _v1: Vec<u32> = vec![1, 2, 3];
  let _v2: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
  let _v3: Vec<&str> = my_vec!["A", "B", "C", "D", "E"];
}

#[macro_export]
macro_rules! my_vec {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec:: new();
      $(
        temp_vec.push ($x);
        println!($x);
      )*
      temp_vec
    }
  };
}