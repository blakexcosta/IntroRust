fn main() {
    // PART 4 - MORE DATA TYPES
    // TUPLES
    let tuple = (1000, 59.69, "Bob".to_string());
    // accessing elements in tuple
    println!("tuple.0: {}", tuple.0);
    println!("tuple.2: {}", tuple.2);
    // destructuring a tuple
    let (a, b, c) = ("sally".to_string(), "bob".to_string(), "joe".to_string()); // unpacks tuple into variables
    println!("a: {}, b: {}, c: {}", a, b, c);

    // ARRAYS
    let menu_items = [
        "fish".to_string(),
        "chicken".to_string(),
        "tuna".to_string(),
    ];
    // arrays cannot have different types
    //let err_error = [1, 2, 3, "Bob".to_string()];
    // can access elements by index in arrays
    println!("new_array[0]: {}", menu_items[0]);
    println!("new_array[4]: {}", menu_items[2]);
    println!("new_array[0]: {}", menu_items[menu_items.len() - 1]);

    // VECTORS
    // create a vector with initial values
    let mut menu_vec = vec!["fish", "chicken", "tuna"];
    // add an element to the vector
    menu_vec.push("salmon");
    // access individual elements of the vector
    println!("First element: {}", menu_vec[0]);
    println!("Second element: {}", menu_vec[1]);
    println!("Second element: {}", menu_vec[menu_vec.len() - 1]);
    println!("menu_vec: {:?}", menu_vec);

    // remove an element from the vector
    menu_vec.pop(); // removes the last added element/last element
    menu_vec.remove(1); // zero based as well
                        // iterate over the vector again
    println!("menu_vec: {:?}", menu_vec);

    // ASSSIGNMENT:
}
