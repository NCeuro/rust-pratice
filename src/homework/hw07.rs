fn main() {
    let input = "European university UKRAINE";

    let swapped: String = input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c
            }
        })
        .collect();

    println!("Original: {}", input);
    println!("Swapped:  {}", swapped);
}
