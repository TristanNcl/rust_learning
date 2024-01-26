fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // change(&s1); That Does not compile
    main_mut_ref();
    main_comb_mut_no_mut_ref();
    main_comb_mut_no_mut_ref_nopb();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // By passing a reference the value is not dropped when the function ends

// fn change(some_string: &String) {   DOES NOT COMPILE
//     some_string.push_str(", world");
// }

fn main_mut_ref() {
    let mut s1 = String::from("hello");

    change(&mut s1);
    println!("The new string is: {s1}");

    // Note on mutable references
    // if you have a mutable reference to a value, you can have no other references to that value.

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // This will not compile
    // println!("{}, {}", r1, r2);

    // You can have multiple immutable references to a value, but not at the same time as a mutable reference
    // Because the mutable reference to a value will be dropped when it goes out of scope, the immutable references will be valid again
    // Therefore this will compile:

    {
        let r1 = &mut s1;
        println!("From r1 scope: {}", r1);
    }

    let r2 = &mut s1;
    println!("After the scope: {}", r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn main_comb_mut_no_mut_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM - cannot have a mutable reference while having immutable references
                 // Which makes sense because if you have a mutable reference, you could change the value of the immutable references

    println!("{}, {}", r1, r2);
}

fn main_comb_mut_no_mut_ref_nopb() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM - cannot have a mutable reference while having immutable references
                 // Which makes sense because if you have a mutable reference, you could change the value of the immutable references

    println!("{}, {}", r1, r2);
    let r3 = &mut s; // no problem
    r3.push_str(", world!");
    println!("{}", r3);
}

fn main_dangling_ref() {
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
//} The danger is that the value s will be dropped at the of the function and the reference wil be pointing to nothing

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // This is fine because the value is returned and moved out of the function so we move out the ownership
