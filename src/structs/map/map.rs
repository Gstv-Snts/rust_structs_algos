#[derive(Debug)]
pub struct Map<K, V> {
    load_factor: f32,
    values: Vec<V>,
    keys: Vec<Key<K>>,
}

#[derive(Debug)]
struct Key<K> {
    key: K,
    value_index: usize,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map {
            values: Vec::new(),
            keys: Vec::new(),
            load_factor: 0.0,
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.values.push(value);
        self.keys.push(Key {
            key,
            value_index: self.values.len() - 1,
        })
    }
}
mod test {
    use super::Map;

    #[test]
    fn new() {
        let m: Map<usize, usize> = Map::new();
        assert_eq!(m.load_factor, 0.0);
        assert!(m.values.is_empty());
        assert!(m.keys.is_empty());
    }

    #[test]
    fn insert() {
        let mut m: Map<&'static str, usize> = Map::new();
        assert_eq!(m.load_factor, 0.0);
        assert!(m.values.is_empty());
        assert!(m.keys.is_empty());
        m.insert("potato", 23);
        m.insert("banana", 20);
        m.insert("apple", 10);
        m.insert("watermelon", 3);
        for i in m.values {
            println!("{:?}", i);
        }
        for i in m.keys {
            println!("{:?}", i);
        }
        assert!(false)
    }
}
