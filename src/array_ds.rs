use std::io::stdin;
pub fn insert_beg(mut arr: Vec<i32>) -> Vec<i32> {
    let mut arr2 = arr.clone();
    println!("Enter the value to be insert:");
    let mut data = String::new();
    stdin().read_line(&mut data).expect("failed to read");

    for i in (0..=arr.len()).rev() {
        arr2[i] = arr[i - 1]
    }
    arr2[0] = data.trim().parse().unwrap_or(0);
    return arr2;
}

pub fn create_array() -> Vec<i32> {
    let mut arr: Vec<i32> = vec![];
    println!("Enter the Size of array:");
    let mut size: String = String::new();
    stdin().read_line(&mut size).expect("Failed to read");
    for i in 1..=size.trim().parse().unwrap() {
        println!("Enter the value of {i} in array");
        let mut data: String = String::new();
        stdin().read_line(&mut data).expect("failed to read input");
        arr.push(data.trim().parse().unwrap_or(0));
    }

    return arr;
}

pub fn print_arr(arr: Vec<i32>) {
    println!("Your array:");
    for i in arr.iter() {
        println!("{i}");
    }
}

pub fn find_max_element(arr: Vec<i32>) -> i32 {
    let mut temp = arr[0];
    for i in 0..arr.len() {
        if temp < arr[i] {
            temp = arr[i];
        }
    }
    temp
}

pub fn find_min_element(arr: Vec<i32>) -> i32 {
    let mut temp = arr[0];

    for i in 0..arr.len() {
        if temp > arr[i] {
            temp = arr[i];
        }
    }
    temp
}

pub fn sum_of_all(arr: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in arr.iter() {
        sum += i;
    }
    sum
}

pub fn search_element(arr: Vec<i32>, target: i32) -> usize {
    let mut index = 0usize;
    for i in 0..arr.len() {
        if target == arr[i] {
            index = i + 1;
            break;
        }
    }
    index
}

pub fn count_occurence(arr: Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    for i in 0..arr.len() {
        if arr[i] == target {
            count += 1
        }
    }
    count
}

pub fn rotate_array(mut arr: Vec<i32>, steps: i32) -> Vec<i32> {
    for _i in 0..steps {
        let temp = arr[arr.len() - 1];
        for j in (1..arr.len()).rev() {
            arr[j] = arr[j - 1];
        }
        arr[0] = temp;
    }
    arr
}


pub fn find_second_max_element(arr: Vec<i32>)->i32{
    let max = find_max_element(arr.clone());

    let mut s_max = arr[0];
    let mut temp:i32;
   for i in 0..arr.len(){
    temp = arr[i];
    if temp == max{
        continue;
    }else {
        if s_max < arr[i]{
            s_max = arr[i];
        }
    }
   }
   s_max
}