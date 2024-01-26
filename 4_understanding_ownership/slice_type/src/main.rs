fn main() {
    let s = String::from("Hello, world!");

    let word = first_word(&s);
    let word = first_word(&s[1..]);

    
    let word = first_word("Bonjour, je m'apelle Tristan!");

    // s.clear(); This is not working as clear uses a mutable reference but immutable reference is passed to word that is used in the println afterwards

    println!("The first word is: {word}");
    // let slice = &s[0..2];
    // let slice = &s[..2];

    // println!("slice: {slice}");
    
    // // Same the other way around
    // let len = s.len();
    
    // let slice = &s[2..];
    // let slice = &s[2..];
    // println!("slice: {slice}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
