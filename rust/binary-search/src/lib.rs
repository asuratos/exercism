/// Recursive implementation of binary search
pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: Ord,
{
    let array = array.as_ref();

    // Checks for invalid inputs
    // Invalid if the array is empty or the target is not in the array itself
    if array.is_empty() || !array.contains(&key) {
        return None;
    }

    let mid = array.len() / 2;

    let current = &array[mid];

    if current == &key {
        return Some(mid);
    }

    let (left, right) = array.split_at(mid);

    if current < &key {
        if let Some(i) = find(right, key) {
            return Some(i + mid);
        }
    } else if let Some(i) = find(left, key) {
        return Some(i);
    };

    None
}
