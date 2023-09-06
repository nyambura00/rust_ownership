// TASK: create a simple Rust program that demonstrates the concepts of ownership, 
// borrowing, and references. 
// The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.

fn concatenate_strings(string_one: &str, string_two: &str) -> String {
    let mut result = String::from(string_one);
    result.push_str(string_two);
    result
}

fn main() {
    let string1: String = String::from("Jambo");
    let string2: String = String::from("Kenya");

    let concatenated_string: String = concatenate_strings(&string1, &string2);
    println!("The result: {}", concatenated_string);
}
