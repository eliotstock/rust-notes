// Rust notes made while reading "the book". Build and run with
// `cargo run`.
// See: https://doc.rust-lang.org/book

use std::any::type_name;

fn main() {
    // Forward declarations are fine.
    chapter03_section02();

    // Pass arguments to functions as expected.
    chapter03_section03(1);

    chapter03_section05();

    chapter04();

    chapter05();

    chapter06();
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

// Ownership
fn chapter04() {
    // A string literal is stored on the stack and is immutable.
    let stack_string = "foo";

    // A String instance is stored on the heap and can be mutable.
    let mut heap_string = String::from("bar");
    heap_string.push_str(" baz");

    println!("Strings: {}, {}", stack_string, heap_string);

    // At the end of the current block (function, in this case),
    // heap_string goes out of scope, Rust calls drop() on it, and
    // the heap memory is freed. We don't have to explicitly
    // deallocate it.

    // Another string instance has a copy of the stuff on the stack,
    // but points to the same memory on the heap.
    let heap_string2 = heap_string;

    // But Rust also invalidates heap_string to prevent a double free
    // error when both variables go out of scope.
    // heap_string has "moved" into heap_string2.
    println!("String: {}", heap_string2);

    // Using heap_string here would be an error ("value borroed here
    // after move")
    // println!("String: {}", heap_string);

    // Use clone() to make a copy of the heap data, not just the
    // stack data.
    let heap_string3 = heap_string2.clone();

    println!("String: {}", heap_string3);

    // If a type implements the Copy trait, the old variable is still
    // OK to use after the new one is assigned to it. You can't mix
    // the Copy and Drop traits.

    // Calling functions works like assignment.
    // Passing a String will take ownership. heap_string3 can't be
    // used after this.
    chapter04_takes_ownership(heap_string3);

    // Passing an integer makes a copy.
    let x = 1;
    chapter04_makes_copy(x);

    // It's fine to use x after this.
    println!("Integer: {}", x);

    // Functions move their ownership out when returning a value.
    let s = chapter04_gives_ownership();

    println!("String: {}", s);

    // Functions take ownership when we pass them Strings.
    let t = chapter04_takes_and_gives_back(s);

    println!("String: {}", t);

    // References, denoted with &, allow you to pass around a value
    // without transfering ownership of it.
    let l = chapter04_borrows(&t);

    println!("String length: {}", l);

    // Mutable references can have their values modified.
    let mut m = String::from("foo");

    chapter04_borrows_as_mutable(&mut m);

    // You can have only one mutable reference to a particular piece
    // of data in a particular scope. This allows Rust to prevent
    // data races at compile time. A data race could happen if:
    //   * Two pointers access the same data at the same time
    //   * At least one is being used to write
    //   * There's no synchronisation
    // To get a second mutable reference, just create a new scope.

    // Rust prevents dangling references, where the memory has been
    // freed but the pointer remains in use. If a variable goes out
    // of scope but the reference doesn't, you'll get an error.

    // At any given time, you can have either one mutable reference
    // or any number of immutable references.
    // References must always be valid.

    // Use the range operator to get a string slice from a string.
    // The start index is zero based and the end index is one after
    // the last included character.
    let s = String::from("hello world");
    let slice1 = &s[0..5];

    // You can omit either the start index if it's zero or the end
    // index if it's the end of the string.
    let slice2 = &s[..5];
    let slice3 = &s[6..];

    println!("Slices: {}, {}, {}", slice1, slice2, slice3);

    // The type of a slice is &str. String literals are actually
    // slices. We prefer to use &str in method signatures rather
    // than String.
}

fn chapter04_takes_ownership(s: String) {
    println!("{}", s);
    // Drop is called here.
}

fn chapter04_makes_copy(i: i32) {
    println!("{}", i);
}

fn chapter04_gives_ownership() -> String {
    let s = String::from("hello");
    // Returning s moves it to the calling function.
    s
}

fn chapter04_takes_and_gives_back(s: String) -> String {
    // Returning s moves it to the calling function.
    s
}

fn chapter04_borrows(s: &String) -> usize {
    // We can't modify a borrowed value. This would give an error:
    // s.push_str(" bar");

    s.len()
    // s is not dropped here, because we're only passed a reference.
}

fn chapter04_borrows_as_mutable(s: &mut String) {
    // We can modify the value for a mutable reference.
    s.push_str(" bar");
}

// Structs and methods

// Structs have fields and are defined like this (nevermind the
// annotation for now).
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs have a name for the type, but not for each field.
// Two tuple structs with all the same fields types are not
// interchangeable - they're different types.
struct Color(i32, i32, i32);

// Unit-like structs don't have any fields and behave similarly to
// (), the unit type.
struct UnitLike();

// Methods are like functions, but within the scope of a struct, enum
// or trait and they take &self as the first parameter. They go in an
// implementation block, not the struct definition itself. You can
// have mulitple impl blocks for a given struct.

impl User {
    // Methods can:
    //   1. take ownership of self: self
    //   2. borrow self immutably: &self
    //   3. borrow self mutably: &mut self
    fn sign_out(&self) {
        println!("Signed out: {}", self.username);
    }

    // Associated functions are like static methods in Java - you
    // don't need an instance to call them on.
    fn foo() {
        println!("Associated function invoked.");
    }
}

fn chapter05() {
    // Create an instance of a struct like this.
    let u = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Use the {:?} specifier to call the Debug output format.
    println!("Struct instance: {:?}", u);

    // Call methods as expected. There's no need for the -> like in
    // C or C++ because Rust does automatic referencing and
    // dereferencing.
    u.sign_out();

    // Call associated functions using ::.
    User::foo();

    // The entire instance is mutable or not - you can't mark
    // individual fields as mutable.

    let u2 = chapter05_build_user(String::from("a@b.net"),
        String::from("foo"));

    println!("Struct instance field: {}", u2.email);

    // Use struct update syntax to create a new instance that is
    // mostly the same as another instance.
    let u3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..u2
    };

    println!("Struct instance field: {}", u3.email);

    // Instatiate a tuple struct like this.
    let t = Color(0, 0, 0);

    println!("Tuple struct instance field: {}", t.0);

    // Unit-like struct usage to prevent compilation warnings only.
    println!("Tuple struct instance field: {}",
        type_name::<UnitLike>());
}

// Field init shorthand syntax. No need for "email: email".
fn chapter05_build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Enums and pattern matching

fn chapter06() {
    // Simple enum:
    #[derive(Debug)]
    enum Operation {
        Set,
        Check,
    }

    // Enums can have associated values. Each value can take
    // different types.
    #[derive(Debug)]
    enum OperationWithLabel {
        Set(String),
        Check(u32),
    }

    // Enums can have methods, too.
    impl Operation {
        fn execute(&self) {
            println!("Operation executed.");
        }
    }

    // Use of a simple enum variant:
    let op1 = Operation::Set;
    let op2 = Operation::Check;
    println!("Enum variants: {:?}, {:?}", op1, op2);

    // Use of an enum variant that takes an associated value:
    let op3 = OperationWithLabel::Set(String::from("set"));
    let op4 = OperationWithLabel::Check(100);
    println!("Enum variants: {:?}, {:?}", op3, op4);

    // Calling a method on an enum variant:
    op1.execute();

    // The Option enum in the standard library is so common that you
    // don't need to bring it into scope. Its variants are Some and
    // None. With None, we need to specify the type of which we have
    // no value.
    let a = Some(1);
    let b: Option<u32> = None;
    println!("Some and none: {:?}, {:?}", a, b);

    // The patterns for each arm of a "match" expression can be any
    // type, whereas an "if" needs bools. Only the first arm to match
    // will be executed.
    match op1 {
        Operation::Set => {
            println!("set");
        },
        Operation::Check => {
            println!("check");
        }
    }

    // Patterns can also bind to values, so that we can use the value
    // in the arm. Matches are exhaustive: we must cover every
    // possible pattern.
    let c: Option<u32> = match a {
        None => None,
        Some(i) => Some(i + 1),
    };

    println!("c is some: {}", c.is_some());

    // The _ placeholder will match all values not explicitly
    // specified before it.
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // The "unit value".
    }

    // If we only care about one of the cases, use "if let".
    if let Some(2) = c {
        println!("c is 2");
    }
}
