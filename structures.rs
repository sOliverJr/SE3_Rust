struct Rechteck {
  breite: u32,
  höhe: u32,
}

impl Rechteck {
  // Wenn self, &self oder &mut self als Parameter übergeben wird, 
  // handelt es sich um eine Methodenfunkion.
  fn fläche(&self) -> u32 {
    self.breite * self.höhe
  }
}

impl Rechteck {
  // Zu Rechteck dazugehörende Funktion (vgl. mit Klassenfunktion).
  fn quadrat(kantenlänge: u32) -> Rechteck { 
      Rechteck {
        breite: kantenlänge,
        // höhe: kantenlänge,
    } 
  }
}

fn main(){
  let _rechteck = Rechteck {
    breite: 30,
    höhe: 40
  };

  println!("Die Fläche des Rechtecks ist {}m2.", _rechteck.fläche());

  let _quadrat = Rechteck::quadrat(30);

  println!("Die Fläche des Quadrats ist {}m2.", _quadrat.fläche());
}