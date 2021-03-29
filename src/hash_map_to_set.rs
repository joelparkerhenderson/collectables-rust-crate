use std::collections::{HashMap, HashSet};
use std::cmp::Eq;
use std::hash::Hash;

pub type HashMapToSet<K, V> = HashMap<K, HashSet<V>>;

pub trait HashMapToSetExt<K, V> {

    fn sub_contains(&self, key: &K, value: &V) -> bool
    where
        K: Hash + Eq,
        V: Hash + Eq;

    fn sub_insert(&mut self, key: K, value: V) -> bool
    where
        K: Hash + Eq,
        V: Hash + Eq;
  
    fn sub_remove(&mut self, key: &K, value: &V) -> bool
    where
        K: Hash + Eq,
        V: Hash + Eq;

}

impl<K, V> HashMapToSetExt<K, V> for HashMapToSet<K, V> {
    
    /// Return `true` if the collection contains a sub-key-value item.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sixarm_collections::*;
    /// let mut collection: HashMapToSet<u8, u8> = HashMapToSet::new();
    /// collection.sub_insert(1, 2);
    /// assert_eq!(subject.sub_contains(&1, &2), true);
    /// assert_eq!(subject.sub_contains(&3, &4), false);
    /// ```
    #[inline]
    fn sub_contains(&self, key: &K, value: &V) -> bool
    where
        K: Hash + Eq,
        V: Hash + Eq,
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
    /// let mut collection: HashMapToSet<u8, u8> = HashMapToSet::new();
    /// collection.sub_insert(1, 2);
    /// assert_eq!(subject.sub_contains(&1, &2), true);
    /// ```
    #[inline]
    fn sub_insert(&mut self, key: K, value: V) -> bool    
    where
        K: Hash + Eq,
        V: Hash + Eq,
    {
        self.entry(key)
        .or_insert(HashSet::new())
        .insert(value)
    }

    /// Remove a sub-key-value pair from the collection.
    ///
    /// Return whether the value was present in the set.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sixarm_collections::*;
    /// let mut collection: HashMapToSet<u8, u8> = HashMapToSet::new();
    /// collection.sub_insert(1, 2);
    /// assert_eq!(subject.sub_contains(&1, &2), true);
    /// collection.sub_remove(&1, &2);
    /// assert_eq!(subject.sub_contains(&1, &2), false);
    /// ```
    #[inline]
    fn sub_remove(&mut self, key: &K, value: &V) -> bool 
    where
        K: Hash + Eq,
        V: Hash + Eq,
    {
        match self.get_mut(key) {
            Some(set) => set.remove(&value),
            None => false,
        }
    }

}

#[cfg(test)]
//#[macro_use] extern crate assert_matches;
mod tests {

    use super::*;
    use crate::hash_map_to_set::HashMapToSet;
    use std::iter::IntoIterator;

    fn assert_hash_set_eq<T: std::fmt::Debug>(a: HashSet<T>, b: HashSet<T>) {
        assert_eq!(a.len(), b.len());
        a.iter().for_each(|x| assert!(b.contains(x)));
    } 

    fn assert_vec_set_eq<T: std::fmt::Debug>(a: Vec<&T>, b: Vec<&T>) {
        assert_eq!(a.len(), b.len());
        let a_set = HashSet::<T>::new();
        let b_set = HashSet::<T>::new();
        a.iter().for_each(|x| a_set.insert(&x));
        b.iter().for_each(|x| b_set.insert(&x));
        assert_eq!(a_set, b_set);
    } 

    fn assert_iter_set_eq<T: std::fmt::Debug, I: IntoIterator<Item = T>>(a: I, b: I) {
        let a_set = HashSet::<T>::new();
        let b_set = HashSet::<T>::new();
        a.into_iter().for_each(|x| a_set.insert(&x));
        b.into_iter().for_each(|x| b_set.insert(&x));
        assert_eq!(a_set, b_set);
    }

    #[test]
    /// Test `sub_contains` with some items.
    fn test_sub_contains() {
        let mut subject: HashMapToSet<u8, u8> = HashMapToSet::new();
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
        let mut subject: HashMapToSet<u8, u8> = HashMapToSet::new();
        let k1 = 1;
        let k2 = 2;
        let v1 = 3;
        let v2 = 4;
        let v3 = 5;
        let v4 = 7;
        // Item 1
        assert_eq!(subject.sub_insert(k1, v1), true);


        let mut keys = subject.keys().collect::<Vec<_>>();
        let values = subject.get(&k1).unwrap().into_iter().collect::<Vec<_>>();


        assert_iter_set_eq(subject.keys(), vec![&k1]);
        assert_iter_set_eq(subject.get(&k1).unwrap(), HashSet::from_iter(vec![&v1]));
        // Item 2
        assert_eq!(subject.sub_insert(k1, v2), true);
        assert_iter_set_eq(subject.keys().collect::<Vec<_>>(), vec![&k1]);  
        assert_iter_set_eq(subject.get(&k1).unwrap().into_iter().collect::<Vec<_>>(), vec![&v1, &v2]);
        // Item 3
        assert_eq!(subject.sub_insert(k2, v3), true);
        assert_iter_set_eq(subject.keys().collect::<Vec<_>>(), vec![&k1, &k2]);  
        assert_iter_set_eq(subject.get(&k1).unwrap().into_iter().collect::<Vec<_>>(), vec![&v1, &v2]);
        assert_iter_set_eq(subject.get(&k2).unwrap().into_iter().collect::<Vec<_>>(), vec![&v3]);
        // Item 4
        assert_eq!(subject.sub_insert(k2, v4), true);
        assert_iter_set_eq(subject.keys().collect::<Vec<_>>(), vec![&k1, &k2]);
        assert_iter_set_eq(subject.get(&k1).unwrap().into_iter().collect::<Vec<_>>(), vec![&v1, &v2]);
        assert_iter_set_eq(subject.get(&k2).unwrap().into_iter().collect::<Vec<_>>(), vec![&v3, &v4]);
    }

    #[test]
    /// Test `remove` with a present item.
    fn test_sub_remove_x_present_item() {
        let mut subject: HashMapToSet<u8, u8> = HashMapToSet::new();
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
        let mut subject: HashMapToSet<u8, u8> = HashMapToSet::new();
        let k = 1;
        let v = 2;
        let z = 3;
        assert_eq!(subject.sub_remove(&k, &z), false);
        assert_eq!(subject.sub_remove(&z, &v), false);
    }

}
