fn main() {
    // s is not valid here, it's not yet declared
    let s = "hello"; // s is valid from this point forward

    {
        // s is valid here, it's declared above
        let s = "hello_2"; // s take another "hello" and shadow the previous one

        // do stuff with s
    } // this scope is now over, and s is no longer shadowed

    // String type

    {
        let snl = String::from("hello"); // snl is valid from this point forward
                                         // do stuff with snl
    } // this scope is now over, and snl is no longer valid

    // Move

    {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // To ensure memory safety rust considers s1 invlaid and doesn't need to free anything when s1 goes out of scope
    // this is called a move and ensure that rust will never automatically create "deep" copies of your data
    // as a result there is no runtime cost for this operation and not double free error

    // Because Rust also invalidates the first variable, instead of being called a shallow copy, it's known as a move

    // Clone

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // Ownership and Functions
    main_ownership_fn();

    // Return Values and Scope
    main_values_scope();
}

fn main_ownership_fn() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function and so is no longer valid here
                        // println!("{}", s); // this will cause an error

    let x = 5;

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and "drop" is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens

// Return Values and Scope

fn main_values_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2);
} // s1 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s3 goes out of scope and is dropped

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

// The Ownership Rules

// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

