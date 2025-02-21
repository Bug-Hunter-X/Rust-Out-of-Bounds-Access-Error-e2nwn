fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.get(5); // Use get() to avoid panic
    match number {
        Some(n) => println!("The number is {}", n),
        None => println!("Index out of bounds")
    }
    //Alternatively, you can use if let to check for the value.
    if let Some(n) = numbers.get(2){
        println!("The number at index 2 is {}", n);
    }
} 