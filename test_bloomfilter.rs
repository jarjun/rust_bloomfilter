mod bloomfilter;

#[test]
fn test_hash_on_int(){
	println!("{}", bloomfilter::hash(5));
}

#[test]
fn test_hash_on_string(){
	println!("{}", bloomfilter::hash("string"));
}

#[test]
fn test_bloom_new() {
    let bloom = bloomfilter::Bloom::new(0.01, 1000);
    println!("{:?}, {}", bloom.vec, bloom.hashes);
}

#[test]
fn test_contains_good_value() {
    let mut bloom = bloomfilter::Bloom::new(0.01, 1000);
    bloom.insert("test".to_string());
    
    let contains = bloom.contains("test".to_string());
    
    if contains != true {
        panic!("contain not TRUE");
    }
}

#[test]
fn test_contains_bad_value() {
    let mut bloom = bloomfilter::Bloom::new(0.01, 1000);
    bloom.insert("test".to_string());

    let contains = bloom.contains("nope".to_string());

    if contains == true {
        panic!("contains should be FALSE");
    }
}
