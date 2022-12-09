fn fläche (x: u32, y: u32) -> u32{
  x * y
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  
  fn it_works() {
    let _fläche = fläche(30, 40);
    assert_eq!(_fläche, 1200);
  }
}