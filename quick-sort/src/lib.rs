use rand::Rng;

pub fn quick_sort<T: Ord>(array: &mut Vec<T>, start: usize, end: usize) {
    if end - start <= 1 {
        return;
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

    quick_sort(array, start, seperator_index + 1);
    quick_sort(array, seperator_index + 1, end);
}