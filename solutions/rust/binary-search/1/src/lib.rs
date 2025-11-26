pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // todo!( "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index.");
    if array.is_empty() { return None; }
    let mid = array.len() / 2;
    if key > array[mid] { Some(mid + 1 + find(&array[mid + 1..], key)?) }
    else if key < array[mid] { find(&array[..mid], key) }
    else { Some(mid) }
}