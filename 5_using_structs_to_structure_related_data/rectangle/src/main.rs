#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    // Here we borrow the self instance we do not take ownership of it, even though we could (in rare case)
    // We only read here so we borrow immutability
    // In case we would like to write we would borrow mutability
    fn area(&self) -> u32 {
        // &self is short for self: &Self
        self.height * self.width
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height > rect.height
    }

}

// Multiple impl block for a struct is a valid syntax
impl Rectangle {
    // Associated functions are often used as constructors of a struct
    // Compared to methods associated functions do not have self as a parameter
    
    fn square (size: u32) -> Self {
        Rectangle {
            height: size,
            width: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 500,
        width: 730,
    };

    println!("The area of the rectangle is: {}", rect.area());
    println!("The area of the rectangle is: {}", area_fn(&rect));

    println!("Width of rect is: {}", rect.width); // Would not compile if we would Rectangle and not Reference rect in area_fn

    dbg!(&rect); // Here we need to pass the reference to dbg! as the macro is taking ownership of the variable

    dbg!(30 * 60);

    println!("Here is the rectangle: {:?}", rect); // Not fancy
    println!("Here is the rectangle: {:#?}", rect); // Fancy

    let small_rect = Rectangle {
        height: 400,
        width: 200,
    };

    let bigger_rect = Rectangle {
        height: 800,
        width: 900,
    };

    println!("Hey I'm a width getter for bigger_rect: {}", bigger_rect.width());
    
    println!("Smaller rect can be contained in rect ? {}", rect.can_hold(&small_rect));
    println!("Bigger rect can be contained in rect ? {}", rect.can_hold(&bigger_rect));

    let square = Rectangle::square(32);

    dbg!(&square);
}

fn area_fn(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
