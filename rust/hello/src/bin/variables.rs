fn main() {
    println!("Bonjour");
    let age = 30;
    println!("J'ai {} ans", age);
    let is_even = is_even(age);
    println!("Est-ce que mon âge est pair ? {}", is_even);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        true
    }
    else {
        false
    }
}