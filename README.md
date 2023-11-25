Rust Algorithms in WebAssembly
==============================

This demonstrates the implementation of algorithms in Rust, compiled to WebAssembly (WASM) for execution in web browsers. It showcases the power and versatility of Rust and WASM, especially for executing computationally intensive tasks efficiently in a web environment.

Implemented Algorithms
----------------------

### 1. Quick Sort

* **Description**: A sorting algorithm that follows the divide and conquer paradigm. It picks an element as a pivot and partitions the given array around the picked pivot.
* **Rust Implementation**: Implemented in Rust, the quick sort algorithm sorts an array of integers. The implementation showcases Rust's array manipulation capabilities.

### 2. Prime Number Generation

* **Description**: Generates all prime numbers up to a specified limit.
* **Rust Implementation**: This algorithm uses simple iterative methods to find all prime numbers up to a given limit, demonstrating basic iteration and conditional logic in Rust.

### 3. Huffman Encoding

* **Description**: A compression algorithm that uses variable-length codes to encode characters. The frequency of each character is used to build a binary tree where each character is represented by a unique binary string.
* **Rust Implementation**: The Huffman encoding algorithm in Rust builds a binary tree based on character frequencies and encodes a text input into a binary string.

### 4. SHA-256 Hashing

* **Description**: A cryptographic hash function widely used for data integrity verification. It generates a fixed-size string (hash) which is unique for unique input data.
* **Rust Implementation**: Utilizes the `sha2` Rust crate to implement SHA-256, showcasing how external libraries can be used in Rust for specialized tasks, and therefore used in a web browser in case they are not already implemented in JavaScript.

WebAssembly Integration
-----------------------

Each of these algorithms is compiled to WASM using `wasm-pack`, allowing them to run in a web browser. This process involves writing Rust code for the algorithms, annotating them with `#[wasm_bindgen]` to expose them to JavaScript, and then using `wasm-pack` to compile the Rust code to WASM.

### Running the Algorithms

To run these algorithms in a web browser:

1. Compile the Rust project to WASM using `wasm-pack build --target web`.
2. Serve the generated files using a web server.
3. Access the web page and interact with the algorithms through a user-friendly interface.

Conclusion
----------

This project serves as a practical example of how Rust, combined with WebAssembly, can be used to implement and execute various algorithms efficiently in a web environment. It also demonstrates the integration of Rust crates in a WASM project, expanding the capabilities and potential of web applications.

* * *
