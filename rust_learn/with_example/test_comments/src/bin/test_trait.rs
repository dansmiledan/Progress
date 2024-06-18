struct Empty;
struct Null;

trait DoubleDrop<T> {
    // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事。
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
	fn double_drop(self, _: T) {
		
	}
}

fn main() {
	println!("hello world");
	let empty = Empty;
	let null = Null;
	empty.double_drop("hl");
	8i32.double_drop(_);
}