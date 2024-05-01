fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
	if a.len() > b.len() {
		return a;
	} else {
		return b;
	}
}
fn main() {
	let string1 = String::from("long string is long");
	{
		let string2 = String::from("xyz");
	}
	let mut vec: Vec<i32> = Vec::new();
	vec.push(4);
	vec.push(3);
	vec.push(6);
	vec.sort();
	for v in vec {
		println!("{v}");
	}
}
