

pub trait Identifiable {
    fn id(&self) -> i32;
}


impl Identifiable for i32 {
    fn id(&self) -> i32 {
        *self
    }
}

impl Identifiable for u32 {
    fn id(&self) -> i32 {
        *self as i32
    }
}

#[derive(Default, Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    modulo: usize,
}

impl<K:Eq + Identifiable, V> HashMap <K, V> {
    pub fn new(modulo: usize) -> Self {
        let mut buckets = Vec::with_capacity(modulo);
        for _ in 0..modulo {
            buckets.push(Vec::new());
        }
        HashMap { buckets, modulo }
    }

    pub fn hash(&self, key: &K) -> usize {
        (key.id() as usize) % self.modulo
    }

    pub fn insert(&mut self, key: K, value: V) {
        let bucket = self.hash(&key);
        self.buckets[bucket].push((key, value));
    }

    pub fn search(&mut self, key: &K) -> Option<&mut V>
    where
        K: Eq,
    {
        let bucket_index = self.hash(key);
        for &mut (ref existing_key, ref mut existing_value) in &mut self.buckets[bucket_index] {
            if (*existing_key) == (*key) {
                return Some(existing_value);
            }
        }
        None
    }

}

