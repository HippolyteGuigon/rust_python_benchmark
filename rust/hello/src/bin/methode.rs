struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

impl Rectangle {
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }

    fn est_carre(&self) -> bool {
        self.largeur == self.hauteur
    }
}

fn main() {
    let r = Rectangle {
        largeur: 4.0,
        hauteur: 5.0,
    };

    println!("Aire: {}", r.aire());
    println!("Est un carré ? {}", r.est_carre())
}