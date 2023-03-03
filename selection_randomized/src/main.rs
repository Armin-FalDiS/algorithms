use rand::Rng;

fn main() {
    let mut array = vec![5, 1, -10, 3, 100, 20000, 77, 6, 7, -1, 0];

    let i = 5;

    let answer = selection(&mut array, 0, 11, i - 1);

    println!("{}th smallest element = {}", i, answer);
}

fn selection<T: Ord + Clone>(array: &mut Vec<T>, start: usize, end: usize, element: usize) -> T {
    if end - start <= 1 {
        return array.get(start).unwrap().clone();
    }

    let pivot = rand::thread_rng().gen_range(start..end);
    array.swap(start, pivot);

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
