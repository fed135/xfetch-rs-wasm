extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::describe::WasmDescribe;
use lru::LruCache;
use xfetch::CacheEntry;
use std::time::Duration;

trait TLRU {
  fn new(&self, maxItems: u64, ttl: u64);
  fn put(&self, key: &str, value: &JsValue);
  fn get(&self, key: &str) -> JsValue;
  fn remove(&self, key: &str);
  fn clear(&self);
  fn count(&self) -> u64;
}

struct xFetchWASM {
  ttl: Duration,
  cache: LruCache,
}

impl TLRU for xFetchWASM {
  fn new(&self, maxItems: u64, ttl: u64) {
    self.cache = LruCache::new(maxItems.try_into().unwrap());
    self.ttl = Duration::from_millis(ttl);
  }
  
  fn put(&self, key: &str, value: &JsValue) -> () {
    self.cache.put(key, CacheEntry::builder({ value, self.ttl })
      .with_ttl(self.ttl)
      .build());
  }
  
  fn get(&self, key: &str) -> JsValue {
    return self.cache.get(key);
  }
  
  fn remove(&self, key: &str) -> () {
    self.cache.pop(key);
  }
  
  fn clear(&self) -> () {
    self.cache.clear();
  }

  fn count(&self) -> u64 {
    return self.cache.len();
  }
}

impl IntoWasmAbi for xFetchWASM {
  fn into_abi(self) -> JsValue { self as JsValue }
}

impl WasmDescribe for xFetchWASM {
  fn describe() -> JsValue { None as JsValue }
}

#[wasm_bindgen]
pub fn main(maxItems: u64, ttl: u64) -> xFetchWASM {
  let c = xFetchWASM{};
  c.new(maxItems, ttl);
  return c;
}
