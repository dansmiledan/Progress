use std::mem;

/// This is main function
fn main() {
    println!("Hello, world!");
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // 美化打印
    println!("{:#?}", peter);
    let a: u8 = -2i8 as u8;
    println!("{}", a);
    println!("{}", mem::size_of_val(&a));
    for i in 1..23 {
        println!("{}", i);
    }
    let array1 = [1, 2, 3];
    println!("{}", array1.iter().any(|&x| x % 2 == 3));
    let upper = 1000;
    let square_sum = (0..)
        .map(|x| x * x)
        .take_while(|&n| n < upper)
        .filter(|x| x % 2 == 1)
        .fold(0, |sum, i| sum + i);
	println!("{square_sum}");
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

/**
 * format print test
 */
#[test]
fn test_format_print() {
    println!("{} of {:b} people knows binary", 1, 2);
    let pi = 3.141592653;
    println!("{:.width$}", pi, width = 3);
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    #[derive(Debug)]
    struct Structure(u32);
    println!("{:#?} ", Structure(24));
}
