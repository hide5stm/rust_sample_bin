#[macro_use] extern crate itertools;


fn main() {
    for (i, j) in iproduct!(0..4, 0..4) {
        println!("{} {}", i, j);
    }
}
