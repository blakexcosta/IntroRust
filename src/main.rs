fn main() {
    // PART 1 - 2
    println!("Hello, Blake!");

    // Comments
    // THIS IS A COMMENT
    /* THIS IS A MULTILINE COMMENTS */

    // Variables and Basic Data Types
    let age: u32 = 5; // cant be negative
    let height: f64 = 5.10;
    let a_boolean: bool = false;
    let first_letter_of_name: char = 'B';
    let a_statement: String = String::from("I don't like the letter B");
    let mut b_statement: String = "I love sunsets over the mountains".to_string();
    println!(
        "age: {}\nheight: {}\na_boolean: {}\nfirst_letter_of_name: {}\na_statement: {}\nb_statement: {}",
        age, height, a_boolean, first_letter_of_name, a_statement, b_statement
    );

    // PART 3
    // uncomment and show this erroring out
    // a_statement = String::from("I do like the letter B"); // this will error out
    // println!("b_statement: {}", b_statement);
    b_statement = String::from("I don't like sunsets");
    println!("b_statement: {}", b_statement);

    // ASSSIGNMENT:
    // 1. Reassign Variables with the 'mut' keyword
    // 2. Print out variables before and after reassignment
    let num1: u32 = 5;
    let string1: String = String::from("hello");
}
