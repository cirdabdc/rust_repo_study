# 示例代码片段

## 快速开始

### 1. 基础用法

```rust
use async_channel::bounded;

#[tokio::main]
async fn main() {
    // 创建容量为 5 的有界 channel
    let (tx, rx) = bounded(5);

    // 发送消息
    tx.send("Hello").await.unwrap();

    // 接收消息
    let msg = rx.recv().await.unwrap();
    println!("{}", msg);  // 输出: Hello
}
```

### 2. 多生产者-多消费者(核心用法)

```rust
use async_channel::bounded;
use tokio;

#[tokio::main]
async fn main() {
    let (tx, rx) = bounded(10);

    // 生产者 1
    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx1.send("From Producer 1").await.unwrap();
    });

    // 生产者 2
    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx2.send("From Producer 2").await.unwrap();
    });

    drop(tx); // 关闭原始发送端

    // 消费者 1
    let rx1 = rx.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("Consumer 1: {}", msg);
        }
    });

    // 消费者 2
    let rx2 = rx.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("Consumer 2: {}", msg);
        }
    });

    // 注意: 每个消息只被一个消费者接收
}
```

## Solana MEV 实战示例

### 场景: 多 RPC 监听 + 多 Worker 处理

```rust
use async_channel::bounded;
use tokio;
use std::time::Duration;

#[derive(Debug, Clone)]
struct SolanaTransaction {
    signature: String,
    priority_fee: u64,
    instructions: Vec<String>,
}

#[tokio::main]
async fn main() {
    // 创建交易队列
    let (tx_queue, rx_queue) = bounded::<SolanaTransaction>(100);

    // 启动 3 个 RPC 监听器
    let rpc_endpoints = vec![
        "https://api.mainnet-beta.solana.com",
        "https://solana-api.projectserum.com",
        "https://rpc.ankr.com/solana",
    ];

    for (i, endpoint) in rpc_endpoints.iter().enumerate() {
        let tx = tx_queue.clone();
        let endpoint = endpoint.to_string();

        tokio::spawn(async move {
            loop {
                // 模拟监听 mempool
                let tx_data = listen_mempool(&endpoint).await;

                // 发送到队列
                if let Err(e) = tx.send(tx_data).await {
                    eprintln!("RPC-{} send error: {}", i, e);
                    break;
                }
            }
        });
    }

    drop(tx_queue); // 关闭原始发送端

    // 启动 4 个 MEV 处理 worker
    let mut handles = vec![];
    for worker_id in 0..4 {
        let rx = rx_queue.clone();

        let handle = tokio::spawn(async move {
            let mut processed = 0;

            while let Ok(tx) = rx.recv().await {
                // 检查 MEV 机会
                if is_mev_opportunity(&tx) {
                    // 构建并提交 bundle
                    submit_mev_bundle(&tx).await;
                    processed += 1;
                }
            }

            println!("Worker-{} processed {} MEV opportunities", worker_id, processed);
        });

        handles.push(handle);
    }

    // 等待所有 worker 完成
    for handle in handles {
        handle.await.unwrap();
    }
}

// 模拟函数
async fn listen_mempool(endpoint: &str) -> SolanaTransaction {
    tokio::time::sleep(Duration::from_millis(100)).await;
    SolanaTransaction {
        signature: "5J8...".to_string(),
        priority_fee: 10000,
        instructions: vec!["swap".to_string()],
    }
}

fn is_mev_opportunity(tx: &SolanaTransaction) -> bool {
    tx.priority_fee > 5000 && tx.instructions.contains(&"swap".to_string())
}

async fn submit_mev_bundle(tx: &SolanaTransaction) {
    println!("🚀 Submitting MEV bundle for {}", tx.signature);
}
```

## 高级模式

### 1. 优雅关闭

```rust
use async_channel::bounded;
use tokio::signal;

#[tokio::main]
async fn main() {
    let (tx, rx) = bounded(10);

    // Worker
    let handle = tokio::spawn(async move {
        while let Ok(task) = rx.recv().await {
            process(task).await;
        }
        println!("Worker shutting down gracefully");
    });

    // 等待 Ctrl-C
    signal::ctrl_c().await.unwrap();

    // 关闭发送端,触发 worker 退出
    drop(tx);

    handle.await.unwrap();
}
```

### 2. 超时处理

```rust
use async_channel::bounded;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = bounded(1);

    // 尝试在 1 秒内接收
    match timeout(Duration::from_secs(1), rx.recv()).await {
        Ok(Ok(msg)) => println!("Received: {}", msg),
        Ok(Err(_)) => println!("Channel closed"),
        Err(_) => println!("Timeout!"),
    }
}
```

### 3. 选择性接收 (使用 select!)

```rust
use async_channel::bounded;
use tokio::select;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = bounded(1);
    let (tx2, rx2) = bounded(1);

    tx1.send("From channel 1").await.unwrap();

    select! {
        msg = rx1.recv() => println!("Received: {:?}", msg),
        msg = rx2.recv() => println!("Received: {:?}", msg),
    }
}
```

### 4. 带重试的发送

```rust
use async_channel::{bounded, TrySendError};
use tokio::time::{sleep, Duration};

async fn send_with_retry<T: Clone>(
    tx: &async_channel::Sender<T>,
    value: T,
    max_retries: usize,
) -> Result<(), String> {
    for attempt in 0..max_retries {
        match tx.try_send(value.clone()) {
            Ok(_) => return Ok(()),
            Err(TrySendError::Full(_)) => {
                println!("Channel full, retrying... ({})", attempt + 1);
                sleep(Duration::from_millis(100)).await;
            }
            Err(TrySendError::Closed(_)) => {
                return Err("Channel closed".to_string());
            }
        }
    }
    Err("Max retries exceeded".to_string())
}
```

## 性能优化技巧

### 1. 批量发送

```rust
let batch_size = 100;
let mut batch = Vec::new();

for item in items {
    batch.push(item);

    if batch.len() >= batch_size {
        tx.send(batch.clone()).await.unwrap();
        batch.clear();
    }
}

// 发送剩余
if !batch.is_empty() {
    tx.send(batch).await.unwrap();
}
```

### 2. 动态调整 worker 数量

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

let max_workers = 10;
let semaphore = Arc::new(Semaphore::new(max_workers));

for _ in 0..max_workers {
    let rx = rx_queue.clone();
    let sem = semaphore.clone();

    tokio::spawn(async move {
        while let Ok(task) = rx.recv().await {
            let permit = sem.acquire().await.unwrap();

            tokio::spawn(async move {
                process(task).await;
                drop(permit);
            });
        }
    });
}
```

## 常见错误处理

### 1. Channel 已关闭

```rust
match rx.recv().await {
    Ok(msg) => process(msg),
    Err(async_channel::RecvError) => {
        println!("All senders have been dropped");
    }
}
```

### 2. 发送到已关闭的 channel

```rust
match tx.send(msg).await {
    Ok(_) => println!("Sent successfully"),
    Err(async_channel::SendError(msg)) => {
        println!("All receivers dropped, message: {:?}", msg);
    }
}
```

### 3. try_recv (非阻塞)

```rust
use async_channel::TryRecvError;

match rx.try_recv() {
    Ok(msg) => println!("Got: {}", msg),
    Err(TryRecvError::Empty) => println!("No messages"),
    Err(TryRecvError::Closed) => println!("Channel closed"),
}
```

## 测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use async_channel::bounded;

    #[tokio::test]
    async fn test_multiple_consumers() {
        let (tx, rx) = bounded(10);

        // 发送 10 个消息
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
        drop(tx);

        // 2 个消费者
        let rx1 = rx.clone();
        let h1 = tokio::spawn(async move {
            let mut count = 0;
            while rx1.recv().await.is_ok() {
                count += 1;
            }
            count
        });

        let rx2 = rx.clone();
        let h2 = tokio::spawn(async move {
            let mut count = 0;
            while rx2.recv().await.is_ok() {
                count += 1;
            }
            count
        });

        let count1 = h1.await.unwrap();
        let count2 = h2.await.unwrap();

        // 验证: 总共 10 个消息被接收,且每个消息只被一个消费者接收
        assert_eq!(count1 + count2, 10);
    }
}
```

## 监控和调试

```rust
use async_channel::bounded;

let (tx, rx) = bounded(100);

// 获取 channel 状态
println!("Channel capacity: {}", tx.capacity().unwrap());
println!("Current length: {}", tx.len());
println!("Is empty: {}", tx.is_empty());
println!("Is full: {}", tx.is_full());
```
