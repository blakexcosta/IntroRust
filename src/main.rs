fn main() {
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
    let b_statement: String = "I love sunsets over the mountains".to_string();
    println!(
        "age: {}\nheight: {}\na_boolean: {}\nfirst_letter_of_name: {}\na_statement: {}\nb_statement: {}",
        age, height, a_boolean, first_letter_of_name, a_statement, b_statement
    );

    // ASSSIGNMENT:
    let new_person: String = String::from("Ezekiel");
    let new_age: u32 = 20;
    let new_height_meters: f64 = 1.85; // this is ~6.1.
    let likes_tacos: bool = true;
    let new_first_letter_of_name: char = 'E';
    println!(
        "new_person: {}\nnew_age: {}\nnew_height_meters: {}\nlikes_tacos: {}\nnew_first_letter_of_name: {}",
        new_person, new_age, new_height_meters, likes_tacos, new_first_letter_of_name
    );
}
