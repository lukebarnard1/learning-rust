mod test;

fn main() {
    println!("Use 'cargo test' to run tests")
}

fn swap(arr: &mut [i32], src: usize, dest: usize) -> &[i32] {
    let temp = arr[src];
    arr[src] = arr[dest];
    arr[dest] = temp;
    return arr;
}

/*
 * Heaps
 *
 * Heaps are stored in array such that the first element is the root of an incomplete binary tree.
 * The following two elements are the decendents (left, right) of the first node. The following
 * four elements are the decendents of the left and right node of the root, and so on.
 *
 *       1
 *     2   3
 *    4 5 6 7
 *   ...
 *
 * To traverse the tree, the following equations can be used on the current node index, i. Note the
 * tree is 1-indexed.
 *
 * Left node = 2i
 * Right node = 2i + 1
 * Parent node = i / 2
 *
 * Terminating nodes must also be represented. For simplicity, they will be represented with -1.
 *
 * Here's an example of a heap:
 *
 * [3, 6, 5, 7, 9, 4, 8, 1, 2]
 *
 * 0:         3
 * 1:      6     5
 * 3:    7   9  4  8
 * 7:  1  2
 *
 */

fn left(i: usize) -> usize {
    return 2 * i + 1;
}
fn right(i: usize) -> usize {
    return 2 * (i + 1);
}
fn parent(i: usize) -> usize {
    return ((i + 1) / 2) - 1;
}

/*
 * Find and correct a single violation of the "max heap" property:
 *  - For any node at i, left(i) and right(i) are ≤
 *
 * This function assumes that the trees at left(i) and right(i) are max heaps. This means we can
 * start at node i, check if it needs to be swapped with the child nodes, then repeat after
 * swapping a child by moving to the swapped child until we reach a terminating node.
 */
fn max_heapify(arr: &mut [i32], ix_start: usize) {
    let ix_left = left(ix_start);
    let ix_right = right(ix_start);
    if arr[ix_left] >= 0 && arr[ix_start] < arr[ix_left] {
        swap(arr, ix_start, ix_left);
        max_heapify(arr, ix_left);
    }
    if arr[ix_right] >= 0 && arr[ix_start] < arr[ix_right] {
        swap(arr, ix_start, ix_right);
        max_heapify(arr, ix_right);
    }
}

/*
 * An implementation of heap_sort, which first creates a max heap from the input array, then
 * repeatidly removes the root, swapping the last item and decreasing the heap size until no
 * items remain in the heap.
 */
pub fn heap_sort(arr: &mut [i32]) -> &[i32] {
    // Pad the array with -1, the terminating value.
    let len = arr.len();
    if len < 2 {
        return arr;
    }

    let mut next_pow = 1;
    for i in 1..32 {
        next_pow = 2_usize.pow(i);
        if next_pow > len {
            break;
        }
    }

    let mut heap = vec![-1_i32; next_pow * 2];

    for ix in 0..len {
        heap[ix] = arr[ix];
    }

    // Create max heap by starting at the parents of leaf nodes, which are already max heaps.
    // max_heapify will only work on sub-trees who's children are max heaps, which leaves are
    // because they have no children.
    for i in 0..(len / 2) {
        // j = n / 2 - 1 -> 0
        let j = len / 2 - 1 - i;
        max_heapify(&mut heap, j);
    }

    // Now that heap is a max heap, repeatedly remove the largest item, which is always at the
    // root, and swap it with the last item, the propagate that item to a valid place in the heap
    // by using max_heapify. max_heapify works because the children of the root after the swap are
    // still max heaps. After the max_heapify, the heap is a max heap again and the largest item is
    // at the root.
    let mut heap_size = len;
    while heap_size > 0 {
        swap(&mut heap, 0, heap_size - 1);
        // Get the largest number in the heap, discard it
        let discarded = heap[heap_size - 1];
        heap[heap_size - 1] = -1;

        // Fix any violation introduced by the new root
        max_heapify(&mut heap, 0);

        // TODO: To make this in-place, without the Vec<i32>, the
        // largest element can be moved after the end of the heap.
        //
        // Insert new largest into new array
        //   Note: This can be reversed by inserting by indexing with len - heap_size
        arr[heap_size - 1] = discarded;

        print!(
            "\x1b[40Dheap sort running - {}%  ",
            100 - 100 * heap_size / len
        );
        heap_size = heap_size - 1;
    }
    println!("\x1b[40Dheap sort complete     ");

    return arr;
}
