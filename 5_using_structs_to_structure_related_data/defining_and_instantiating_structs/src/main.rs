struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs

struct Color(u32, u32, u32);
struct Point(u32, u32, u32);

// Unit like Structs

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("someeonelse@example.com");

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    // BE CAREFUL HERE:
    // we used = meaning that username has been moved from user1 to user2 which means that user1 is no lonfer usable
    // println!("user1: {user1}");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(u, v, w) = black;
    let Point(x, y, z) = origin;

    let x_equal_u: bool = x == u;

    println!("is x = u ?  {x_equal_u}");

    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand syntax
        email,    // field init shorthand syntax
        sign_in_count: 1,
    }
}
