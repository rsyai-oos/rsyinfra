//! rsycache is a local cache manager that belongs to the rsyinfra ecosystem.
//! It provides an abstraction for managing different types of caches (e.g., memory and disk).
use rsyhash::rustc_hash;
use rsyhash::rustc_hash::FxBuildHasher;
use rsyhash::rustc_hash::FxHashMap;

#[cfg(test)]
mod tests {
    use rsyhash::rustc_hash::{FxBuildHasher, FxHashMap};

    #[test]
    fn it_works() {
        let mut map = FxHashMap::default();
        map.insert(1, 2);
        assert_eq!(map.get(&1), Some(&2));
    }
}
