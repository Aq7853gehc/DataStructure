mod sorting_algos;
use sorting_algos::{insertion_sort, quick_sort};

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90, 5];
    quick_sort(&mut arr);
    println!("{:?}",arr);
}
