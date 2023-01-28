#[allow(dead_code)]
fn is_array_eq(arr1: &[i32], arr2: &[i32]) -> bool {
    for (ix, i) in arr1.into_iter().enumerate() {
        if arr2[ix] != *i {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::heap_sort;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base0() {
        // Empty arrays should not change
        assert_eq!(true, is_array_eq(heap_sort(&mut []), &[]));
    }

    #[test]
    fn test_base1() {
        // Arrays containing a single element should not change
        assert_eq!(true, is_array_eq(heap_sort(&mut [42]), &[42]));
    }

    #[test]
    fn test_sorted() {
        // Arrays containing sorted elements should not change
        assert_eq!(true, is_array_eq(heap_sort(&mut [1, 2]), &[1, 2]));
    }

    #[test]
    fn test_sorted_equal() {
        // Arrays containing sorted elements should not change
        assert_eq!(
            true,
            is_array_eq(
                heap_sort(&mut [1, 1, 1, 1, 1, 1, 1]),
                &[1, 1, 1, 1, 1, 1, 1]
            )
        );
    }

    #[test]
    fn test_unsorted_smallest() {
        // Arrays of length 2 of unsorted i32 should be sorted
        assert_eq!(true, is_array_eq(heap_sort(&mut [2, 1]), &[1, 2]));
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            true,
            is_array_eq(
                heap_sort(&mut [3, 6, 5, 7, 9, 4, 8, 1, 2]),
                &[1, 2, 3, 4, 5, 6, 7, 8, 9]
            )
        );
    }

    #[test]
    fn test_similar() {
        assert_eq!(
            true,
            is_array_eq(
                heap_sort(&mut [100, 100, 100, 100, 99, 100, 100]),
                &[99, 100, 100, 100, 100, 100, 100]
            )
        );
    }

    #[test]
    fn test_large() {
        assert_eq!(
            true,
            is_array_eq(
                heap_sort(&mut [
                    3, 6, 5, 7, 9, 4, 8, 1, 2, 3, 6, 5, 7, 9, 4, 8, 1, 2, 3, 6, 5, 7, 9, 4, 8, 1,
                    2, 3, 6, 5, 7, 9, 4, 8, 1, 2, 3, 6, 5, 7, 9, 4, 8, 1, 2
                ]),
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
        println!("#\n# Warning: this might take a long time\n#");
        // Compare the standard sort to the merge sort
        assert_eq!(
            true,
            is_array_eq(heap_sort(v.as_mut_slice()), sorted.as_mut_slice())
        );
    }
}
