fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');
    defined_expression();
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn defined_expression() -> i32 {
    let y = {
        let x = 3; // statement
        x + 1 // expression - NO SEMICOLON
    };

    println!("The value of y is: {y}");
    // What's in the block is an expression
    y
}
