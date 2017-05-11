use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut result:BTreeMap<String, i32> = BTreeMap::new();
    for (point, keys) in input {
        for key in keys {
            result.insert(key.to_lowercase(), *point);
        }
    }
    result
}
