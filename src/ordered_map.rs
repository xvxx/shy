//! HashMap that maintains its insertion order when iterating.

use std::{collections::HashMap, ops::Deref};

struct OrderedMap<K, V> {
    pub keys: Vec<K>,
    pub values: HashMap<K, V>,
}

impl<K, V> OrderedMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// Create a new OrderedMap.
    pub fn new() -> OrderedMap<K, V> {
        OrderedMap {
            keys: vec![],
            values: HashMap::new(),
        }
    }

    /// Insert a key/value pair.
    pub fn insert(&mut self, key: K, val: V) {
        self.keys.push(key.clone());
        self.values.insert(key, val);
    }

    /// Remove a key/value pair.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        for (i, k) in self.keys.iter().enumerate() {
            if k == key {
                self.keys.remove(i);
                break;
            }
        }
        self.values.remove(key)
    }
}

impl<K, V> Deref for OrderedMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

struct OrderedMapIterator<'m, K, V> {
    map: &'m OrderedMap<K, V>,
    curr: usize,
}

impl<'m, K, V> Iterator for OrderedMapIterator<'m, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    type Item = (&'m K, &'m V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.map.len() {
            None
        } else {
            let key = &self.map.keys[self.curr];
            if let Some(val) = self.map.values.get(&key) {
                self.curr += 1;
                Some((&key, val))
            } else {
                None
            }
        }
    }
}

impl<'m, K, V> IntoIterator for &'m OrderedMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    type Item = (&'m K, &'m V);
    type IntoIter = OrderedMapIterator<'m, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedMapIterator {
            map: &self,
            curr: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        let mut map = OrderedMap::<String, String>::new();
        map.insert("first".to_string(), "Bob".to_string());
        map.insert("last".to_string(), "Bobberson".to_string());
        map.insert("age".to_string(), "100".to_string());

        let mut vals: Vec<String> = vec!["Bob".into(), "Bobberson".into(), "100".into()];
        for (_k, v) in &map {
            assert_eq!(&vals.remove(0), v);
        }
    }

    #[test]
    fn test_map_functions() {
        let mut map = OrderedMap::<String, usize>::new();
        map.insert("bob".to_string(), 33);
        map.insert("roger".to_string(), 40);
        map.insert("annie".to_string(), 100);

        assert_eq!(3, map.len());
        assert!(!map.is_empty());
        assert!(map.contains_key("roger"));
        assert!(!map.contains_key("dodger"));
    }

    #[test]
    fn test_remove() {
        let mut map = OrderedMap::<String, usize>::new();
        map.insert("one".to_string(), 1);
        map.insert("two".to_string(), 2);
        map.insert("three".to_string(), 3);
        map.insert("four".to_string(), 4);

        assert_eq!(map.len(), 4);
        map.remove(&"two".to_string());
        assert_eq!(map.len(), 3);
    }
}
