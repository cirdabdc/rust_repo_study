/// 工作队列模式: 模拟 Solana MEV 场景
/// 多个交易生产者,多个处理器竞争处理交易
use async_channel::bounded;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
struct Transaction {
    id: u64,
    from: String,
    priority_fee: u64,
}

pub async fn run_mev_work_queue() {
    println!("=== MEV 工作队列模式 ===\n");

    // 创建交易队列,容量为 20
    let (tx_queue, rx_queue) = bounded::<Transaction>(20);

    // 模拟 3 个 RPC 节点接收交易(生产者)
    for node_id in 1..=3 {
        let sender = tx_queue.clone();
        tokio::spawn(async move {
            for tx_id in 1..=5 {
                let tx = Transaction {
                    id: node_id * 100 + tx_id,
                    from: format!("RPC-{}", node_id),
                    priority_fee: (tx_id * 1000) as u64,
                };

                sender.send(tx.clone()).await.unwrap();
                println!("[RPC-{}] 提交交易 #{}", node_id, tx.id);
                sleep(Duration::from_millis(100)).await;
            }
        });
    }

    drop(tx_queue); // 关闭发送端

    // 模拟 4 个 MEV 处理器(消费者)竞争处理交易
    let mut handles = vec![];
    for worker_id in 1..=4 {
        let receiver = rx_queue.clone();
        let handle = tokio::spawn(async move {
            let mut processed_count = 0;

            while let Ok(tx) = receiver.recv().await {
                println!(
                    "  [Worker-{}] 🔄 开始处理交易 #{} (fee: {})",
                    worker_id, tx.id, tx.priority_fee
                );

                // 模拟处理时间(优先费越高,处理越快)
                let process_time = 200 - (tx.priority_fee / 100);
                sleep(Duration::from_millis(process_time)).await;

                println!(
                    "  [Worker-{}] ✅ 完成交易 #{} (来自 {})",
                    worker_id, tx.id, tx.from
                );
                processed_count += 1;
            }

            println!("\n[Worker-{}] 总共处理: {} 笔交易", worker_id, processed_count);
            processed_count
        });
        handles.push(handle);
    }

    drop(rx_queue);

    // 等待所有 worker 完成
    let mut total = 0;
    for handle in handles {
        total += handle.await.unwrap();
    }

    println!("\n📊 统计: 总共处理 {} 笔交易", total);
    println!("关键点: 每笔交易只被一个 worker 处理,负载自动均衡\n");
}

/// 带优先级的工作队列
pub async fn run_priority_queue() {
    println!("=== 优先级队列模式 ===\n");

    let (sender, rx) = bounded::<Transaction>(10);

    // 生产者:发送不同优先级的交易
    tokio::spawn(async move {
        let txs = vec![
            Transaction { id: 1, from: "User-A".to_string(), priority_fee: 5000 },
            Transaction { id: 2, from: "User-B".to_string(), priority_fee: 1000 },
            Transaction { id: 3, from: "User-C".to_string(), priority_fee: 10000 },
            Transaction { id: 4, from: "User-D".to_string(), priority_fee: 500 },
        ];

        for tx in txs {
            sender.send(tx.clone()).await.unwrap();
            println!("提交交易 #{} (fee: {})", tx.id, tx.priority_fee);
            sleep(Duration::from_millis(50)).await;
        }
    });

    sleep(Duration::from_millis(100)).await;

    // 消费者:按接收顺序处理
    println!("\n按 FIFO 顺序处理:");
    while let Ok(tx) = rx.recv().await {
        println!("  处理交易 #{} (fee: {})", tx.id, tx.priority_fee);
    }

    println!("\n注意: async-channel 是 FIFO,若需优先级队列需额外排序\n");
}
