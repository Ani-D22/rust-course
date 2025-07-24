fn main() {
    variable();
    mutability();
    explicit_typing();
    shadowing1();
    shadowing2();
    shadowing3();
    constant();
    variable_scope();
    challenge();
}

/*
    📌 Quick Reference Cheatsheet

        Feature	Rust	|    Java Analogy (brief)
        Immutable var	|   let x = 5;	final int x = 5; (but default!)
        Mutable var	    |   let mut x = 5;	Normal variable
        Shadowing	    |   let x = ...; let x = ...;	Redeclaring variable name
        Constant	    |   const MAX: i32 = 100;	static final int MAX = 100;
        Type hint	    |   let x: u32 = 42;	Type declared
        Type inferred	|   let x = 42;	Java with var x = 42; (Java 10+)
*/

fn variable() {
    let x = "Hello"; // let is used for immutable variables;
    println!("Value of x is: {}", x);

    // NOTE: `println` is a macro here, which is why we use `!` between `println` and `()`
}

fn mutability() {
    /* 
        let x = 6; ❌ Error: cannot assign twice to immutable variable;
        Use `let mut`;
    */

    let mut x = "Hello";
    println!("Before changing: Value of x is: {}", x);

    x = "6";
    println!("After changing: Value of x is: {}", x);
    
    /*
         Why not default to mutable?
            - Makes reasoning about state easier
            - Encourages functional, side-effect-free programming
            - Reduces data races (especially when you go multithreaded)
    */
}

fn explicit_typing() {
    /*
        Instead of using implicit - inference:
    */

    // Use `u` for undefined integer
    // Use `i` for integer
    let score: u32 = 65538;
    println!("Score = {}",score);

    // Use `f` for float
    let marks: f32 = 23.123;
    println!("Marks = {}",marks);

    /*
        This is useful when:
            - You need to control memory layout
            - You're working with generic APIs
            - You’re interfacing with external data formats
    */
}

fn shadowing1() {
    /*
        Shadowing vs mut:
            - Shadowing: creates a new binding over the previous one
            - mut: changes the value in-place
    */
    
    let x = 5;
    println!("1. x is: {}", x);
    let x = x + 1;
    println!("2. Now x is: {}", x);
    let x = x * 2;
    println!("3. Now x is: {}", x);

    /*
        Why is this useful?
            - Allows transformation of types:
    */

    let spaces = "   ";
    println!("spaces: {}", spaces); // prints nothing
    let spaces = spaces.len(); // allowed: old `spaces` (str), new `spaces` (usize)
    println!("spaces: {}", spaces); // prints: 3
}

fn shadowing2() {
    let num = "42"; // &str
    let num = num.trim(); // &str (no leading/trailing spaces)
    let num = num.parse::<i32>().unwrap(); // i32

    println!("Parsed number: {}", num);

    /*
        💬 TL;DR
        Use Case	Use mut	Use shadowing (let x = x)
        Reassign same type	✅	✅ (but not necessary)
        Change type	❌ (compile error)	✅ Shadowing allows type change
        Rebinding in block	❌	✅ Cleaner and scoped
        Transformations	❌ (unless same type)	✅ Especially for type changes
    */
}

fn shadowing3() {
    let input = "123";               // &str
    println!("input : {}", input);
    let input = input.trim();        // &str (trimmed)
    println!("input : {}", input);
    let input = input.parse::<i32>().unwrap(); // i32
    println!("input : {}", input);
    /*
        Here you reuse the same variable name, changing both value and type at each step. This is intentional:
            - Cleaner pipelines
            - Avoids name pollution (input, input_trimmed, parsed_input, finalInput, etc.)
            - Keeps the previous bindings inaccessible after shadowing
    */
}
fn constant() {
    /*
        Defined using const, must have type annotations and be set at compile-time:
        
        Global scope allowed:
            - Naming convention: SCREAMING_SNAKE_CASE
            - Cannot use result of runtime computation
    */

    const PI: f64 = 3.1415;
    println!("PI: {}", PI);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);
}

fn variable_scope() {
    /*
        ✅ Clear and predictable.
        ❌ No surprise mutations from nested scopes (a common bug source in other languages).
    */

    let n = 10;
    {
        let n = 20;
        println!("inside scope: {}", n);
    }
    println!("outside scope: {}", n);
}

fn challenge() {
    let mut counter = 0;
    println!("counter: {}", counter);
    counter += 1;
    println!("counter: {}", counter);
    counter += 1;
    println!("counter: {}", counter);
    counter += 1;
    println!("counter: {}", counter);

    let counter = "Demo counter";
    println!("counter string: {}", counter);
}
