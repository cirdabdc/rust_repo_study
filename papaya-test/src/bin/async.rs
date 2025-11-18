use papaya::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let map1 = HashMap::new();
    let map = map1.pin();
    map.insert('A', 1);
    assert_eq!(map.get(&'A'), Some(&1));
    assert_eq!(map.len(), 1);

    println!("{:?}", map);

    let map1 = HashMap::new();
    map1.pin().insert('A', 2);
    // Use a map from multiple threads.
    let maparc = Arc::new(map1);
    run(maparc.clone()).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}


async fn run(map: Arc<HashMap<char, i32>>) {
    tokio::spawn(async move {
        // Pin the map with an owned guard.
        let map = map.pin_owned();

        // Hold references across await points.
        let value = map.get(&'A');
        // tokio::fs::write("db.txt", format!("{value:?}")).await;
        println!("{:?}", map);
    });
}