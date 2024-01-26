fn main() {

    // immutable variable vs mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let shadowed = 5;

    let shadowed = shadowed + 1;

    {
        let shadowed = shadowed * 2;
        println!("The value of shadowed in the inner scope is: {shadowed}");
    }

    println!("The value of shadowed is: {shadowed}");

    // mut vs shadowing
    // mut: the variable can be reassigned
    // shadowing: the variable can be redefined with a new type
    // shadowing is useful when you want to change the type of a variable
    // but reuse the same name - no need to come up with different names for the same value

    // Shadowing vs mut examples

    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();

    // -> no compile error

    // Mut
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // -> compile error
}
