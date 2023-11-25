import init, { quick_sort, generate_primes, compute_sha256, encode_huffman } from './pkg/wasm_project.js';

async function runWasm() {
    await init();

    document.getElementById('sortButton').addEventListener('click', () => {
        let input = document.getElementById('inputArray').value;
        let numbers = input.split(',').map(x => parseInt(x, 10));
        let sortedNumbers = quick_sort(numbers);
        document.getElementById('sortedArray').textContent = sortedNumbers.join(', ');
    });

    document.getElementById('primeButton').addEventListener('click', () => {
        let limit = parseInt(document.getElementById('primeLimit').value, 10);
        let primes = generate_primes(limit);
        document.getElementById('primeArray').textContent = primes.join(', ');
    });

    document.getElementById('hashButton').addEventListener('click', () => {
        let input = document.getElementById('hashInput').value;
        let hash = compute_sha256(input);
        document.getElementById('hashOutput').textContent = hash;
    });

    document.getElementById('encodeButton').addEventListener('click', () => {
        let input = document.getElementById('inputText').value;
        let encoded = encode_huffman(input);
        document.getElementById('encodedOutput').textContent = encoded;
    });
}

runWasm();
