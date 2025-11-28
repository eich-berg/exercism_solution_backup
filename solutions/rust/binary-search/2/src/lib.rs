pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // todo!( "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index.");
    if array.is_empty() { return None; }
    let mut left = 0;
    let mut right = array.len();
    while left < right {
        let mid = (left + right) / 2;
        if key < array[mid] {  right = mid; }
        else if key > array[mid] { left = mid + 1; }
        else { return Some(mid) }
        }
    None
}