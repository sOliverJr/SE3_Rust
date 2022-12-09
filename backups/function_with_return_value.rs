fn square(value: u32) -> u32{
  // Kein Semikolon für den Rückgabewert
  value * value
}

fn main() {
  let zahl: u32 = 32;
  let quadrat : u32 = square(zahl);

  println!("Das Quadrat von {zahl} ist {quadrat}.");
}
