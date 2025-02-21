fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.get(5); // This will panic if the index is out of bounds
    match number {
        Some(n) => println!("The number is {}", n),
        None => println!("Index out of bounds")
    }
}