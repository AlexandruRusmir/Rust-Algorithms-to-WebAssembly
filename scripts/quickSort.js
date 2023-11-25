import init, { quick_sort } from '../pkg/wasm_project.js';

const setExampleInput = () => {
    document.getElementById('inputText').value = "3, 1, 4, 1, 5, 9, 2, 6";
}

const performQuickSort = () => {
    let input = document.getElementById('inputText').value;
    let numbers = input.split(',').map(x => parseInt(x, 10)).filter(x => !isNaN(x));
    let sortedNumbers = quick_sort(numbers);
    document.getElementById('sortedArray').textContent = sortedNumbers.join(', ');
}

document.getElementById('exampleButton').addEventListener('click', setExampleInput);
document.getElementById('sortButton').addEventListener('click', performQuickSort);

const initWasm = async () => {
    await init();
}

initWasm();
