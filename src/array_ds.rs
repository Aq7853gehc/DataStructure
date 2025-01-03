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


pub fn create_array()->Vec<i32>{
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

pub fn print_arr(arr:Vec<i32>){
    println!("Your array:");
    for i in arr.iter(){
        println!("{i}");
    }
}


