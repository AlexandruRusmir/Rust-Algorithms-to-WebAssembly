import init, { encode_huffman } from '../pkg/wasm_project.js';

const setExampleText = () => {
    document.getElementById('inputText').value = "example text";
}

const encodeText = () => {
    let text = document.getElementById('inputText').value;
    let encoded = encode_huffman(text);
    document.getElementById('encodedOutput').textContent = encoded;
}

document.getElementById('exampleButton').addEventListener('click', setExampleText);
document.getElementById('encodeButton').addEventListener('click', encodeText);

const initWasm = async () => {
    await init();
}

initWasm();
