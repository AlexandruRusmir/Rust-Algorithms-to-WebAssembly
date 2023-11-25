use wasm_bindgen::prelude::*;

/*
* Prime Numbers Generation
*/
#[wasm_bindgen]
pub fn generate_primes(limit: usize) -> Vec<u32> {
    let mut primes = vec![];
    for number in 2..=limit {
        if is_prime(number) {
            primes.push(number as u32);
        }
    }
    primes
}

#[wasm_bindgen]
pub fn is_prime(number: usize) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as usize) {
        if number % i == 0 {
            return false;
        }
    }
    true
}