/*
    Selection Sort
    Bubble Sort
    insertion sort
    merge sort
    heap sort
    quick sort
*/
#[allow(dead_code)]
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

    // complexity = O(n^2)
}
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    // complexity: O(n^2)
}

#[allow(dead_code)]
pub fn quick_sort(arr: &mut [i32]) {
    let ln = arr.len();
    if ln < 2 {
        return;
    }
    let p_index = partition(arr);
    quick_sort(&mut arr[0..p_index]);
    quick_sort(&mut arr[p_index + 1..]);
    // complexity: average:O(n * log n)
}

fn partition(arr: &mut [i32]) -> usize {
    let ln = arr.len();
    let mut i = 0;
    let pivot = arr[ln - 1]; //i took last element of array
    for j in 0..ln - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, ln - 1);
    i
}

#[allow(dead_code)]

pub fn counting_sort(arr: &mut [i32]) {
    let max_val = *arr.iter().max().unwrap();
    let mut count: Vec<i32> = vec![0; (max_val + 1) as usize];
    for &num in arr.iter() {
        count[num as usize] += 1;
    }
    let mut index = 0;
    for (num, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            arr[index] = num as i32;
            index += 1;
        }
    }
}

#[allow(dead_code)]
pub fn merge_sort(arr: &mut [i32]) -> &[i32] {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid + 1..]);
    merge(arr, 0, mid, arr.len() - 1)
}

fn merge(arr: &mut [i32], start: usize, mid: usize, end: usize) -> &[i32] {
    arr
}
