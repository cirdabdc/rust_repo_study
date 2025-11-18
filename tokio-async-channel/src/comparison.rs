/// 对比示例: async-channel vs Tokio channels
use async_channel::bounded as async_bounded;
use tokio::sync::{broadcast, mpsc};
use tokio::time::{sleep, Duration};

/// 对比 1: async-channel vs tokio::mpsc
/// 关键区别: async-channel 支持多个消费者竞争,mpsc 只能单消费者
pub async fn compare_mpsc() {
    println!("=== 对比: async-channel vs tokio::mpsc ===\n");

    // async-channel: 多消费者
    println!("1️⃣ async-channel (支持多消费者):");
    let (tx, rx) = async_bounded(3);

    for i in 1..=3 {
        tx.send(format!("Message {}", i)).await.unwrap();
    }
    drop(tx);

    let rx1 = rx.clone();
    let h1 = tokio::spawn(async move {
        if let Ok(msg) = rx1.recv().await {
            println!("  Consumer-1 收到: {}", msg);
        }
    });

    let rx2 = rx.clone();
    let h2 = tokio::spawn(async move {
        if let Ok(msg) = rx2.recv().await {
            println!("  Consumer-2 收到: {}", msg);
        }
    });

    h1.await.unwrap();
    h2.await.unwrap();

    println!("\n2️⃣ tokio::mpsc (单消费者):");
    let (tx, mut rx) = mpsc::channel(3);

    for i in 1..=3 {
        tx.send(format!("Message {}", i)).await.unwrap();
    }
    drop(tx);

    // mpsc 的 Receiver 没有 Clone trait,只能单个消费者
    while let Some(msg) = rx.recv().await {
        println!("  Consumer 收到: {}", msg);
    }

    println!("\n✅ 结论: 需要多消费者竞争处理时,使用 async-channel\n");
}

/// 对比 2: async-channel vs tokio::broadcast
/// 关键区别: async-channel 每个消息只被一个消费者接收,broadcast 每个消费者都收到
pub async fn compare_broadcast() {
    println!("=== 对比: async-channel vs tokio::broadcast ===\n");

    // async-channel: 每个消息只被一个消费者接收
    println!("1️⃣ async-channel (竞争模式 - 每个消息只给一个消费者):");
    let (tx, rx) = async_bounded(3);

    for i in 1..=3 {
        tx.send(format!("Task {}", i)).await.unwrap();
    }
    drop(tx);

    let rx1 = rx.clone();
    let h1 = tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("  Worker-1 处理: {}", msg);
            sleep(Duration::from_millis(10)).await;
        }
    });

    let rx2 = rx.clone();
    let h2 = tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("  Worker-2 处理: {}", msg);
            sleep(Duration::from_millis(10)).await;
        }
    });

    h1.await.unwrap();
    h2.await.unwrap();

    println!("\n2️⃣ tokio::broadcast (广播模式 - 每个消费者都收到):");
    let (tx, _) = broadcast::channel(3);

    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    for i in 1..=3 {
        tx.send(format!("Event {}", i)).unwrap();
    }
    drop(tx);

    let h1 = tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("  Subscriber-1 收到: {}", msg);
        }
    });

    let h2 = tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("  Subscriber-2 收到: {}", msg);
        }
    });

    h1.await.unwrap();
    h2.await.unwrap();

    println!("\n✅ 结论:");
    println!("   - 工作队列(每个任务只处理一次): 用 async-channel");
    println!("   - 事件广播(所有订阅者都通知): 用 broadcast\n");
}

/// Tokio channels 对比总结
pub fn print_summary() {
    println!("=== Tokio Channels 完整对比 ===\n");
    println!("┌─────────────────┬───────────┬───────────┬──────────────────────┐");
    println!("│ Channel 类型    │ 生产者    │ 消费者    │ 使用场景             │");
    println!("├─────────────────┼───────────┼───────────┼──────────────────────┤");
    println!("│ tokio::mpsc     │ 多个      │ 单个      │ 任务调度,命令传递    │");
    println!("│ tokio::oneshot  │ 单个      │ 单个      │ 请求-响应,超时控制   │");
    println!("│ tokio::broadcast│ 多个      │ 多个      │ 事件通知,状态广播    │");
    println!("│ tokio::watch    │ 多个      │ 多个      │ 配置热更新,状态同步  │");
    println!("│ async-channel   │ 多个      │ 多个(竞争)│ 工作队列,负载均衡    │");
    println!("└─────────────────┴───────────┴───────────┴──────────────────────┘");
    println!("\n💡 关键区别:");
    println!("   • broadcast: 每个消费者都收到所有消息(N→N 广播)");
    println!("   • async-channel: 每个消息只被一个消费者接收(N→1 竞争)");
    println!("\n🎯 Solana MEV 场景推荐: async-channel");
    println!("   理由: 多个 RPC 接收交易,多个 worker 竞争处理,负载均衡\n");
}
