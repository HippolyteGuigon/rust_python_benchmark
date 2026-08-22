fn trouver_index(v: &Vec<i32>, cible: i32) -> Option<usize> {
    for (i, &valeur) in v.iter().enumerate() {
        if valeur == cible {
            return Some(i);
        }
    }
    None
}

fn main() {
    let nombres = vec![10, 25, 3, 47];

    match trouver_index(&nombres, 3){
        Some(index)=>println!("L'élement 3 se trouver à l'index : {}", index),
        None=>println!("L'élement 3 n'est pas dans le vecteur")
    }

    match trouver_index(&nombres, 50){
        Some(index)=>println!("L'élement 100 se trouver à l'index : {}", index),
        None=>println!("L'élement 100 n'est pas dans le vecteur")
    }
}