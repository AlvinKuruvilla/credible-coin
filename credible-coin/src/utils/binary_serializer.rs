use bincode::serialize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) struct BinarySerializer;

impl BinarySerializer {
    pub(crate) fn serialize_to_file<K: Serialize, V: Serialize>(data: &HashMap<K, V>, path: &str) {
        let serialized = serialize(&data).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&serialized).unwrap();
    }

    pub(crate) fn deserialize_from_file<K, V>(path: &str) -> HashMap<K, V>
    where
        K: for<'de> Deserialize<'de> + std::cmp::Eq + std::hash::Hash,
        V: for<'de> Deserialize<'de>,
    {
        let file = File::open(path).unwrap();
        bincode::deserialize_from(file).unwrap()
    }
    pub(crate) fn path_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
}
