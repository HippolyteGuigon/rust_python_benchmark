fn main() {
    let nombres = vec![1, 2, 3, 4, 5];

    let mut somme = 0;
    for n in &nombres {
        somme += n;
    }

    println!("La somme des nombres est : {}", somme);
    println!("Le vecteur de nombres est : {:?}", nombres);
}