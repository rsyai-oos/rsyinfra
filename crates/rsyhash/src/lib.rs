//! rsyhash is a wrapper for rust hash library.
//!
pub use rustc_hash;
use rustc_hash::FxBuildHasher;
// use rustc_hash::FxHashMap;

#[cfg(test)]
mod rustc_hash_tests {
    #[test]
    fn default_test() {
        // use rustc_hash::FxHashMap;
        // let mut map: FxHashMap<u32, u32> = FxHashMap::default();
        // map.insert(22, 44);
        // assert_eq!(map.get(&22), Some(&44));
        use rustc_hash::FxHashMap;
    }
}
