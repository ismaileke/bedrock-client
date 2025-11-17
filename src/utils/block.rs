use std::collections::HashMap;

pub type PropertyMap = HashMap<String, Vec<PropertyValue>>;
pub type StateCombination = HashMap<String, PropertyValue>;

const FNV1_32_INIT: u32 = 0x811c9dc5;
const FNV1_PRIME_32: u32 = 0x0100_0193;

const FNV1_64_INIT: u64 = 0xcbf29ce484222325;
const FNV1_PRIME_64: u64 = 0x00000100000001b3;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyValue {
    Int(i32),
    Str(String),
    Byte(i8)
}

pub fn fnv1a_32(data: &[u8]) -> u32 {
    let mut hash: u32 = FNV1_32_INIT;
    for &datum in data {
        hash ^= datum as u32;
        hash = hash.wrapping_mul(FNV1_PRIME_32);
    }
    hash
}

pub fn fnv1_64(data: &[u8]) -> u64 {
    let mut hash: u64 = FNV1_64_INIT;
    for &datum in data {
        hash = hash.wrapping_mul(FNV1_PRIME_64);
        hash ^= datum as u64;
    }
    hash
}

pub fn cartesian_product_enum(properties: &PropertyMap) -> Vec<StateCombination> {
    let mut results = vec![];

    let mut keys = properties.keys().cloned().collect::<Vec<_>>();
    keys.sort();

    fn helper(
        keys: &[String],
        index: usize,
        properties: &PropertyMap,
        current: &mut StateCombination,
        results: &mut Vec<StateCombination>
    ) {
        if index == keys.len() {
            results.push(current.clone());
            return;
        }

        let key = &keys[index];
        if let Some(values) = properties.get(key) {
            for value in values {
                current.insert(key.clone(), value.clone());
                helper(keys, index + 1, properties, current, results);
                current.remove(key);
            }
        }
    }

    let mut current = HashMap::new();
    helper(&keys, 0, properties, &mut current, &mut results);

    results
}