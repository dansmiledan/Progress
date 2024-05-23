fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {

	#[test]
	fn test_vector_of_string() {
		let bits = vec!["hello", "world"];
		assert_eq!(bits.concat(), "helloworld");
		assert_eq!(bits.join(","), "helloworld");
		// assert_eq!(bits.join(","), ());
	}
}
