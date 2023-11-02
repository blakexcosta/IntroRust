fn main() {
    // PART 5 CONTROL FLOW

    // ----------------------------------------
    // IF
    let number = 1000;
    if number > 100 {
        println!("Number is greater than 100");
    } else {
        println!("Number is not greater than 100");
    }
    // else if
    let num = -50;
    if num > 0 {
        println!("Number is positive");
    } else if num < 0 {
        println!("Number is negative");
    } else {
        println!("Number is zero");
    }
    // ----------------------------------------
    // LOOPS

    //while loop
    //declare counter variable
    let mut counter = 1;
    //set our looping condition
    while counter < 6 {
        println!("{}", counter);
        // increment counter for condtion
        counter += 1;
    }

    // for loops
    // inclusive range
    for number in 1..=5 {
        println!("for loop inclusive: {}", number);
    }
    // exclusive range
    for number in 1..5 {
        println!("for loop exclusive: {}", number);
    }
    // looping over a vector
    let numbers = vec![1, 2, 3, 4, 5];
    for number in &numbers {
        println!("{}", number);
    }

    // ----------------------------------------
    // ASSSIGNMENT:
    // Quiz:
    // 1. is `..=` inclusive or exclusive when in a for loop?
    // 2. What will the last println! of the following code be?
    // for number in 1..12 {
    //     println!("{}", number);
    // }

    // Practical
    // 1. Iterate over this vector and print out the values:
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in &names {
        println!("{}", name);
    }

    //2. Use a while loop to print the values 0 to 100 and add each element to a vector
    // HINT: Use type annotations when declaring a new vector (in the format of Vec<T>)
    let mut counter = 0;
    let mut new_vec: Vec<i32> = Vec::new();
    //set our looping condition
    while counter < 101 {
        // add element to vector
        new_vec.push(counter);
        // increment counter for condtion
        counter += 1;
    }

    //3. use an if statement to check whether the altitude is > 5000
    let altitude = 6000;
    if altitude > 5000 {
        println!("Altitude is greater than 5000");
    } else {
        println!("Altitude is less than 5000");
    }
}
