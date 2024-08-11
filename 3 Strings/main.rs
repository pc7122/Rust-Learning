fn main() {
    // by default the string is of &str type
    // &str type is fixed size string
    let name: &str = "Prathamesh Chaudhary";
    println!("Hello, {}!", name);

    // String type is mutable and growable
    let empty_string: String = String::new();
    let new_string: String = String::from("New String");

    println!("length of empty string: {}", empty_string.len());
    println!("length of new string: {}", new_string.len());
}