use log::debug;
use odht::{Config, FxHashFn, HashTableOwned};
use std::collections::HashSet;

struct OdhtConfig;

const CHUNK_SIZE: usize = 127;

impl Config for OdhtConfig {
    type Key = [u8; CHUNK_SIZE + 1];
    type Value = bool;

    type EncodedKey = [u8; CHUNK_SIZE + 1];
    type EncodedValue = [u8; 1];

    type H = FxHashFn;

    #[inline]
    fn encode_key(k: &Self::Key) -> Self::EncodedKey {
        *k
    }
    #[inline]
    fn encode_value(v: &Self::Value) -> Self::EncodedValue {
        [if *v { 1 } else { 0 }; 1]
    }
    #[inline]
    fn decode_key(k: &Self::EncodedKey) -> Self::Key {
        *k
    }
    #[inline]
    fn decode_value(v: &Self::EncodedValue) -> Self::Value {
        v[0] == 1
    }
}

pub struct Cache {
    memo: HashSet<String>,
    disk: Option<HashTableOwned<OdhtConfig>>,
    memo_limit: usize,
    memo_size: usize,
}

impl Cache {
    pub fn new(memo_limit: usize) -> Self {
        Self {
            memo: HashSet::new(),
            disk: None,
            memo_limit: if memo_limit == 0 {
                usize::MAX
            } else {
                memo_limit
            },
            memo_size: 0,
        }
    }

    pub fn insert(&mut self, item: &str) -> bool {
        if self.memo_size >= self.memo_limit {
            debug!("Memory cache is full, dump to disk");
            self.dump_to_disk();
        }

        let mut res = self.memo.insert(item.to_owned());
        if res {
            self.memo_size = self.memo_size + item.len();
            if self.disk.is_some() {
                res = self.insert_on_disk(item);
                debug!("Insert on disk: {}", res);
            }
        }

        res
    }

    pub fn contains(&self, item: &str) -> bool {
        if self.memo.contains(item) {
            return true;
        }

        return if let Some(ref disk) = self.disk {
            Cache::item_to_keys(item).all(|key| disk.contains_key(&key))
        } else {
            false
        };
    }

    fn insert_on_disk(&mut self, item: &str) -> bool {
        let disk = self.disk.get_or_insert_with(|| {
            debug!("Create new disk cache");
            HashTableOwned::<OdhtConfig>::with_capacity(1_000_000, 95)
        });
        let mut res = false;
        for key in Cache::item_to_keys(item) {
            res = disk.insert(&key, &true).is_none() || res;
        }
        res
    }

    fn item_to_keys<'a>(item: &'a str) -> impl Iterator<Item = [u8; CHUNK_SIZE + 1]> + 'a {
        let res = item
            .as_bytes()
            .chunks(CHUNK_SIZE)
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                let mut key = [0 as u8; CHUNK_SIZE + 1];
                key[CHUNK_SIZE] = i as u8;
                key[..chunk.len()].copy_from_slice(chunk);
                key
            });
        return res;
    }

    fn dump_to_disk(&mut self) {
        let keys = self.memo.drain().collect::<Vec<_>>();
        for key in keys {
            self.insert_on_disk(&key);
        }
        self.memo_size = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_basic_cache() {
        let mut cache = Cache::new(0);
        assert!(cache.insert("hello"));
        assert!(cache.insert("world"));

        assert!(cache.contains("hello"));
        assert!(cache.contains("world"));
        assert!(!cache.contains("other"));
    }

    #[test]
    fn test_limit_memory() {
        let mut cache = Cache::new(1024);
        for _ in 0..100 {
            cache.insert(&rand_string(32));
        }
        assert!(cache.memo.len() < 100);
        assert!(cache.disk.is_some());
        assert!(cache.disk.unwrap().len() > 0);
    }

    fn rand_string(len: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}
