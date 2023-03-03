extern crate quick_sort;
use quick_sort::quick_sort;
use std::cmp::min;

fn main() {
    let mut array = vec![5, 1, -10, 3, 100, 20000, 77, 6, 7, -1, 0];

    let i = 9;

    let answer = selection(&mut array, 0, 11, i - 1);

    println!("{}th smallest element = {}", i, answer);
}

fn selection<T: Ord + Clone>(array: &mut Vec<T>, start: usize, end: usize, element: usize) -> T {
    if end - start <= 1 {
        return array.get(start).unwrap().clone();
    }

    let mut medians: Vec<T> = Default::default();

    for a in (start..end).step_by(5) {
        let b = min(a + 5, end);
        quick_sort(array, a, b);

        let median = array.get(((b - a) / 2) + a).unwrap().clone();
        medians.push(median);
    }

    let medians_size = medians.len();

    let pivot = selection(&mut medians, 0, medians_size, medians_size / 2);
    
    let pivot_index = array.iter().position(|e| *e == pivot).unwrap();

    array.swap(start, pivot_index);

    let mut seperator_index = start;

    for i in (start + 1)..end {
        if array.get(i) < array.get(start) {
            array.swap(i, seperator_index + 1);

            seperator_index += 1;
        }
    }

    array.swap(start, seperator_index);
    
    if element == seperator_index {
        array.get(seperator_index).unwrap().clone()
    }
    else if element < seperator_index {
        selection(array, start, seperator_index, element)
    } else {
        selection(array, seperator_index + 1, end, element)
    }
}
