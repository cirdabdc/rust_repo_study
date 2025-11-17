use std::rc::Rc;

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    println!("{:?}", a.as_ptr());
    println!("{:?}", b.as_ptr());

    assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
}
