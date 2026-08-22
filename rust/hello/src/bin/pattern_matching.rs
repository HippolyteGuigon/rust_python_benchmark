enum Figure {
    Cercle {rayon: f64},
    Rectangle {largeur: f64, hauteur: f64},
}

fn aire(figure: &Figure) -> f64 {
    match figure {
        Figure::Cercle {rayon} => 3.159 * rayon * rayon,
        Figure::Rectangle {largeur, hauteur} => largeur * hauteur,
    }
}

fn main() {
    let c=Figure::Cercle {rayon: 3.0};
    let r=Figure::Rectangle {largeur: 4.0, hauteur: 5.0};

    println!("Aire du cercle : {}", aire(&c));
    println!("Aire du rectangle : {}", aire(&r));
}