fn main() {
    let array = vec![1, 4, 6, 8, 5, 2, 3, 5, 7, -1];

    let sorted = merge_sort(array);

    println!("{:?}", sorted);
}

fn merge_sort<T: Ord + Clone>(array: Vec<T>) -> Vec<T> {
    if array.len() < 2 {
        return array;
    }

    let mut sorted: Vec<T> = Default::default();

    let halved_array = array.split_at(array.len() / 2);

    let left_half = merge_sort(halved_array.0.to_vec());
    let right_half = merge_sort(halved_array.1.to_vec());

    let mut left_half_cursor = left_half.iter().peekable();
    let mut right_half_cursor = right_half.iter().peekable();

    let mut insert_element = |element: T, cursor: &mut std::iter::Peekable<std::slice::Iter<T>>| {
        sorted.push(element);
        cursor.next();
    };

    loop {
        let left = left_half_cursor.peek();
        let right = right_half_cursor.peek();

        if left.is_some() && right.is_some() {
            let a = (*left.unwrap()).clone();
            let b = (*right.unwrap()).clone();

            match std::cmp::Ord::cmp(&a, &b) {
                std::cmp::Ordering::Less => {
                    insert_element(a, &mut left_half_cursor);
                }
                std::cmp::Ordering::Greater => {
                    insert_element(b, &mut right_half_cursor);
                }
                std::cmp::Ordering::Equal => {
                    insert_element(a, &mut left_half_cursor);
                    insert_element(b, &mut right_half_cursor);
                }
            }
        } else if left.is_some() {
            insert_element((*left.unwrap()).clone(), &mut left_half_cursor);
        } else if right.is_some() {
            insert_element((*right.unwrap()).clone(), &mut right_half_cursor);
        } else {
            break;
        }
    }

    sorted
}
