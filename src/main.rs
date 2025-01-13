fn main() {
    let arr: Vec<i32> = vec![4, 5, 4, 6, 7, 8, 6, 5, 7];
    println!("Array: {:?}", arr);
    let ar: Vec<i32> = remove_duplicate(arr.clone());
    println!("Array: {:?}", ar);
}

fn remove_duplicate(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] == arr[j] && i != j {
                if j == (arr.len() - 1) {
                    break;
                }
                arr[j] = arr[j + 1];
            }
        }
    }

    println!("{}", arr.len());
    arr
}
