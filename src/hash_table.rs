

pub trait Identifiable {
    fn id(&self) -> u32;
}


impl Identifiable for i32 {
    fn id(&self) -> u32 {
        *self as u32
    }
}

impl Identifiable for u32 {
    fn id(&self) -> u32 {
        *self 
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

    pub fn occupancy(&self) -> usize {
        self.buckets.iter().filter(|bucket| !bucket.is_empty()).count()
    }

    pub fn average_bucket_length(&self) -> f64 {
        let non_empty_buckets: Vec<&Vec<(K, V)>> = self.buckets.iter().filter(|bucket| !bucket.is_empty()).collect();
        
        if non_empty_buckets.is_empty() {
            return 0.0;
        }

        let total_length: usize = non_empty_buckets.iter().map(|bucket| bucket.len()).sum();
        total_length as f64 / non_empty_buckets.len() as f64
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


    pub fn search_non_mut(&self, key: &K) -> Option<&V>
    where
        K: Eq,
    {
        let bucket_index = self.hash(key);
        for (existing_key, existing_value) in &self.buckets[bucket_index] {
            if existing_key == key {
                return Some(existing_value);
            }
        }
        None
    }

}

