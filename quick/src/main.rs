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

fn swap(arr: &mut [i32], src: usize, dest: usize) -> &[i32] {
    let temp = arr[src];
    arr[src] = arr[dest];
    arr[dest] = temp;
    return arr;
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
    let pivot = arr[to - 1];
    // Index of the first element greater than the pivot, and
    // after the last element that is less than the pivot.
    let mut ix_greater = 0;
    // Index of the first unchecked element, after the last
    // element that is greater than the pivot.
    let mut ix_unchecked = 0;
    let mut pivot_count = 0;
    while ix_unchecked < len {
        if arr[from + ix_unchecked] < pivot {
            swap(arr, from + ix_unchecked, from + ix_greater);
            ix_greater = ix_greater + 1;
        } else if arr[from + ix_unchecked] == pivot {
            swap(arr, from + ix_unchecked, from + ix_greater);
            ix_greater = ix_greater + 1;
            pivot_count = pivot_count + 1;
        }
        ix_unchecked = ix_unchecked + 1;
    }

    // Index of the first element equal to the pivot.
    let mut ix_pivot = 0;
    // Starting from the pivot (one before ix_greater), expand the pivot
    // range by finding other elements that equal the pivot. There will
    // be other elements = pivot in the first half, but this is okay.
    for ix in 1..ix_greater {
        if arr[from + ix_greater - ix] != pivot {
            ix_pivot = ix_greater - ix + 1;
            break;
        }
    }

    // All items = the pivot, so this range is sorted
    if pivot_count == len {
        return arr;
    }

    // Recursively sort the two halves, ignoring the pivot range
    quick_sort(arr, from, from + ix_pivot);
    quick_sort(arr, from + ix_greater, to);

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
