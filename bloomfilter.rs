
use std::hash::{Hash, Hasher, SipHasher};

struct Bloom{
    x: i32
    , array: [i32; 1024]
}

fn main() {
    println!("Hello, world!");
}

impl Bloom {
    fn insert(&self){

    }
}

pub fn hash<T>(obj: T) -> u64 where T: Hash{
    let mut hasher = SipHasher::new();
    obj.hash(&mut hasher);
    return hasher.finish()
}