use algebra_concrete::number_theory::{PrimeAlgorithm, TrialDivision};

fn main() {
    // Call via the struct, since it's an associated function
    let list_primes = TrialDivision::primes_up_to(10u32); 
    
    // Note: Vec doesn't implement Display by default, use {:?} for Debug
    println!("{:?}", list_primes);
}