use std::cmp::PartialOrd;
use std::fmt::Debug;

pub fn quick_sort<T>(list: &mut [T])
    where T: PartialOrd + Copy + Debug
{
    if list.len() > 1 {
        let partition_index = partition_exchange(list);
        quick_sort(&mut list[..partition_index]);
        quick_sort(&mut list[(partition_index+1)..]);
    }
}

fn partition_exchange<T>(list: &mut [T]) -> usize
    where T: PartialOrd + Copy + Debug
{
    let pivot = *list.last().unwrap();

    let mut partition_index: isize = -1;
    let right_index: usize = list.len() - 1;

    for i in 0..right_index {
        if list[i] <= pivot {
            partition_index += 1;
            list.swap(partition_index as usize, i);
        }
    }

    partition_index += 1;
    list.swap(partition_index as usize, right_index);

    return partition_index as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quickort() {
        let mut arr = vec![2, 1, 3, 5, 4];
        quick_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);

        let mut arr = vec![6, 1, 3, 2, 5, 4];
        quick_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], arr);

        let mut arr = vec![2, 1, 2, 2, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(vec![1, 1, 2, 2, 2, 2], arr);
    }
}