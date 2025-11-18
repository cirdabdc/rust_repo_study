/// 基础示例: async-channel 的基本用法
/// 展示多生产者、多消费者,每个消息只被一个消费者接收
use async_channel::{bounded, unbounded};
use tokio::time::{sleep, Duration};

pub async fn run_basic() {
    println!("=== 基础示例: Bounded Channel ===\n");

    // 创建一个容量为 5 的有界 channel
    let (tx, rx) = bounded(5);

    // 生产者 1
    let tx1 = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            tx1.send(format!("Producer-1: Message {}", i))
                .await
                .unwrap();
            println!("✓ Producer-1 发送: Message {}", i);
            sleep(Duration::from_millis(100)).await;
        }
    });

    // 生产者 2
    let tx2 = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            tx2.send(format!("Producer-2: Message {}", i))
                .await
                .unwrap();
            println!("✓ Producer-2 发送: Message {}", i);
            sleep(Duration::from_millis(100)).await;
        }
    });

    // 关闭原始发送端(只保留克隆的)
    drop(tx);

    // 消费者 1
    let rx1 = rx.clone();
    let handle1 = tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("  → Consumer-1 接收: {}", msg);
            sleep(Duration::from_millis(50)).await;
        }
        println!("Consumer-1 退出");
    });

    // 消费者 2
    let rx2 = rx.clone();
    let handle2 = tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("  → Consumer-2 接收: {}", msg);
            sleep(Duration::from_millis(50)).await;
        }
        println!("Consumer-2 退出");
    });

    drop(rx); // 关闭原始接收端

    handle1.await.unwrap();
    handle2.await.unwrap();

    println!("\n关键点: 每个消息只被一个消费者接收(与 broadcast 不同)\n");
}

pub async fn run_unbounded() {
    println!("=== 无界 Channel 示例 ===\n");

    let (tx, rx) = unbounded();

    // 快速发送大量消息
    tokio::spawn(async move {
        for i in 1..=10 {
            tx.send(i).await.unwrap();
            println!("发送: {}", i);
        }
    });

    sleep(Duration::from_millis(100)).await;

    // 单个消费者慢速接收
    while let Ok(num) = rx.recv().await {
        println!("  接收: {}", num);
        sleep(Duration::from_millis(50)).await;
    }

    println!("\n无界 channel 不会阻塞发送者\n");
}
