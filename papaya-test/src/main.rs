fn main() {
    use papaya::HashMap;

    // Create a map.
    let map = HashMap::new();

    // Pin the map.
    let map = map.pin();

    // Use the map as normal.
    map.insert('A', 1);
    assert_eq!(map.get(&'A'), Some(&1));
    assert_eq!(map.len(), 1);

    println!("{:?}", map);


    // Use a map from multiple threads.
    let map = HashMap::new();
    std::thread::scope(|s| {
        // Insert some values.
        s.spawn(|| {
            let map = map.pin();
            for i in 'A'..='Z' {
                map.insert(i, 1);
            }
        });

        // Read the values.
        s.spawn(|| {
            for (key, value) in map.pin().iter() {
                println!("{key}: {value}");
            }
        });

        // Remove the values.
        s.spawn(|| {
            let map = map.pin();
            for i in 'A'..='Z' {
                map.remove(&i);
            }
        });

        
    });


    let map1 = HashMap::new();
    let map1 = map1.pin();
    map1.insert("poney", 42);
    map1.update_or_insert("poney", |e| e+1, 42);
    println!("{:?}", map1);


    let map = HashMap::new();
    assert_eq!(*map.pin().update_or_insert_with("a", |i| i + 1, || 0), 0);
    assert_eq!(*map.pin().update_or_insert_with("a", |i| i + 1, || 0), 1);

}
