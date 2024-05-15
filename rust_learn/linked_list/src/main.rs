struct Node {
	data: i32,
	next: Option<Box<Node>>
}

struct List {
	head: Box<Node>
}

impl List {
	fn add(&mut self, data: i32) {
		let mut tmp = Box::new(Node {
			data: data,
			next: Option::None
		});
		tmp.next = self.head.next.take();
		self.head.next = Some(tmp);
	}

	fn print(&self) {
		let mut tmp = self.head.next.as_deref();
		while let Some(node) = tmp {
			let v = node.data;
			println!(" {v} ");
			tmp = node.next.as_deref();
		}
	}

	fn build() -> Self {
		List {
			head: Box::new(Node {
				data: 0,
				next: Option::None
			})
		}
	}
}
fn main() {
	let mut list = List::build();
	list.add(1);
	list.add(3);
	list.print();
	list.print();
    println!("Hello, world!");
}
