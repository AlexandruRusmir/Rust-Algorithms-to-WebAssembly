import init, { compute_sha256 } from '../pkg/wasm_project.js';

const setExampleText = () => {
    document.getElementById('inputText').value = "Hello, world!";
}

const computeHash = () => {
    let text = document.getElementById('inputText').value;
    let hash = compute_sha256(text);
    document.getElementById('hashOutput').textContent = hash;
}

document.getElementById('exampleButton').addEventListener('click', setExampleText);
document.getElementById('hashButton').addEventListener('click', computeHash);

const initWasm = async () => {
    await init();
}

initWasm();
