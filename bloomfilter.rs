use std::hash::{Hash, Hasher, SipHasher};
use std::vec::Vec;

pub struct Bloom{
    pub vec: Vec<bool>
    , pub hashes: u32
}

impl Bloom {
	pub fn new(prob: f64, capacity: i32) -> Bloom{
		let size = (-(capacity as f64 * prob.ln())/(2_f64.ln().powi(2))) as u64;
		let hashes = ((size as f64*2_f64.ln())/capacity as f64) as u32;
        let mut vec = Vec::new();
        for _ in 0..size{
            vec.push(false)
        }
        return Bloom{vec: vec, hashes: hashes}
	}

    pub fn insert(&mut self, value: String){
        let hash_val = hash(value) % self.vec.len() as u64;
        self.vec[hash_val as usize] = true;
    }

    pub fn contains(&self, value: String) -> bool {
        let hash_val = hash(value) % self.vec.len() as u64;
        self.vec[hash_val as usize] == true
    }
}

pub fn hash<T>(obj: T) -> u64 where T: Hash {
    let mut hasher = SipHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

pub fn main(){
    println!("Hello, world!");
}