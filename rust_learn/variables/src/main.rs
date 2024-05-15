fn gcd(mut n: u64, mut m: u64) -> u64 {
	while m != 0 {
		if m < n {
			let t = m;
			m = n;
			n = t;
		}
		m %= n;
	}
	n
}

fn main() {
	println!("{:?}", gcd(12, 4));
}

#[test]
fn test_gcd() {
	assert_eq!(gcd(14, 15), 1);
}