fn main() {
    // Scalar Types
    // Integer Types

    // Length	Signed	Unsigned
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize

    // Signed and unsigned refer to whether it’s possible for the number to be negative—in other words,

    // Signed >> can be negative
    // Unsigned >> cannot be negative

    // Signed between -2^(n-1) and 2^(n-1) - 1
    // Unsigned between 0 and 2^n - 1

    // Integer Literals
    // Number literals	Example
    // Decimal	98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'

    // Floating-Point Types
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient_32: f32 = 56.7 / 32.2;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1 (Round to the closest integer)

    println!("The value of quotient f32 is: {quotient_32}");
    println!("The value of quotient f64 is: {quotient}");
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    // Character Type - 4 bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("the value of heart_eyed_cat is: {heart_eyed_cat}");

    // Compound Types
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // Tuple Type

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Accessing tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // Array Type

    // Arrays in Rust have a fixed length, like tuples.
    // Unlike a tuple, every element of an array must have the same type.

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // create an array of 5 elements that all have the value 3
    let a: [i32; 5] = [3; 5];

    // Accessing array elements
    let first = a[0];
    let second = a[1];

    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
}
