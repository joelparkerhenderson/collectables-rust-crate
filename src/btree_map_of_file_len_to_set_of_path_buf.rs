use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

pub type BTreeMapOfFileLenToSetOfPathBuf = BTreeMap<u64, BTreeSet<PathBuf>>;

pub trait BTreeMapOfFileLenToSetOfPathBufExt {
    fn sub_contains_path(&self, value: &PathBuf) -> bool;
    fn sub_insert_path(&mut self, value: PathBuf) -> bool;
    fn sub_remove_path(&mut self, value: PathBuf) -> bool;
}

impl BTreeMapOfFileLenToSetOfPathBufExt for BTreeMapOfFileLenToSetOfPathBuf {
    
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
    /// let mut collection: BTreeMapOfFileLenToSetOfPathBuf<FileLen, SetOfPathBuf> = BTreeMapOfFileLenToSetOfPathBuf::new();
    /// let a = PathBuf("alpha.txt");
    /// let b = PathBuf("bravo.txt");
    /// collection.sub_insert_path(a);
    /// assert_eq!(subject.sub_contains_path(a), true);
    /// assert_eq!(subject.sub_contains_path(b), false);
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
    /// let mut collection: BTreeMapOfFileLenToSetOfPathBuf<FileLen, SetOfPathBuf> = BTreeMapOfFileLenToSetOfPathBuf::new();
    /// let a = PathBuf("alpha.txt");
    /// collection.sub_insert_path(a);
    /// assert_eq!(subject.sub_contains_path(a), true);
    /// ```
    #[inline]
    fn sub_insert_path(&mut self, value: PathBuf) -> bool {
        let key = fs::metadata(&value).expect("metadata").len();
        self.entry(key)
        .or_insert(BTreeSet::new())
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
    /// let mut collection: BTreeMapOfFileLenToSetOfPathBuf<FileLen, SetOfPathBuf> = BTreeMapOfFileLenToSetOfPathBuf::new();
    /// let a = PathBuf("alpha.txt");
    /// collection.sub_insert_path(a);
    /// assert_eq!(subject.sub_contains_path(&a), true);
    /// collection.sub_remove_path(&a);
    /// assert_eq!(subject.sub_contains_path(&a), false);
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
//#[macro_use] extern crate assert_matches;
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    /// Test `contains` with a true result.
    /// Must succeed.
    /// 
    fn test_contains_path_x_true() {
        let mut collection: BTreeMapOfFileLenToSetOfPathBuf = BTreeMapOfFileLenToSetOfPathBuf::new();
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "alpha.txt"].iter().collect::<PathBuf>();
        let b: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "bravo.txt"].iter().collect::<PathBuf>();
        collection.sub_insert_path(a);
        assert_eq!(subject.sub_contains_path(&b), true);
    }

    #[test]
    /// Test `contains` with a false result.
    /// Must succeed.
    /// 
    fn test_contains_path_x_false() {
        let mut collection: BTreeMapOfFileLenToSetOfPathBuf = BTreeMapOfFileLenToSetOfPathBuf::new();
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "alpha.txt"].iter().collect::<PathBuf>();
        assert_eq!(subject.sub_contains_path(&a), false);
    }

    #[test]
    /// Test `insert` with path.
    /// Must succeed.
    /// 
    fn test_insert_path() {
        let mut collection: BTreeMapOfFileLenToSetOfPathBuf = BTreeMapOfFileLenToSetOfPathBuf::new();
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "alpha.txt"].iter().collect::<PathBuf>();
        let len = 5;
        assert_eq!(subject.sub_insert_path(a), true);
        assert_eq!(subject.keys().collect(), [len]);
        assert_eq!(subject.get(&len).unwrap().contains(&a), true);
    }

    #[test]
    /// Test `remove` with path.
    /// Must succeed.
    /// 
    fn test_remove_path() {
        let mut collection: BTreeMapOfFileLenToSetOfPathBuf = BTreeMapOfFileLenToSetOfPathBuf::new();
        let a: PathBuf = [env!("CARGO_MANIFEST_DIR"), "test", "hash_map_of_file_len_to_set_of_path_buf", "alpha.txt"].iter().collect::<PathBuf>();
        let len = 5;
        assert_eq!(subject.sub_insert_path(a), true);
        assert_eq!(subject.sub_remove_path(a), true);
        assert_eq!(subject.keys().collect(), [len]);
        assert_eq!(subject.get(&len).unwrap().is_empty(), true);
    }

}
