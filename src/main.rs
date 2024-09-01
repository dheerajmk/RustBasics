// Function to add two integers
pub fn f_add_integers(a: i32, b: i32) -> i32 
{
    a + b
}


// Function to add two floating-point numbers
pub fn f_add_floats(a: f64, b: f64) -> f64 
{
    a + b
}

fn main() {
    println!("Hello, world!");

    let int_sum = f_add_integers(5, 3);
    println!("The sum of integers is: {}", int_sum);

    let float_sum = f_add_floats(5.5, 3.3);
    println!("The sum of floats is: {}", float_sum);
}
