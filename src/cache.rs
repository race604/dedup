use std::collections::HashSet;

pub struct Cache<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    cache: HashSet<T>,
}

impl<T> Cache<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub fn new() -> Self {
        Self {
            cache: HashSet::new(),
        }
    }

		pub fn insert(&mut self, item: T) -> bool {
				self.cache.insert(item)
		}

		pub fn contains(&self, item: &T) -> bool {
				self.cache.contains(item)
		}
}
