use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // todo!("How will you transform the tree {h:?}?")
    let mut new_mapping = BTreeMap::new();
    for (key, values) in h {
        for value in values {
            new_mapping.insert(value.to_ascii_lowercase(), *key);
        };
    };
    new_mapping
}