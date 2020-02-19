use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const NUM_BUCKETS: usize = 16;

type Bucket = Vec<(String, String)>;

pub struct HashMap {
  buckets: Vec<Bucket>,
}

impl HashMap {
  pub fn new() -> Self {
    let mut buckets = Vec::new();
    for _ in 0..NUM_BUCKETS {
      buckets.push(Bucket::new());
    }
    Self { buckets }
  }

  pub fn find_bucket(key: &String) -> usize {
    let mut hasher = DefaultHasher::new();
    hasher.write(key.as_bytes());
    let hash = hasher.finish();

    (hash % NUM_BUCKETS as u64) as usize
  }

  pub fn insert(&mut self, key: &str, value: &str) -> Option<&str> {
    let maybe_value = self.get(key);
    if maybe_value != None {
      return maybe_value;
    } else {
      let key2 = String::from(key);
      let bucket_index = HashMap::find_bucket(&key2);
      // if we didn't find it, push it
      let value2 = String::from(value);
      self.buckets[bucket_index].push((key2, value2));
      None
    }

    // for i in 0..self.buckets[bucket_index].len() {
    //   let (k, v) = &self.buckets[bucket_index][i];
    //   if k.eq(&key2) {
    //     // found it!
    //     return Some(&v.as_str());
    //   }
    // }
    // for elt in self.buckets[bucket_index].iter() {
    //   let (k, v) = elt;
    //   if k.eq(&key2) {
    //     // found it!
    //     return Some(&v.as_str());
    //   }
    // }

    // None
  }

  pub fn get(&self, key: &str) -> Option<&str> {
    // Hash value
    let key2 = String::from(key);
    let bucket_index = HashMap::find_bucket(&key2);
    // Go thru vector
    for elt in &self.buckets[bucket_index] {
      let (k, v) = elt;
      if k.eq(&key2) {
        return Some(&v.as_str());
      }
    }
    None
  }

  pub fn remove(&mut self, key: &str) -> Option<&str> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    // create a new HashMap
    let mut map = HashMap::new();

    // insert key/value pairs into the HashMap
    assert_eq!(map.insert("foo", "bar"), None);
    // if an item already exists for that value, it should return the old value
    assert_eq!(map.insert("foo", "lol"), Some("bar"));

    // get a value based on its key
    assert_eq!(map.get(&"foo"), Some(&"lol"));
    // you should be able to do this multiple times
    assert_eq!(map.get(&"foo"), Some(&"lol"));
    // if no value exists for a key, return None
    assert_eq!(map.get(&"qux"), None);

    // remove a value for a key
    assert_eq!(map.remove(&"foo"), Some("lol"));
    // once it no longer exists in the map, it should return None
    assert_eq!(map.get(&"foo"), None);
  }
}
