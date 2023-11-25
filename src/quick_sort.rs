use wasm_bindgen::prelude::*;

/*
* Quick Sort
*/
#[wasm_bindgen]
pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    if len < 2 {
        return arr;
    }

    let pivot_index = partition(&mut arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    let (pivot, right) = right.split_first_mut().unwrap();

    let sorted_left = quick_sort(left.to_vec());
    let sorted_right = quick_sort(right.to_vec());

    [sorted_left, vec![*pivot], sorted_right].concat()
}

#[wasm_bindgen]
pub fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;

    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}