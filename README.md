# SixArm.com collections Rust crate

This Rust crate provides two collections helpers:

* BTreeMapToSet<K, V> is based on BTreeMap<K, BTreeSet<V>>

* HashMapToSet<K, V> is based on HashMap<K, HashSet<V>>

The helpers are implemented as trait extensions i.e. the helpers add functions to existing Rust std::collections code.


## Tracking

Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>

License: GPL-2.0 or Apache-2.0 or MIT