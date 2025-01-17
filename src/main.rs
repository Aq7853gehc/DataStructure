use sorting_algos::merge_sort;

mod sorting_algos;
// use sorting_algos::counting_sort;
#[allow(unused_labels, unreachable_code, unused_mut)]
fn main() {
    let mut arr = [12, 8, 9, 3, 11, 5, 4];
    merge_sort(&mut arr);
    println!("{:?}", arr);
}
