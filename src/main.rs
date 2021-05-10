mod sort;

use sort::Sort;

fn main() {
    let mut arr = [ 6, 5, 2, 7, 3, 9, 8, 4, 10, 1 ];
    Sort::quickSort(&mut arr);

    for n in arr.iter() {
        println!("result: {}", n);
    }
}
