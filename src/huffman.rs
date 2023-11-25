use std::collections::{BinaryHeap, HashMap};
use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq)]
struct Node {
    frequency: usize,
    value: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[wasm_bindgen]
pub fn encode_huffman(input: String) -> String {
    let mut freq_map = HashMap::new();
    for byte in input.bytes() {
        *freq_map.entry(byte).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (value, frequency) in freq_map {
        heap.push(Node {
            frequency,
            value: Some(value),
            left: None,
            right: None,
        });
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        heap.push(Node {
            frequency: left.frequency + right.frequency,
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
    }

    let root = heap.pop().unwrap();
    let mut code_map = HashMap::new();
    build_code_map(&root, &mut code_map, Vec::new());

    let mut encoded = String::new();
    for byte in input.bytes() {
        let code = &code_map[&byte];
        for &bit in code {
            encoded.push(if bit { '1' } else { '0' });
        }
    }

    encoded
}

fn build_code_map(node: &Node, code_map: &mut HashMap<u8, Vec<bool>>, mut prefix: Vec<bool>) {
    if let Some(value) = node.value {
        code_map.insert(value, prefix);
    } else {
        if let Some(ref left) = node.left {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false);
            build_code_map(left, code_map, left_prefix);
        }
        if let Some(ref right) = node.right {
            prefix.push(true);
            build_code_map(right, code_map, prefix);
        }
    }
}