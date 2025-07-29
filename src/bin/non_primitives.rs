
fn main() {
    // 🟨 DEEP DIVE: ARRAYS, TUPLES, SLICES
    arrays();
    tuples();
}

fn arrays() {
    println!("\narrays()");
    
    /*
        Arrays: Basics
            - Fixed size known at compile-time
            - Same data type for all elements
            - Stored contiguously in memory (fast indexing)
    */

    let arr = [10, 20, 30, 40, 50];
    println!("arr: {}", arr[1]);

    let arr_with_type: [i32; 3] = [1, 2, 3];
    println!("arr_with_type: {}", arr_with_type[1]);

    let arr_repeated = [0; 5]; // [0, 0, 0, 0, 0];
    println!("arr_repeated: {}", arr_repeated[1]);

    /*
        ACCESS:
            .len() gives length
            Index out of range → runtime panic (safe)
    */
    
    let first = arr[0];
    println!("first: {first}");

    let last = arr[arr.len() - 1];
    println!("last: {last}");

    /*
        ⚡ Comparison with Java
            - Rust array = int[] in Java
            - BUT Rust array size is part of the type: [i32; 5]
            - Java arrays are heap-allocated objects; Rust arrays can live on the stack → faster for small fixed data
    */

}

fn tuples() {
    println!("\ntuples()");
    
    /*
        Tuples: Basics
            - Can store different types
            - Size is fixed and known at compile-time
    */

    let tup: (i32, f64, char) = (42, 6.9, 'R');

    //ACCESS:
    let (x, y, z) = tup; // destructuring
    print!("x: {x}, y: {y}, z: {z}");
    println!("{}", tup.0); // direct indexing

    /*
        ✅ Use Cases
                - Lightweight grouping without a struct
                - Intermediate data packing (common in iterators)
                - Returning multiple values:
    */

    println!("calc.1: {}\ncalc.2: {}", calc(10, 20).0, calc(10, 20).1);
    
    fn calc(x: i32, y: i32) -> (i32, i32) {
        (x + y, x * y)
    }
    /*
        ⚡ Comparison with Java
            - Java lacked tuples until records (Java 14+) or needed custom classes
            - Rust tuples are first-class and used heavily
            - No overhead of object wrappers → zero-cost
    */
}

