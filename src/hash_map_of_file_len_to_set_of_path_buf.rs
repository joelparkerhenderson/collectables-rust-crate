use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

pub type HashMapOfFileLenToSetOfPathBuf = HashMap<u64, HashSet<PathBuf>>;

pub trait HashMapOfFileLenToSetOfPathBufExt {
    fn sub_contains_path(&self, value: &PathBuf) -> bool;
    fn sub_insert_path(&mut self, value: PathBuf) -> bool;
    fn sub_remove_path(&mut self, value: PathBuf) -> bool;
}

impl HashMapOfFileLenToSetOfPathBufExt for HashMapOfFileLenToSetOfPathBuf {
    
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
    /// use std::collections::{HashMap, HashSet};
    /// use std::path::PathBuf;
    ///
    /// let mut a: HashMapOfFileLenToSetOfPathBuf = HashMapOfFileLenToSetOfPathBuf::new();
    /// # std::fs::write("alpha.txt", "alpha");
    /// # std::fs::write("bravo.txt", "bravo");
    /// let alpha = PathBuf::from("alpha.txt");
    /// let bravo = PathBuf::from("bravo.txt");
    /// a.sub_insert_path(alpha.clone());
    /// assert_eq!(a.sub_contains_path(&alpha), true);
    /// assert_eq!(a.sub_contains_path(&bravo), false);
    /// # std::fs::remove_file("alpha.txt");
    /// # std::fs::remove_file("bravo.txt");
    /// ```
    #[inline]
    fn sub_contains_path(&self, value: &PathBuf) -> bool {
        let key = fs::metadata(&value).expect("metadata").len();
        match self.get(&key) {
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
    /// use std::collections::{HashMap, HashSet};
    /// use std::path::PathBuf;
    ///
    /// let mut a: HashMapOfFileLenToSetOfPathBuf = HashMapOfFileLenToSetOfPathBuf::new();
    /// # std::fs::write("alpha.txt", "alpha");
    /// let alpha = PathBuf::from("alpha.txt");
    /// a.sub_insert_path(alpha.clone());
    /// assert_eq!(a.sub_contains_path(&alpha), true);
    /// # std::fs::remove_file("alpha.txt");
    /// ```
    #[inline]
    fn sub_insert_path(&mut self, value: PathBuf) -> bool {
        let key = fs::metadata(&value).expect("metadata").len();
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
    /// use std::collections::{HashMap, HashSet};
    /// use std::path::PathBuf;
    ///
    /// let mut a: HashMapOfFileLenToSetOfPathBuf = HashMapOfFileLenToSetOfPathBuf::new();
    /// # std::fs::write("alpha.txt", "alpha");
    /// let alpha = PathBuf::from("alpha.txt");
    /// a.sub_insert_path(alpha.clone());
    /// assert_eq!(a.sub_contains_path(&alpha), true);
    /// a.sub_remove_path(alpha.clone());
    /// assert_eq!(a.sub_contains_path(&alpha), false);
    /// # std::fs::remove_file("alpha.txt");
    /// ```
    #[inline]
    fn sub_remove_path(&mut self, value: PathBuf) -> bool {
        let key = fs::metadata(&value).expect("metadata").len();
        match self.get_mut(&key) {
            Some(set) => set.remove(&value),
            None => false,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use sixarm_assert::*;
    use std::path::PathBuf;

    #[test]
    /// Test `sub_contains_path`.
    /// Must succeed.
    /// 
    fn test_sub_contains_path() {
        let mut subject: HashMapOfFileLenToSetOfPathBuf = HashMapOfFileLenToSetOfPathBuf::new();
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "alpha.txt"].iter().collect::<PathBuf>();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "bravo.txt"].iter().collect::<PathBuf>();
        assert!(subject.sub_insert_path(a.clone()));
        assert_eq!(subject.sub_contains_path(&a), true);
        assert_eq!(subject.sub_contains_path(&b), false);
    }

    #[test]
    /// Test `sub_insert_path`.
    /// Must succeed.
    /// 
    fn test_sub_insert_path() {
        let mut subject: HashMapOfFileLenToSetOfPathBuf = HashMapOfFileLenToSetOfPathBuf::new();
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "alpha.txt"].iter().collect::<PathBuf>();
        let len = 5;
        assert!(subject.sub_insert_path(a.clone()));
        //assert_set_eq!(subject.keys(), [len]);
        assert!(subject.get(&len).unwrap().contains(&a));
    }

    #[test]
    /// Test `sub_remove_path`.
    /// Must succeed.
    /// 
    fn test_sub_remove_path() {
        let mut subject: HashMapOfFileLenToSetOfPathBuf = HashMapOfFileLenToSetOfPathBuf::new();
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "alpha.txt"].iter().collect::<PathBuf>();
        let len = 5;
        assert!(subject.sub_insert_path(a.clone()));
        assert!(subject.sub_remove_path(a));
        //assert_set_eq!(subject.keys(), [len]);
        assert!(subject.get(&len).unwrap().is_empty());
    }

}
