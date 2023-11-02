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

    //2. Use a while loop to print the values 0 to 100 and add each element to a vector
    // HINT: Use type annotations when declaring a new vector (in the format of Vec<T>)

    //3. use an if statement to check whether the altitude is > 5000
    let altitude = 6000;
}
