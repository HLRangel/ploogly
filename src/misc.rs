use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_hashmap<K,V: Hash>(map: &HashMap<K, V>) -> u64 {
    let mut final_val: u64 = 0;
    for (k, v) in map.into_iter() {
        let mut h = DefaultHasher::new();
        v.hash(&mut h);

        final_val ^= h.finish();
    }

    final_val
}
