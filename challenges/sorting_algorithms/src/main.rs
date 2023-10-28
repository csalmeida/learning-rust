fn main() {
    let numbers: Vec<u32> = vec![7, 2, 5, 4, 1, 6, 0, 3];
    let numbers_list: Vec<u32> = selection_sort(&numbers);

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
fn selection_sort(numbers: &Vec<u32>) -> Vec<u32> {
    // Clones the list so that the original one is not modified.
    let mut numbers_list = numbers.clone();

    for index in 0..numbers_list.len() {
        let number: u32 = numbers_list[index];
        let mut lowest: u32 = number;
        let mut swap_position: usize = index;

        for position in 0..numbers_list.len() {
            let comparable: u32 = numbers_list[position];

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

#[cfg(test)]
mod tests {
    use crate::selection_sort;

    #[test]
    fn selection_sort_orders_handles_empty_vec() {
        let original: Vec<u32> = vec![];
        let result = vec![];

        let sorted = selection_sort(&original);
        assert_eq!(sorted, result);
    }

    #[test]
    fn selection_sort_orders_handles_single_number() {
        let original = vec![7];
        let result = vec![7];

        let sorted = selection_sort(&original);
        assert_eq!(sorted, result);
    }

    #[test]
    fn selection_sort_orders_handles_two_numbers() {
        let original = vec![7, 2];
        let result = vec![2, 7];

        let sorted = selection_sort(&original);
        assert_eq!(sorted, result);
    }

    #[test]
    fn selection_sort_orders_numbers() {
        let original = vec![7, 2, 5, 4, 1, 6, 0, 3];
        let result = vec![0, 1, 2, 3, 4, 5, 6, 7];

        let sorted = selection_sort(&original);
        assert_eq!(sorted, result);
    }

    #[test]
    fn selection_sort_orders_base_ten() {
        let original = vec![70, 20, 50, 100, 40, 90, 10, 60, 30, 80];
        let result = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        let sorted = selection_sort(&original);
        assert_eq!(sorted, result);
    }

    #[test]
    fn selection_sort_orders_base_hundred() {
        let original = vec![700, 200, 500, 1000, 400, 900, 100, 600, 300, 800];
        let result = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];

        let sorted = selection_sort(&original);
        assert_eq!(sorted, result);
    }

    #[test]
    fn selection_sort_orders_base_thousand() {
        let original = vec![7000, 2000, 5000, 10000, 4000, 9000, 1000, 6000, 3000, 8000];
        let result = vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10_000];

        let sorted = selection_sort(&original);
        assert_eq!(sorted, result);
    }
}
