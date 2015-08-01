mod bloomfilter;


#[test]
fn test_hash_on_int(){
	println!("{}", bloomfilter::hash(5));
}

#[test]
fn test_hash_on_string(){
	println!("{}", bloomfilter::hash("string"));
}