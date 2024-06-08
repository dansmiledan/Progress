use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

struct Person {
    name: String,
    birth: i32,
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn test_struct_in_box() {
        let mut composers = Vec::new();
        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });
        composers.push(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        });
        composers.push(Person {
            name: "Lully".to_string(),
            birth: 1632,
        });
        for composer in composers {
            println!("{}, born {}", composer.name, composer.birth);
        }
    }

	#[test]
	fn test_rc() {
		let s: Rc<String> = Rc::new("shirataki".to_string());
		let t = s.clone();
		let u = t.clone();
		assert!(s.contains("shira"));
		assert_eq!(t.find("taki"), Some(5));
		println!("print u: {}", u);
	}

	#[test]
	fn test_ref() {
		let x = 10;
		let r = &x;
		assert!(*r == x);
		assert!(*r == 10);

		let mut y = 10;
		let m = &mut y;
		*m = 64;
		assert!(y == 64);

		let x = 10;
		let mut r = &x;
		if *r > 10 {
			r = &y;
		}
		assert!(*r == 10);
	}
}
