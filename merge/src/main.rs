fn main() {
    println!("Use 'cargo test' to run tests")
}

#[allow(dead_code)]
fn is_array_eq(arr1: &[i32], arr2: &[i32]) -> bool {
    println!("{:?} {:?}", arr1, arr2);
    for (ix, i) in arr1.into_iter().enumerate() {
       if arr2[ix] != *i {
           return false;
       }
    }
    return true;
}

// Resursively half the input, then sort that until the base conditions
pub fn merge_sort (arr: &mut [i32], from: usize, to: usize) -> &[i32] {
  let len : usize = to - from;
  // Elements of less than length 2 are already sorted
  if len < 2 {
      return arr;
  }
  // Array of length 2 can be easily compared and swapped if necessary
  if len == 2 {
      if arr[from] < arr[from + 1] {
          return arr;
      } else {
          let t = arr[from];
          arr[from] = arr[from + 1];
          arr[from + 1] = t;
          return arr;
      }
  } else {
      // Otherwise, take the two halves of the range to be sorted, sort them and then merge them
      // into a sorted array.
      //
      // Sort the two halves
      let l = len / 2;
      merge_sort(arr, from, to - l);
      merge_sort(arr, from + l, to);

      // Merge the two halves
      let mut left_ix = from;
      let mut right_ix = from + l;

      // Use a temporary vector to store the sorted range
      let mut arr2 : Vec<i32> = vec![0; len];

      for ix in from..to {
          // Only allow the left array item to be inserted if the left index
          // is still within the left half
          //
          // Force the left array item to be inserted if the right index is
          // outside of the right half range
          //
          // Otherwise, decide on which item to use based on a comparison
          if left_ix < from + l && (right_ix >= to || arr[left_ix] < arr[right_ix]) {
              arr2[ix - from] = arr[left_ix];
              left_ix = left_ix + 1;
          } else {
              arr2[ix - from] = arr[right_ix];
              right_ix = right_ix + 1;
          }
      }
      // Copy the temporary vector back into the new array
      for ix in from..to {
          arr[ix] = arr2[ix - from];
      }
  }
  return arr;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_base0() {
        // Empty arrays should not change
        assert_eq!(true, is_array_eq(merge_sort(&mut[], 0, 0),&[]));
    }

    #[test]
    fn test_base1() {
        // Arrays containing a single element should not change
        assert_eq!(true, is_array_eq(merge_sort(&mut[42], 0, 1),&[42]));
    }

    #[test]
    fn test_sorted() {
        // Arrays containing sorted elements should not change
        assert_eq!(true, is_array_eq(merge_sort(&mut [1, 2], 0, 2),&[1, 2]));
    }

    #[test]
    fn test_unsorted_smallest() {
        // Arrays of length 2 of unsorted i32 should be sorted
        assert_eq!(true, is_array_eq(merge_sort(&mut[2, 1], 0, 2),&[1, 2]));
    }

    #[test]
    fn test_simple() {
        assert_eq!(true, is_array_eq(merge_sort(&mut[3,6,5,7,9,4,8,1,2], 0, 9),&[1,2,3,4,5,6,7,8,9]));
    }

    #[test]
    fn test_large() {
        assert_eq!(true, is_array_eq(merge_sort(
          &mut[3,6,5,7,9,4,8,1,2,3,6,5,7,9,4,8,1,2,3,6,5,7,9,4,8,1,2,3,6,5,7,9,4,8,1,2,3,6,5,7,9,4,8,1,2], 0, 45),
             &[1,1,1,1,1,2,2,2,2,2,3,3,3,3,3,4,4,4,4,4,5,5,5,5,5,6,6,6,6,6,7,7,7,7,7,8,8,8,8,8,9,9,9,9,9])
        );
    }

    #[test]
    fn test_n() {
        // Generate pseudo-random array
        let mut v : Vec<i32> = vec![0; 3243567];
        let mut c : i32 = 0;
        for i in 0..v.len() {
            c = c + 1;
            v[i] = ((((c + 4) % 340) * 56) % (2 + c + 423)).into();
        }
        // Sort clone using Rust standard library
        let mut sorted = v.clone();
        sorted.sort();
        let l = v.len();
        // Compare the standard sort to the merge sort
        assert_eq!(true, is_array_eq(merge_sort(v.as_mut_slice(), 0, l), sorted.as_mut_slice()));
    }
}
