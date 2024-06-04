
mod plant_structure;
mod spores;

use spores::test_spores;

fn main() {
    println!("Hello, world!");
	test_spores();
	plant_structure::leaves::test_leaves();
}
