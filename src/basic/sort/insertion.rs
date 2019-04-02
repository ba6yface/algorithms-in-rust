use std::cmp::PartialOrd;

pub fn insertion_sort<T>(list: &mut [T])
    where T: PartialOrd + Copy
{
    for i in 1..list.len() {
        let mut j = i;
        let tmp = list[j];
        while j > 0 && list[j - 1] > tmp {
            list[j] = list[j - 1];
            j -= 1;
        }
        list[j] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![2, 1, 3, 5, 4];
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        assert_eq!(vec![1, 2, 3, 4, 5], arr);
    }
}