
/*
    Rust is a statically typed language — meaning every variable's type is known at compile time. This enables:
        -blazing-fast performance 🏎️
        -rock-solid safety 🛡️
        -powerful tooling (like autocompletion + inference)

    ✅ Categories of Data Types
        Rust's types fall broadly into two categories:
        
            Kind	  |   Examples
           -----------|--------------------------------------
            Scalar	  |   integer, floating-point, char, bool
            Compound  |	  tuple, array, slice, struct, enum
*/

fn main() {
    /*
        🔢 1. Scalar Types:-
            These are single-value types, similar to Java's primitives.

            🔸 Integer Types

            Type	      |      Signed	      |  Unsigned	         |  Size (bits)
            ---------------|-------------------|----------------------|--------
            i8	          |      ✔️	          |  ❌	                |  8
            u8	          |      ❌	         |  ✔️	                |  8
            i16 to i128	  |      ...	      |  ...	             |  ...
            isize / usize |	    pointer-sized |	 platform-dependent  |
    */
    integers();
    integer_overflow();
    floating_pointers();
    booleans();
    chars();

    // 🧳 2. Compound Types
    tuples();
    arrays();
    slices();
    /*
        🔍 Type Inference
        Rust is good at figuring types out for you:

        let x = 42;          // inferred as i32
        let pi = 3.14;       // inferred as f64
        let f = true         // inferred as bool
        let alphabet = 'M'   // inferred as char | 4-byte Unicode Scalar Value (char ≠ u8)
    
        But when needed (e.g., passing to generic functions or when ambiguity arises), you must annotate.
        
        -----------------------------------------------------------
        
        🔒 No Null:-
            - Rust has no null.
            - Instead, it uses:
            - Option<T> – for values that may or may not exist
            - Result<T, E> – for operations that may fail
            - We’ll go deep into those later — they're a big deal in Rust.

        -----------------------------------------------------------

        🧠 Deep Insight: Rust vs Java:-

            | Concept        | Rust                     | Java                             |
            | -------------- | ------------------------ | -------------------------------- |
            | Integer safety | Overflows checked/debug  | Silent overflows                 |
            | `null`         | Doesn't exist (`Option`) | NullPointerException 😨          |
            | char           | Unicode 4-byte scalar    | UTF-16 2-byte (not full Unicode) |
            | arrays         | Fixed-size `[T; N]`      | Object-type with `.length`       |
            | tuples         | Built-in, heterogenous   | Needs custom class / record      |
            --------------------------------------------------------------------------------

        -----------------------------------------------------------
    */

    challenge();
    summary_cheatsheet();
}


fn integers() {
    println!("\nintegers()");

    // Signed (i) = can be negative
    let x: i32 = -42;
    println!("value of x : {}", x);

    // Unsigned (u) = always positive
    let y: u64 = 999_999;
    println!("value of y : {}", y);

    // 💡 Rust won’t automatically cast types like Java does. You must do it explicitly.
    let a: u8 = 10;
    println!("value of a : {}", a);

    let b: u16 = a as u16; // ✅ explicit casting
    println!("value of b : {}", b);
}

fn integer_overflow() {
    println!("\ninteger_overflow()");

    /*
        ⚠️ Integer Overflow:-
            - Debug mode: panics on overflow
            - Release mode: wraps around (2’s complement)
    */
    
    let x: u8 = 255;
    println!("value of x : {x}");

    let y = x ;//+ 1; // ⚠️ panic in debug mode!
    println!("value of y : {y}");

    // Use wrapping_add, checked_add, etc. for control:
    let z = x.wrapping_add(1); // z = 0
    println!("value of z : {}", z);
}

fn floating_pointers() {
    println!("\nfloating_pointers()");

    /*
        Float:-
            - f64 is the default — higher precision
            - No implicit casting between f32 and f64
    */
    
    let x: f64 = 3.14; // default
    println!("value of x : {}", x);

    let y: f32 = 2.5;
    println!("value of y : {}", y);

    let z = y as f64;
    println!("value of z : {}", z);
}

fn booleans() {
    println!("\nbooleans()");

    /*
        bool:-
        NOTE:
            - No auto-conversion from int to bool like in C/C++
            - Must be true or false
    */

    let is_ready: bool = true;
    
    if is_ready {
        println!("Let's go!");
        let f = false;
        if !f {
            println!("f is {}", f);
        }
    }
}

fn chars() {
    println!("\ncharacters()");
    /*
        char:-
        - 4-byte Unicode Scalar Value (char ≠ u8)
        - Supports Unicode: accents, emojis, scripts
        - ✅ In contrast to Java's 2-byte UTF-16 char, Rust chars are fully Unicode.
    */
    let heart: char = '♥';
    println!("value of heart : {}", heart);
    
    let letter = 'A';
    println!("value of letter : {}", letter);

    let emoji = '🦀';
    println!("value of emoji : {}", emoji);
}

fn tuples() {
    println!("\ntuples()");
    
    /*
        Tuples:- A fixed-size collection of heterogeneous values.
            - Useful for grouping values
            - Returned from functions frequently
    */

    let tup: (i32, f64, char) = (500, 6.4, 'R');
    // println!("value of tup: {}", tup); // Will not work
    
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}"); // Will not work
    
    // For direct access:
    println!("value of 1: {}", tup.1);
}

fn arrays() {
    println!("\narrays()");
    
    /*
        Arrays:- Fixed-size, same-type collection.
            - Length is part of the type: [i32; 5]
            - Out-of-bounds access = compile-time panic (safe!)
    */

    let arr: [i64; 5] = [1, 2, 3, 4, 5];
    println!("value of 3: {}", arr[2]);

    let ans = arr[4];
    println!("value of ans: {ans}");

    // let out = arr[99]; // ❌ panics at runtime
}

fn slices() {
    /*
        Slices (&[T]):- Dynamically sized view into a collection (part of an array)
            - Safe, efficient, passed as references
            - Used heavily in string and collection manipulation
    */

    let arr = [1, 2, 3, 4, 5];
    println!("arr[0]: {}", arr[0]);
    println!("arr[1]: {}", arr[1]);
    println!("arr[2]: {}\n", arr[2]);

    let slice = &arr[1..3]; // [2, 3]
    println!("slice[0]: {}", slice[0]);
    println!("slice[1]: {}", slice[1]);
}

fn challenge() {
    let tup: (i32, i32, i32) = (10, 28, 36);
    println!("tup.0: {}", tup.0);
    println!("tup.1: {}", tup.1);
    println!("tup.2: {}", tup.2);

    let avg = (tup.0 + tup.1 + tup.2) as f64 / 3.0;
    println!("avg: {avg}");
}

fn summary_cheatsheet() {
    // Scalar
    let _a: i32 = -100;
    let _b: f64 = 3.14;
    let _c: bool = true;
    let _d: char = '🦀';

    // Compound
    let _tup: (i32, f64, char) = (1, 2.0, 'c');
    let ar: [i32; 3] = [1, 2, 3];
    let _slice = &ar[1..]; // dynamically sized
}
