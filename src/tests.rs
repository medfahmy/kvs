use crate::*;

#[test]
fn get_stored_value() {
    let mut store = KvStore::new();

    store.set("key1".to_string(), "value1".to_string());
    store.set("key2".to_string(), "value2".to_string());

    assert_eq!(store.get("key1".to_string()), Some("value1".to_string()));
    assert_eq!(store.get("key2".to_string()), Some("value2".to_string()));
}

#[test]
fn overwrite_value() {
    let mut store = KvStore::new();

    store.set("key1".to_string(), "value1".to_string());
    assert_eq!(store.get("key1".to_string()), Some("value1".to_string()));

    store.set("key1".to_string(), "value2".to_string());
    assert_eq!(store.get("key1".to_string()), Some("value2".to_string()));
}

#[test]
fn get_non_existent_value() {
    let mut store = KvStore::new();

    store.set("key1".to_string(), "value1".to_string());
    assert_eq!(store.get("key2".to_string()), None);
}

#[test]
fn remove_key() {
    let mut store = KvStore::new();

    store.set("key1".to_string(), "value1".to_string());
    store.remove("key1".to_string());
    assert_eq!(store.get("key1".to_string()), None);
}