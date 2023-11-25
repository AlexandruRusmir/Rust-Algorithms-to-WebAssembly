use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};

/*
* SHA256
*/
#[wasm_bindgen]
pub fn compute_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}