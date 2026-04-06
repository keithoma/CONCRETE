//! Example usage of the CONCRETE algebra library's GCD utilities.

use algebra_concrete::number_theory::{
    EuclideanIterative, EuclideanRecursive, EuclideanSubtraction,
    SteinIterative, SteinRecursive, RationalOps
};

fn main() {
    println!("--- CONCRETE Math Library: GCD Demonstration ---\n");

    // -------------------------------------------------------------------------
    // 1. The Ergonomic Default
    // -------------------------------------------------------------------------
    let a = 48u32;
    let b = 18u32;
    
    // Because of our macro setup, this completely abstracts the algorithm away.
    // At compile-time, this maps directly to `SteinIterative::compute`.
    let result_default = a.gcd(b);
    println!("Default GCD of {a} and {b}: {result_default}");

    // -------------------------------------------------------------------------
    // 2. Explicit Strategy Injection (Zero-Cost Abstractions)
    // -------------------------------------------------------------------------
    println!("\nExplicit Strategies:");
    
    let result_euclidean_iteration = a.gcd_with::<EuclideanIterative>(b);
    println!("  Using Euclidean Iterative: {result_euclidean_iteration}");

    let result_euclidean_subtraction = a.gcd_with::<EuclideanSubtraction>(b);
    println!("  Using Euclidean Sub:       {result_euclidean_subtraction}");

    let result_euclidean_recursive = a.gcd_with::<EuclideanRecursive>(b);
    println!("  Using Euclidean Recursive: {result_euclidean_recursive}");

    let result_stein_iterative = a.gcd_with::<SteinIterative>(b);
    println!("  Using Stein Iterative:     {result_stein_iterative}");

    let result_stein_recursive = a.gcd_with::<SteinRecursive>(b);
    println!("  Using Stein Recursive:     {result_stein_recursive}");


    // -------------------------------------------------------------------------
    // 3. Signed Integer Safety
    // -------------------------------------------------------------------------
    let x = -48i32;
    let y = 18i32;
    
    // Mathematically, the GCD of two integers is the positive generator of their ideal.
    // Notice how the compiler strictly knows this returns a `u32`!
    let result_signed: u32 = x.gcd(y); 
    println!("\nSigned Mathematics:");
    println!("GCD of {x} and {y}: {result_signed}");

    // Our rigorous absolute value handling protects against the MIN panic trap!
    let min_val = i8::MIN; // -128
    let other_val = 64i8;
    let result_min: u8 = min_val.gcd(other_val);
    println!("GCD of {min_val} (i8::MIN) and {other_val}: {result_min}");

    // -------------------------------------------------------------------------
    // 4. Edge Cases
    // -------------------------------------------------------------------------
    println!("\nEdge cases:");
    println!("GCD of 0 and 5: {}", 0u32.gcd(5u32));
    println!("GCD of 7 and 0: {}", 7u32.gcd(0u32));
    println!("GCD of 0 and 0: {}", 0u32.gcd(0u32));
}
