use std::cmp::Ordering;

fn search<T: Ord>(list: &mut [T], target: &T) -> Option<usize> {
    list.sort_unstable();
    if list.len() == 0 {
        return None;
    }

    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match target.cmp(&list[mid]) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => right = mid,
        }
    }

    None
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array() {
        let index = search(&mut vec![], &1);
        assert_eq!(index, None)
    }

    #[test]
    fn one_element_in_array() {
        let index = search(&mut vec![23], &23);
        assert_eq!(index, Some(0))
    }

    #[test]
    fn one_element_not_in_array() {
        let index = search(&mut [23], &31);
        assert_eq!(index, None)
    }

    #[test]
    fn element_in_populated_list() {
        let index = search(&mut [1, 9, 45, 63, 31, 70, 20, 100, 2], &31);
        assert_eq!(index, Some(4))
    }
}
