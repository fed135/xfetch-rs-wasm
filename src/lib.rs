extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::IntoWasmAbi;
use lru::LruCache;
use xfetch::CacheEntry;
use std::time::Duration;

trait TLRU {
  fn put(&self, key: &str, value: &JsValue);
  fn get(&self, key: &str) -> JsValue;
  fn remove(&self, key: &str);
  fn clear(&self);
  fn count(&self) -> u64;
}

struct XFetchWASM {
  ttl: Duration,
  cache: LruCache<isize, char>,
}

impl Default for XFetchWASM {
  fn default () -> XFetchWASM {
    XFetchWASM{cache: LruCache::new(0), ttl: Duration::from_millis(0)}
  }
}

impl TLRU for XFetchWASM {
  fn put(&self, key: &str, value: &JsValue) -> () {
    self.cache.put(key, CacheEntry::builder{value: value, ttl: self.ttl}
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

impl IntoWasmAbi for XFetchWASM {
  type Abi: WasmAbi;
  fn into_abi(self, extra: &mut Stack) -> Self::Abi;
}

#[wasm_bindgen]
pub fn main(maxItems: u64, ttl: u64) -> XFetchWASM {
  let c = XFetchWASM{ttl: Duration::from_millis(ttl), ..Default::default()};
  c.cache.resize(maxItems);
  return c;
}
