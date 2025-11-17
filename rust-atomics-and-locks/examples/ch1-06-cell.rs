use std::cell::Cell;

fn f(v: &Cell<Vec<i32>>) {
    // println!("v {:?}", v);        
    let mut v2 = v.take(); // Replaces the contents of the Cell with an empty Vec
    println!("v2 {:?}", v2);
    v2.push(1);
    v.set(v2); // Put the modified Vec back
    // println!("v {:?}", v);
}

fn main() {
    let v = Cell::new(vec![1, 2, 3]);
    f(&v);
    assert_eq!(v.into_inner(), vec![1, 2, 3, 1]);
}
