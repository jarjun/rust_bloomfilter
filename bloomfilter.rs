
use std::hash::{Hash, Hasher, SipHasher};

pub struct Bloom{
    pub size: u64
    , pub array: [i32; 1024]
}

pub fn main() {
    println!("Hello, world!");
}

impl Bloom {
    pub fn insert(&mut self, value: String){
        let hash_val = hash(value) % self.size;
        self.array[hash_val as usize] = 1;
    }

    pub fn contains(&self, value: String) -> bool {
        let hash_val = hash(value) % self.size;
        self.array[hash_val as usize] == 1
    }
}

///impl Default for Bloom {
///    fn default() -> Bloom {
///        Bloom {
///            size: 1024,
///            array: [0, ..1024]
///        }
///    }
///}

pub fn hash<T>(obj: T) -> u64 where T: Hash {
    let mut hasher = SipHasher::new();
    obj.hash(&mut hasher);
    return hasher.finish()
}
