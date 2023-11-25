mod quick_sort;
mod generate_primes;
mod compute_sha256;
mod huffman;

pub use huffman::encode_huffman;
pub use quick_sort::{quick_sort, partition};
pub use generate_primes::{generate_primes, is_prime};
pub use compute_sha256::{compute_sha256};