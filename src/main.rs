// Rust notes made while reading "the book". Build and run with
// `cargo run`.
// See: https://doc.rust-lang.org/book

fn main() {
    // Forward declarations are fine.
    chapter03_section02();

    // Pass arguments to functions as expected.
    chapter03_section03(1);

    chapter03_section05();
}

// Variables and mutability
fn chapter03_section02() {
    // Variables are immutable by default. To declare a mutable one:
    let mut x = 5;
    x += 1;

    println!("Variables: {}, ", x);

    // Constants can only be set to a constant experssion, not to the
    // result of a function call.
    const CONST_THING: u32 = 10_000;

    // An existing variable can be shadowed by redeclaring it with
    // let. The type can change.
    let x = "six";

    println!("Variables: {}, {}", x, CONST_THING);

    // Data types

    // Type annotations are only required in cases where many types
    // are possible.

    // Scalar types
    //   Integers
    //     i8, u8
    //     i16, u16
    //     i32, u32
    //     i64, u64
    //     isize, usize (size of these depends on the machine
    //       architecture: 32 or 64 bit). Use these when indexing a
    //       collection.
    //     Signed integers are stored using two's complement
    //       representation.
    //     Overflowing an integer variable will panic in debug mode
    //       only. In release mode, they just wrap.
    //       x: u8 = 255;
    //       x++; [panic!]
    //     i32 is default
    //     Types can go after the literal like this
    let i = 127i8;
    //   Floating-point numbers
    //     f32, f64
    //     Stored using IEEE-754 representation
    //     f64 is default
    //     Literals just have a decimal point in them
    let f = 0.1;
    //   Booleans
    //     bool
    //     values: true, false
    let b = false;
    //   Characters
    //     4 bytes, even for ASCII
    //     Use single quotes for literals
    let c = 'z';

    // Compound types
    //   Tuple
    let t: (i32, bool) = (1, false);
    //     Destructure a tuple value like this.
    let (u, v) = t;
    //     Access a tuple's elements like this.
    let y = t.0;
    //     The concept of no value at all is expressed as an empty
    //       tuple: ().
    //   Array
    //     Fixed length.
    //     Allocated on the stack.
    //     Prefer std::vec::Vec in most cases.
    let a = [1, 2, 3, 4, 5];
    //     The type and size aren't required.
    let a_prime: [i32;5] = [1, 2, 3, 4, 5];
    //     Initialise an array that contains the same value for each
    //       element:
    let d = [3; 5];
    //     Indexing:
    let e = a[0];
    //     Index out of bounds errors are caught at compile time if
    //       they can be, or runtime with a panic if not.

    println!("Variables: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        i, f, b, c, u, v, y, a_prime[4], d[4], e);
}

// Functions
fn chapter03_section03(parameter: i32) -> i32 {
    // Types of function parameters must be declared.
    println!("Parameter: {}", parameter);

    // Statements are instructions that perform some action and do
    // not return a value.
    let a = 0;

    // Expressions evaluate to a resulting value.
    let b = {
        let a = 1;
        // Note the lack of a trailing semicolon here. Expressions
        // don't have them. Adding one would turn this into a
        // statement.
        a + 1
    };

    println!("Variable: {}", a);

    // Functions return the last expression implicitly, but they can
    // also use the "return" keyword to return early.
    b
}

// Control flow
fn chapter03_section05() {
    // Conditions in "if" statements don't have brackets. The
    // expression must be a bool.
    let mut x = 1;
    if x > 0 {
        // True "arm"
        println!("x > 0");
    } else {
        // Else "arm"
        println!("x !> 0");
    }

    // Prefer to use the "match" construct rather than an "if"
    // with many arms.

    // "if" is an expression, so it can be used with "let". Both
    // blocks must evaluate to the same type.
    let five = if x == 5 { "five" } else { "not five" };

    println!("five: {}", five);

    // There is no "do...while" in Rust, but there is "loop".
    loop {
        if x > 0 {
            break;
        }
    }

    // "loop" is also an expression. "break" can take a value to
    // return out of the loop.
    let y = loop {
        x += 1;

        if x == 2 {
            break x;
        }
    };

    println!("y: {}", y);

    // "while" works as expected.
    while x < 3 {
        x += 1;
    }

    // "for" loops use an "in" keyword.
    let s = ["one", "two", "three"];

    for e in s.iter() {
        println!("Element: {}", e);
    }

    // Using a "for" loop with a Range is idiomatic.
    for n in (1..10).rev() {
        println!("{}", n);
    }
}
