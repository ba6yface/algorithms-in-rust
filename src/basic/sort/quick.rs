use std::cmp::PartialOrd;
use std::fmt::Debug;

pub fn quick_sort<T>(list: &mut [T])
    where T: PartialOrd + Copy + Debug
{
    println!("{:?}", list);
    if list.len() > 1 {
        let partition_index = partition_exchange(list);
        quick_sort(&mut list[..partition_index]);
        quick_sort(&mut list[partition_index+1..]);
    }
}

fn partition_exchange<T>(list: &mut [T]) -> usize
    where T: PartialOrd + Copy + Debug
{
    let pivot = *list.first().unwrap();

    let mut left_index: usize = 0;
    let mut right_index: usize = list.len() - 1;

    println!("pivot: {:?}, right: {}", pivot, right_index);

    while left_index < right_index {
        while (list[left_index] < pivot) && (left_index < right_index) {
            println!("left index: {}", left_index);
            left_index += 1;
        }

        while (list[right_index] > pivot) && (left_index < right_index) {
            println!("right index: {}", left_index);
            right_index -= 1;
        }

        list.swap(left_index, right_index);
        left_index += 1;
        right_index -= 1;
    }

    return left_index;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![2, 1, 3, 5, 4];
        quick_sort(&mut arr);
        println!("{:?}", arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);

        let mut arr = vec![6, 1, 3, 2, 5, 4];
        quick_sort(&mut arr);
        println!("{:?}", arr);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], arr);

        let mut arr = vec![2, 1, 2, 2, 2, 1];
        quick_sort(&mut arr);
        println!("{:?}", arr);
        assert_eq!(vec![1, 1, 2, 2, 2, 2], arr);
    }
}