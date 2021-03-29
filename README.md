# SixArm.com collections for b-trees and hashes

This crate provides two general-purpose collections helpers:

* BTreeMapToSet<K, V> is based on BTreeMap<K, BTreeSet<V>>

* HashMapToSet<K, V> is based on HashMap<K, HashSet<V>>

This crate provides two specific-purpose collections helpers:

* BTreeMapOfFileLenToSetOfPathBuf is based on BTreeMap<u64, BTreeSet<PathBuf>>

* HashMapOfFileLenToSetOfPathBuf is based on HashMap<u64, HashSet<PathBuf>>

The helpers are implemented as trait extensions i.e. the helpers add 
functions to existing Rust std::collections code.


## Tracking

Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>

License: GPL-2.0 or Apache-2.0 or MIT