mod math_operations;

use math_operations::addition;
use math_operations::division;
use math_operations::multiplication;
use math_operations::subtraction;
fn main() {
    
    let int_sum = addition::f_add_integers(5, 3);
    println!("The sum of integers is: {}", int_sum);

    let float_sum = addition::f_add_floats(5.5, 3.3);
    println!("The sum of floats is: {}", float_sum);

    let int_diff = subtraction::f_subtract_integers(5, 3);
    println!("The difference of integers is: {}", int_diff);

    let float_diff = subtraction::f_subtract_floats(5.5, 3.3);
    println!("The difference of floats is: {}", float_diff);

    let int_product = multiplication::f_multiply_integers(5, 3);
    println!("The product of integers is: {}", int_product);

    let float_product = multiplication::f_multiply_floats(5.5, 3.3);
    println!("The product of floats is: {}", float_product);

    let int_quotient = division::f_divide_integers(5, 3);
    println!("The quotient of integers is: {}", int_quotient);

    let float_quotient = division::f_divide_floats(5.5, 3.3);
    println!("The quotient of floats is: {}", float_quotient);

}
