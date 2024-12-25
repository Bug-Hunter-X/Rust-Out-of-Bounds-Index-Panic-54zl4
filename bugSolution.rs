fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let index = 10;

    // Safe way to access element, handling out-of-bounds case
    match vec.get(index) {
        Some(element) => println!("Element at index {}: {}", index, element),
        None => println!("Index {} is out of bounds", index),
    }
} 