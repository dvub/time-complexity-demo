pub fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}
pub fn selection_sort(array: &mut [i32]) {
    for index in 0..array.len() - 1 {
        let mut minimum_index = index;
        for j in index + 1..array.len() {
            if array[j] < array[minimum_index] {
                minimum_index = j;
            }
        }
        if minimum_index != index {
            array.swap(minimum_index, index);
        }
    }
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged
}
fn merge_sort_r(vec: &[i32]) -> Vec<i32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge_sort_r(&vec[0..size]);
        let right = merge_sort_r(&vec[size..]);

        merge(&left, &right)
    }
}
#[cfg(test)]
mod tests {
    use crate::{insertion_sort, selection_sort};

    use super::merge_sort_r;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 3, 4, 1, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_selection_sort() {
        let mut v = vec![5, 3, 4, 1, 2];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_merge_sort() {
        let v = vec![5, 3, 4, 1, 2];
        let sorted = merge_sort_r(&v);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }
}
