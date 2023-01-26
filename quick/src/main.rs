fn main() {
    println!("Use 'cargo test' to run tests")
}

#[allow(dead_code)]
fn is_array_eq(arr1: &[i32], arr2: &[i32]) -> bool {
    for (ix, i) in arr1.into_iter().enumerate() {
        if arr2[ix] != *i {
            return false;
        }
    }
    return true;
}

// 1. Choose a pivot value to split the range roughly in half
// 2. Sort the elements relative to the pivot by iteratively comparing
//    them
// 3. Recursively sort each half, either side of the pivot
pub fn quick_sort(arr: &mut [i32], from: usize, to: usize) -> &[i32] {
    let len = to - from;
    // Ranges smaller than 2 are considered sorted
    if len < 2 {
        return arr;
    }

    // Ranges of length 2 can be sorted simply
    if len == 2 {
        if arr[from] < arr[from + 1] {
            return arr;
        } else {
            let t = arr[from];
            arr[from] = arr[from + 1];
            arr[from + 1] = t;
            return arr;
        }
    }

    // Choose a value to split the range into two halves
    let pivot = (arr[from] + arr[from + len / 2] + arr[to - 1]) / 3;

    // TODO: This can be done in-place apparently
    // Construct the new halves
    let mut arr_new_left: Vec<i32> = vec![];
    let mut arr_new_pivot: Vec<i32> = vec![];
    let mut arr_new_right: Vec<i32> = vec![];
    for ix in 0..len {
        let el = arr[from + ix];
        if el > pivot {
            arr_new_right.push(el);
        } else if el == pivot {
            arr_new_pivot.push(el);
        } else {
            arr_new_left.push(el);
        }
    }

    // Reconstruct the two halves into a whole, with the pivot in the middle. There might be
    // multiple values that are equal to the pivot.
    let left_len = arr_new_left.len();
    let right_len = arr_new_right.len();
    let mut ix = from;
    for item in arr_new_left.iter() {
        arr[ix] = *item;
        ix = ix + 1;
    }
    for item in arr_new_pivot.iter() {
        arr[ix] = *item;
        ix = ix + 1;
    }
    for item in arr_new_right.iter() {
        arr[ix] = *item;
        ix = ix + 1;
    }

    // Recursively sort the two halves, ignoring the pivot range
    quick_sort(arr, from, from + left_len);
    quick_sort(arr, to - right_len, to);

    return arr;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base0() {
        // Empty arrays should not change
        assert_eq!(true, is_array_eq(quick_sort(&mut [], 0, 0), &[]));
    }

    #[test]
    fn test_base1() {
        // Arrays containing a single element should not change
        assert_eq!(true, is_array_eq(quick_sort(&mut [42], 0, 1), &[42]));
    }

    #[test]
    fn test_sorted() {
        // Arrays containing sorted elements should not change
        assert_eq!(true, is_array_eq(quick_sort(&mut [1, 2], 0, 2), &[1, 2]));
    }

    #[test]
    fn test_sorted_equal() {
        // Arrays containing sorted elements should not change
        assert_eq!(
            true,
            is_array_eq(
                quick_sort(&mut [1, 1, 1, 1, 1, 1, 1], 0, 7),
                &[1, 1, 1, 1, 1, 1, 1]
            )
        );
    }

    #[test]
    fn test_unsorted_smallest() {
        // Arrays of length 2 of unsorted i32 should be sorted
        assert_eq!(true, is_array_eq(quick_sort(&mut [2, 1], 0, 2), &[1, 2]));
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            true,
            is_array_eq(
                quick_sort(&mut [3, 6, 5, 7, 9, 4, 8, 1, 2], 0, 9),
                &[1, 2, 3, 4, 5, 6, 7, 8, 9]
            )
        );
    }

    #[test]
    fn test_similar() {
        assert_eq!(
            true,
            is_array_eq(
                quick_sort(&mut [100, 100, 100, 100, 99, 100, 100], 0, 7),
                &[99, 100, 100, 100, 100, 100, 100]
            )
        );
    }

    #[test]
    fn test_large() {
        assert_eq!(
            true,
            is_array_eq(
                quick_sort(
                    &mut [
                        3, 6, 5, 7, 9, 4, 8, 1, 2, 3, 6, 5, 7, 9, 4, 8, 1, 2, 3, 6, 5, 7, 9, 4, 8,
                        1, 2, 3, 6, 5, 7, 9, 4, 8, 1, 2, 3, 6, 5, 7, 9, 4, 8, 1, 2
                    ],
                    0,
                    45
                ),
                &[
                    1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6,
                    6, 6, 6, 6, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9
                ]
            )
        );
    }

    #[test]
    fn test_3_million() {
        // Generate pseudo-random array
        let mut v: Vec<i32> = vec![0; 3000000];
        let mut c: i32 = 0;
        for i in 0..v.len() {
            c = c + 1;
            v[i] = ((((c + 4) % 340) * 56) % (2 + c + 423)).into();
        }
        // Sort clone using Rust standard library
        let mut sorted = v.clone();
        sorted.sort();
        let l = v.len();
        // Compare the standard sort to the merge sort
        assert_eq!(
            true,
            is_array_eq(quick_sort(v.as_mut_slice(), 0, l), sorted.as_mut_slice())
        );
    }
}
