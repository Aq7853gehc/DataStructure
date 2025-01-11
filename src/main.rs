mod array_ds;

use array_ds::{count_occurence, search_element};

fn main() {
    let arr: [i32; 7] = [23, 45, 66, 99,99,99, 12];
    let target = 99;
    let index = search_element(arr.to_vec(), target);
    let count = count_occurence(arr.to_vec(), target);
    println!("{arr:?}");
    println!("{target} found  {count} times");
}
