mod math;
mod collections;
mod error;
mod generics;
mod traits;
mod lifetime;

fn main() {
    // let area = math::calculate_area(5);
    // println!("Area: {}", area);

    // collections::vec::vec_example_get();
    // collections::string::string_example_tostring();
    // collections::hashmap::hashmap_example();
    // generics::find_max();
    generics::print_struct();
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}