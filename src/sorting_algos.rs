/*
    Selection Sort
    Bubble Sort
    insertion sort
    merge sort
    heap sort
    quick sort
*/

pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut _temp = 0;
    for _i in 0..arr.len() {
        let mut _swap = false;
        for j in 0..(arr.len() - 1) {
            if arr[j] > arr[j + 1] && j < arr.len() {
                _temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = _temp;
                _swap = true;
            }
        }
        if !_swap {
            break;
        }
    }
    arr

    // complexity = O(n**2)
}

pub fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut min_index;
    for i in 0..(arr.len() - 1) {
        // n
        min_index = i;
        for j in (i + 1)..arr.len() {
            //n
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        let temp = arr[min_index];
        arr[min_index] = arr[i];
        arr[i] = temp
    }
    arr

    // complexity = O(n^2)
}

pub fn insertion_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 1..arr.len() {
        let current_value = arr[i];
        let mut j = i as isize - 1; // Use isize to allow negative values

        // Find the correct position for current_value by shifting elements to the right
        while j >= 0 && arr[j as usize] > current_value {
            arr[j as usize + 1] = arr[j as usize];
            j -= 1;
        }

        arr[(j + 1) as usize] = current_value; // Place current_value at the correct position
    }
    arr
}

