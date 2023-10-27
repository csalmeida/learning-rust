fn main() {
    let numbers: Vec<u8> = vec![7, 2, 5, 4, 1, 6, 0, 3];
    let numbers_list: Vec<u8> = selection_sort(&numbers);

    println!("Start list: {:?}", numbers);
    println!("Sorted list: {:?}", numbers_list);
}

/// Applies the selection sort algorithm to a `Vec<u8>`.
/// It sorts a list of numbers to ascending order.
///
/// For example, passing `7, 2, 5, 4, 1, 6, 0, 3` will return `0, 1, 2, 3, 4, 5, 6, 7`.
/// Here's an outline of how the algorithm works:
/// 1. Compares each number with all the numbers to the right.
/// 2. In each iteration it find the lowest number and keeps track of it's position.
/// 3. Once all numbers are checked it swaps them.
///
/// It will follow those steps for the length of the array until all items are swapped, O(n).
///
/// Because it's challenging to modify collection when iterating them, this function makes use of indices to allow the items to be swapped.
///
/// When sorting the list, here are the results the function yields:
///
/// ```
/// [7, 2, 5, 4, 1, 6, 0, 3]
/// [0, 2, 5, 4, 1, 6, 7, 3]
/// [0, 1, 5, 4, 2, 6, 7, 3]
/// [0, 1, 2, 4, 5, 6, 7, 3]
/// [0, 1, 2, 3, 5, 6, 7, 4]
/// [0, 1, 2, 3, 4, 6, 7, 5]
/// [0, 1, 2, 3, 4, 5, 7, 6]
/// [0, 1, 2, 3, 4, 5, 6, 7]
/// ```
///
fn selection_sort(numbers: &Vec<u8>) -> Vec<u8> {
    // Clones the list so that the original one is not modified.
    let mut numbers_list = numbers.clone();

    for index in 0..numbers_list.len() {
        let number: u8 = numbers_list[index];
        let mut lowest: u8 = number;
        let mut swap_position: usize = index;

        for position in 0..numbers_list.len() {
            let comparable: u8 = numbers_list[position];

            if position <= index {
                continue;
            }

            if lowest > comparable {
                lowest = comparable;
                swap_position = position;
            }
        }

        numbers_list.swap(index, swap_position);
    }

    return numbers_list;
}
