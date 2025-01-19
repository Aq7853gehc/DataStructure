const INITIAL: usize = 16;
#[allow(dead_code)]
pub struct HashSet {
    buckets: Vec<Vec<i32>>,
    size: usize,
}

impl HashSet {
    pub fn new(size:usize) -> Self {
        let buckets = vec![Vec::new(); size];
        HashSet { buckets, size: 0 }
    }

    fn hash(&self, key: &i32) -> usize {
        (*key as usize) % 10
    }

    pub fn insert(&mut self, key: i32) {
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        if !bucket.contains(&key) {
            bucket.push(key);
            self.size += 1;
        }
    }

    pub fn contains(&self, key: &i32) -> bool {
        let index = self.hash(key);
        let bucket = &self.buckets[index];

        bucket.contains(key)
    }

    pub fn remove(&mut self, key: &i32) -> bool {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];
        if let Some(pos) = bucket.iter().position(|x| x == key) {
            bucket.remove(pos);
            self.size -= 1;
            return true;
        }
        false
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
