use quick_sort::quick_sort;

fn main() {
    let mut array = vec![5, 1, -10, 3, 100, 20000, 77, 6, 7, -1, 0];

    quick_sort(&mut array, 0, 11);

    println!("{:?}", array);
}

