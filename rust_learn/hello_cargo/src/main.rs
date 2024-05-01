fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &'a 'b str {
	if a.len() > b.len() {
		return a;
	} else {
		return b;
	}
}
fn main() {
	let string1 = String::from("long string is long");
	let r;
	{
		let string2 = String::from("xyz");
		r = longest(string1.as_str(), string2.as_str());
	}
		println!("The longest string is {}", r);
}
