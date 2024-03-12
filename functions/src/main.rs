fn main() {
    println!("Hi there, ");

    a_called_function(); // we just call this function without any parameters

    another_function(5, 6); // we call this function with two parameters

    print_label_measurement(5, 'h');
}

fn a_called_function() {
    println!("I'm a_called_function!");
}

// in function signatures, we MUST define the type of each parameter value
fn another_function(x: i32, y: i32) {
    println!("The value of x: {} and the value of y: {}", x, y);
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}
