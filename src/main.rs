// PART 6 FUNCTIONS - FINAL

// ----------------------------------------

// function that returns nothing
fn return_nothing() -> () {
    println!("Do stuff");
}

// function that returns an I32
fn return_i32() -> i32 {
    println!("Do stuff");
    return 32;
}

// function that uses arguments and an alternative way to return values
fn sum_no_return(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// function that returns both a value and has arguments
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

// public function that  uses both arguments and return values
pub fn sum_pub(a: i32, b: i32) -> i32 {
    return a + b;
}
// ----------------------------------------

fn print_my_name(name: String) {
    println!("{}", name);
}

fn sum_f64(a: f64, b: f64) -> f64 {
    return a + b;
}

fn main() {
    // calling function that returns nothing
    return_nothing();

    // calling function that returns an i32
    let new_i32 = return_i32();
    println!("{}", new_i32);

    // calling function that uses arguments and an alternative way to return values
    sum_no_return(1, 2);

    // calling function that returns both a value and has arguments
    let sum_result = sum(50, 25);
    println!("{}", sum_result);

    // calling public function
    let sum_result = sum_pub(100, 100);
    println!("{}", sum_result);

    // ASSSIGNMENT:
    // 1. Create a function that prints your name

    // 2. Call the new name function in main()
    print_my_name("Cory".to_string());

    // 3. create a function that adds two f64's together and returns a f64

    // 4. Call the new f64 function in main(), store the result in a variable, and print that variable to the console
    let float_sum = sum_f64(5.0, 2.5);
    println!("{}", float_sum);
}
