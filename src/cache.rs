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
    disk_used: bool,
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
            disk_used: false,
        }
    }

    pub fn insert(&mut self, item: &str) -> bool {
        let mut res = self.memo.insert(item.to_owned());
        if res {
            self.memo_size = self.memo_size + item.len();
            if self.disk_used {
                res = self.insert_on_disk(item);
                debug!("Insert on disk: {}", res);
            }

            if self.memo_size >= self.memo_limit {
                debug!("Memory cache is full, dump to disk");
                self.dump_to_disk();
            }
        }

        res
    }

    fn insert_on_disk(&mut self, item: &str) -> bool {
        let disk = self.disk.get_or_insert_with(|| {
            debug!("Create new disk cache");
            HashTableOwned::<OdhtConfig>::with_capacity(1_000_000, 95)
        });
        let mut res = false;
        for (i, chunk) in item.as_bytes().chunks(CHUNK_SIZE).into_iter().enumerate() {
            let mut key = [0 as u8; CHUNK_SIZE + 1];
            key[CHUNK_SIZE] = i as u8;
            key[..chunk.len()].copy_from_slice(chunk);
            res = disk.insert(&key, &true).is_none() || res;
        }
        res
    }

    fn dump_to_disk(&mut self) {
        self.disk_used = true;
        let keys = self.memo.drain().collect::<Vec<_>>();
        for key in keys {
            self.insert_on_disk(&key);
        }
        self.memo_size = 0;
    }
}
