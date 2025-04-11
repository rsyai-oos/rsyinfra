//! rsycache is a local cache manager that belongs to the rsyinfra ecosystem.
//! It provides an abstraction for managing different types of caches (e.g., memory and disk).

use rsysync::parking_lot::{Mutex, MutexGuard};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Define the Cache trait with default implementations for as_any and as_any_mut
pub trait Cache: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct CacheManager {
    caches: Arc<RwLock<HashMap<String, Arc<dyn Cache>>>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            caches: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Add a cache to the manager
    pub fn add_cache(&self, name: String, cache: Arc<dyn Cache>) -> Result<(), String> {
        let mut caches = self.caches.write().unwrap();
        if caches.contains_key(&name) {
            return Err(format!("Cache with name {} already exists", name));
        }
        caches.insert(name, cache);
        Ok(())
    }

    // Remove a cache from the manager
    pub fn remove_cache(&self, name: &str) -> Result<(), String> {
        let mut caches = self.caches.write().unwrap();
        if !caches.contains_key(name) {
            return Err(format!("Cache with name {} does not exist", name));
        }
        caches.remove(name);
        Ok(())
    }

    // Retrieve a cache by name
    pub fn get_cache(&self, name: &str) -> Option<Arc<dyn Cache>> {
        let caches = self.caches.read().unwrap();
        caches.get(name).cloned()
    }

    // Get the number of caches managed
    pub fn cache_count(&self) -> usize {
        let caches = self.caches.read().unwrap();
        caches.len()
    }

    // Clear a specific cache
    pub fn clear_cache(&self, name: &str) -> Result<(), String> {
        let mut caches = self.caches.write().unwrap();
        if let Some(cache) = caches.get_mut(name) {
            if let Some(mem_cache) = cache.as_any_mut().downcast_mut::<MemCache>() {
                mem_cache.cache.clear();
            } else if let Some(disk_cache) = cache.as_any_mut().downcast_mut::<DiskCache>() {
                disk_cache.cache.clear();
            }
            Ok(())
        } else {
            Err(format!("Cache with name {} does not exist", name))
        }
    }

    // Clear all caches
    pub fn clear_all_caches(&self) {
        let mut caches = self.caches.write().unwrap();
        for cache in caches.values_mut() {
            if let Some(mem_cache) = cache.as_any_mut().downcast_mut::<MemCache>() {
                mem_cache.cache.clear();
            } else if let Some(disk_cache) = cache.as_any_mut().downcast_mut::<DiskCache>() {
                disk_cache.cache.clear();
            }
        }
    }
}

// CacheBuilder is responsible for creating instances of Cache
pub struct CacheBuilder {
    cache_type: CacheType,
}

impl CacheBuilder {
    pub fn new(cache_type: CacheType) -> Self {
        Self { cache_type }
    }

    // Build method creates a Cache instance based on the specified CacheType
    pub fn build(self) -> Result<Arc<dyn Cache>, String> {
        match self.cache_type {
            CacheType::MemCache => Ok(Arc::new(MemCache::new())),
            CacheType::DiskCache => {
                #[cfg(not(target_family = "wasm"))]
                {
                    Ok(Arc::new(DiskCache::new()))
                }
                #[cfg(target_family = "wasm")]
                {
                    Err("DiskCache not supported in Wasm".to_string())
                }
            }
            _ => Err("Unsupported CacheType".to_string()), // Handle unknown CacheType variants
        }
    }
}

// Enum representing different types of caches
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    MemCache,
    DiskCache,
}

// MultiCache is a placeholder for managing multiple caches
pub struct MultiCache {
    cachers: Vec<Arc<dyn Cache>>, // Renamed field for clarity
}

impl MultiCache {
    // Add methods to manage multiple caches
    pub fn new() -> Self {
        Self {
            cachers: Vec::new(),
        }
    }

    pub fn add_cache(&mut self, cache: Arc<dyn Cache>) {
        self.cachers.push(cache);
    }

    pub fn get_caches(&self) -> &Vec<Arc<dyn Cache>> {
        &self.cachers
    }
}

// MemCache represents an in-memory cache
pub struct MemCache {
    cache: HashMap<String, Vec<u8>>, // Use HashMap for efficient key-value caching
}

impl Cache for MemCache {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl MemCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    // Add methods to interact with the cache
    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.cache.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.cache.get(key)
    }
}

// DiskCache represents a disk-based cache
#[cfg(not(target_family = "wasm"))]
pub struct DiskCache {
    cache: HashMap<String, Vec<u8>>, // Use HashMap for efficient key-value caching
}

#[cfg(not(target_family = "wasm"))]
impl Cache for DiskCache {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(not(target_family = "wasm"))]
impl DiskCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    // Add methods to interact with the cache
    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.cache.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.cache.get(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CacheBuilder, DiskCache, MemCache};

    #[test]
    fn build_cache() {
        let cache = CacheBuilder::new(crate::CacheType::MemCache).build();
        if let Ok(cache) = cache {
            if let Some(_) = cache.as_any().downcast_ref::<MemCache>() {
                println!("Memory cache created successfully");
            } else {
                panic!("Downcasting failed");
            }
        } else {
            panic!("Failed to build cache");
        }
    }

    #[test]
    fn build_disk_cache() {
        let cache = CacheBuilder::new(crate::CacheType::DiskCache).build();
        assert!(cache.is_ok(), "Failed to build DiskCache");
    }

    #[test]
    #[should_panic(expected = "Unsupported CacheType")]
    fn build_invalid_cache() {
        let cache = CacheBuilder::new(crate::CacheType::MemCache).build(); // Simulate invalid CacheType
        assert!(cache.is_err(), "Expected error for invalid CacheType");
    }

    #[test]
    fn test_mem_cache() {
        let mut cache = MemCache::new();
        cache.set("key1".to_string(), vec![1, 2, 3]);
        assert_eq!(cache.get("key1"), Some(&vec![1, 2, 3]));
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_disk_cache() {
        let mut cache = DiskCache::new();
        cache.set("key1".to_string(), vec![4, 5, 6]);
        assert_eq!(cache.get("key1"), Some(&vec![4, 5, 6]));
        assert_eq!(cache.get("key2"), None);
    }
}
