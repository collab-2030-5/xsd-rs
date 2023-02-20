use std::collections::btree_map::{IntoIter, Iter, Values};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Map<K, V>
where
    K: Debug + Display + std::cmp::Ord,
    V: Debug,
{
    inner: BTreeMap<K, V>,
}

impl<K, V> Default for Map<K, V>
where
    K: Debug + Display + std::cmp::Ord,
    V: Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Map<K, V>
where
    K: Debug + Display + Ord,
    V: Debug,
{
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn to_inner(self) -> BTreeMap<K, V> {
        self.inner
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(x) = self.inner.get(&key) {
            panic!(
                "  ******* Replacing (key == {}) {:#?} with {:#?}",
                key, x, value
            );
        }
        self.inner.insert(key, value);
    }

    pub fn values(&self) -> Values<'_, K, V> {
        self.inner.values()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.inner.iter()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: Debug + Display + Ord,
    V: Debug,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
