use lru::LruCache;
use xfetch::CacheEntry;
use std::time::Duration;

#[wasm_bindgen]
impl<T> TLRUCache<T> {

  pub fn new<F>(maxItems: u64, ttl: u64) -> TLRUCache<T> {
    self.cache = LruCache::new(maxItems);
    self.ttl = Duration::from_millis(ttl);
  }
  
  pub fn put(key: &str, value: &JsValue) -> () {
    self.cache.put(key, CacheEntry::builder()
        .with_ttl(self.ttl)
        .build());
  }
  
  pub fn get(key: &str) -> JsValue {
    return self.cache.get(key);
  }
  
  pub fn remove(key: &str) -> () {
    self.cache.pop(key);
  }
  
  pub fn clear() -> () {
    self.cache.clear();
  }

  pub fn count() -> u64 {
    return self.cache.len();
  }
}
