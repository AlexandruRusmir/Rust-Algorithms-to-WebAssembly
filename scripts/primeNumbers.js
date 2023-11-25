import init, { generate_primes } from '../pkg/wasm_project.js';

const setExampleLimit = () => {
    document.getElementById('inputLimit').value = "100";
}

const generatePrimeNumbers = () => {
    let limit = parseInt(document.getElementById('inputLimit').value, 10);
    if (!isNaN(limit)) {
        let primes = generate_primes(limit);
        document.getElementById('primeOutput').textContent = primes.join(', ');
    } else {
        document.getElementById('primeOutput').textContent = "Invalid limit.";
    }
}

document.getElementById('exampleButton').addEventListener('click', setExampleLimit);
document.getElementById('generateButton').addEventListener('click', generatePrimeNumbers);

const initWasm = async () => {
    await init();
}

initWasm();
