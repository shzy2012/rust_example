use super::log;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref HASHMAP: HashMap<u32, &'static str> = {
        log!("init hashmap");
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

#[test]
fn lazy_static_test() {
    let value1 = HASHMAP.get(&0).unwrap();
    assert_eq!(&"foo", value1);

    let value2 = HASHMAP.get(&1).unwrap();
    assert_eq!(&"bar", value2);
}
