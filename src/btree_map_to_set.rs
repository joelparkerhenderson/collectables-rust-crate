use std::collections::{BTreeMap, BTreeSet};
use std::cmp::Ord;

pub type BTreeMapToSet<K, V> = BTreeMap<K, BTreeSet<V>>;

pub trait BTreeMapToSetExt<K, V> {

    fn sub_contains(&self, key: &K, value: &V) -> bool
    where
        K: Ord,
        V: Ord;

    fn sub_insert(&mut self, key: K, value: V) -> bool
    where
        K: Ord,
        V: Ord;
  
    fn sub_remove(&mut self, key: &K, value: &V) -> bool
    where
        K: Ord,
        V: Ord;

}

impl<K, V> BTreeMapToSetExt<K, V> for BTreeMapToSet<K, V> {
    
    /// Return `true` if the collection contains a sub-key-value item.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`BTree`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sixarm_collections::*;
    /// let mut a: BTreeMapToSet<u8, u8> = BTreeMapToSet::new();
    /// a.sub_insert(1, 2);
    /// assert_eq!(a.sub_contains(&1, &2), true);
    /// assert_eq!(a.sub_contains(&3, &4), false);
    /// ```
    #[inline]
    fn sub_contains(&self, key: &K, value: &V) -> bool
    where
        K: Ord,
        V: Ord,
    {
        match self.get(key) {
            Some(set) => set.contains(value),
            None => false,
        }
    }

    /// Add a sub-key-value item to the collection.
    ///
    /// Return whether the item is added in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use sixarm_collections::*;
    /// let mut a: BTreeMapToSet<u8, u8> = BTreeMapToSet::new();
    /// a.sub_insert(1, 2);
    /// assert_eq!(a.sub_contains(&1, &2), true);
    /// ```
    #[inline]
    fn sub_insert(&mut self, key: K, value: V) -> bool    
    where
        K: Ord,
        V: Ord,
    {
        self.entry(key)
        .or_insert(BTreeSet::new())
        .insert(value)
    }

    /// Remove a sub-key-value pair from the collection.
    ///
    /// Return whether the value was present in the set.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Ord`] on the borrowed form *must* match those for the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sixarm_collections::*;
    /// let mut a: BTreeMapToSet<u8, u8> = BTreeMapToSet::new();
    /// a.sub_insert(1, 2);
    /// assert_eq!(a.sub_contains(&1, &2), true);
    /// a.sub_remove(&1, &2);
    /// assert_eq!(a.sub_contains(&1, &2), false);
    /// ```
    #[inline]
    fn sub_remove(&mut self, key: &K, value: &V) -> bool 
    where
        K: Ord,
        V: Ord,
    {
        match self.get_mut(key) {
            Some(set) => set.remove(&value),
            None => false,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use sixarm_assert::*;
    use crate::btree_map_to_set::BTreeMapToSet;

    #[test]
    /// Test `sub_contains` with some items.
    fn test_sub_contains() {
        let mut subject: BTreeMapToSet<u8, u8> = BTreeMapToSet::new();
        let k = 1;
        let v = 2;
        let absent = 3;
        assert_eq!(subject.sub_insert(k, v), true);
        assert_eq!(subject.sub_contains(&k, &v), true);
        assert_eq!(subject.sub_contains(&k, &absent), false);
        assert_eq!(subject.sub_contains(&absent, &v), false);
    }
    #[test]
    /// Test `sub_insert` with some items.
    fn test_sub_insert() {
        let mut subject: BTreeMapToSet<u8, u8> = BTreeMapToSet::new();
        let k1 = 1;
        let k2 = 2;
        let v1 = 3;
        let v2 = 4;
        let v3 = 5;
        let v4 = 7;
        // Item 1
        assert_eq!(subject.sub_insert(k1, v1), true);
        //assert_set_eq!(subject.keys(), [k1]);
        assert_set_eq!(subject.get(&k1).unwrap(), [v1]);
        // Item 2
        assert_eq!(subject.sub_insert(k1, v2), true);
        //assert_set_eq!(subject.keys(), [k1]);  
        assert_set_eq!(subject.get(&k1).unwrap(), [v1, v2]);
        // Item 3
        assert_eq!(subject.sub_insert(k2, v3), true);
        //assert_set_eq!(subject.keys(), [k1, k2]);  
        assert_set_eq!(subject.get(&k1).unwrap(), [v1, v2]);
        assert_set_eq!(subject.get(&k2).unwrap(), [v3]);
        // Item 4
        assert_eq!(subject.sub_insert(k2, v4), true);
        //assert_set_eq!(subject.keys(), [k1, k2]);
        assert_set_eq!(subject.get(&k1).unwrap(), [v1, v2]);
        assert_set_eq!(subject.get(&k2).unwrap(), [v3, v4]);
    }

    #[test]
    /// Test `remove` with a present item.
    fn test_sub_remove_x_present_item() {
        let mut subject: BTreeMapToSet<u8, u8> = BTreeMapToSet::new();
        let k = 1;
        let v = 2;
        assert_eq!(subject.sub_insert(k, v), true);
        assert_eq!(subject.sub_contains(&k, &v), true);
        assert_eq!(subject.sub_remove(&k, &v), true);
        assert_eq!(subject.sub_contains(&k, &v), false);
    }

    #[test]
    /// Test `remove` with an absent item.
    fn test_sub_remove_x_absent_item() {
        let mut subject: BTreeMapToSet<u8, u8> = BTreeMapToSet::new();
        let k = 1;
        let v = 2;
        let z = 3;
        assert_eq!(subject.sub_remove(&k, &z), false);
        assert_eq!(subject.sub_remove(&z, &v), false);
    }

}
