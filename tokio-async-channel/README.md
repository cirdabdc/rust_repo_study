# tokio-async-channel 测试项目

## 项目简介

这个项目演示如何使用 `async-channel` 与 Tokio 集成,实现**多生产者-多消费者(竞争模式)**的消息传递。

## 为什么需要 async-channel?

### Tokio 标准 channels 的局限

Tokio 提供了 4 种 channel,但都不支持"多消费者竞争处理"模式:

| Channel 类型 | 生产者 | 消费者 | 使用场景 | 局限性 |
|-------------|--------|--------|----------|--------|
| `tokio::mpsc` | 多个 | **单个** | 任务调度 | ❌ 不支持多消费者 |
| `tokio::oneshot` | 单个 | 单个 | 请求-响应 | ❌ 单次使用 |
| `tokio::broadcast` | 多个 | 多个(都收到) | 事件广播 | ❌ 所有消费者都收到同一消息 |
| `tokio::watch` | 多个 | 多个(最新值) | 配置更新 | ❌ 只保留最新值 |

### async-channel 的优势

```rust
use async_channel::bounded;

let (tx, rx) = bounded(10);

// ✅ 支持多个消费者竞争处理
let rx1 = rx.clone();  // 消费者 1
let rx2 = rx.clone();  // 消费者 2

// 每个消息只会被一个消费者接收(自动负载均衡)
```

**关键特性:**
- 🎯 **竞争模式**: 每个消息只被一个消费者接收
- ⚖️ **自动负载均衡**: 多个 worker 自动分配任务
- 🔄 **完全异步**: 与 Tokio 无缝集成
- 📦 **有界/无界**: 支持背压控制

## 使用场景

### 1. 工作队列 (适合 Solana MEV)

```rust
// 多个 RPC 节点接收交易
let (tx_queue, rx_queue) = bounded(100);

// 多个 MEV worker 竞争处理
for worker_id in 1..=4 {
    let rx = rx_queue.clone();
    tokio::spawn(async move {
        while let Ok(tx) = rx.recv().await {
            process_transaction(tx).await;  // 每笔交易只处理一次
        }
    });
}
```

### 2. 爬虫任务分发

```rust
// URL 生产者
tx.send(url).await?;

// 多个爬虫 worker 竞争抓取
while let Ok(url) = rx.recv().await {
    crawl(url).await;
}
```

### 3. 数据处理管道

```rust
// 数据源 → Channel → 多个处理器并行处理
```

## 项目结构

```
src/
├── main.rs             # 交互式菜单主程序
├── basic_example.rs    # 基础用法示例
├── work_queue.rs       # MEV 工作队列模式
└── comparison.rs       # 与 Tokio channels 对比
```

## 运行示例

```bash
cargo run
```

### 菜单选项

1. **Bounded Channel 基础用法** - 展示有界 channel 的基本操作
2. **Unbounded Channel 示例** - 无界 channel 的使用
3. **MEV 工作队列** - 模拟 Solana MEV 场景(多 RPC + 多 Worker)
4. **优先级队列示例** - 展示 FIFO 特性
5. **vs tokio::mpsc** - 对比多消费者 vs 单消费者
6. **vs tokio::broadcast** - 对比竞争模式 vs 广播模式
7. **完整对比总结** - 所有 channel 类型对比表

## 核心概念

### 1. 竞争模式 vs 广播模式

```rust
// async-channel: 竞争模式
let (tx, rx) = bounded(3);
let rx1 = rx.clone();
let rx2 = rx.clone();

tx.send("Task-1").await?;
// 只有 rx1 或 rx2 其中之一收到 "Task-1"
```

```rust
// tokio::broadcast: 广播模式
let (tx, _) = broadcast::channel(3);
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

tx.send("Event-1")?;
// rx1 和 rx2 都收到 "Event-1"
```

### 2. 背压控制

```rust
// 有界 channel: 容量满时,send() 会阻塞(背压)
let (tx, rx) = bounded(5);

// 无界 channel: 永不阻塞(可能内存溢出)
let (tx, rx) = unbounded();
```

### 3. 与 Tokio 集成

```rust
// ✅ async-channel 完全支持 async/await
tx.send(data).await?;
let data = rx.recv().await?;

// 与 tokio::spawn 无缝配合
tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
        // ...
    }
});
```

## Solana MEV 应用建议

### 推荐架构

```rust
use async_channel::bounded;

// 交易队列
let (tx_queue, rx_queue) = bounded(1000);

// RPC 监听器(生产者)
for rpc_url in rpc_urls {
    let tx = tx_queue.clone();
    tokio::spawn(async move {
        // 监听 mempool,发送交易到队列
        tx.send(transaction).await?;
    });
}

// MEV 处理器(消费者)
for worker_id in 0..num_workers {
    let rx = rx_queue.clone();
    tokio::spawn(async move {
        while let Ok(tx) = rx.recv().await {
            // 竞争处理交易
            process_mev_opportunity(tx).await;
        }
    });
}
```

### 优势

1. **自动负载均衡**: Worker 空闲时自动获取下一个交易
2. **背压控制**: 队列满时自然限流,防止内存溢出
3. **无需手动调度**: 不需要实现复杂的任务分配逻辑
4. **水平扩展**: 增加 worker 数量即可提升吞吐量

## API 对比

| 操作 | async-channel | tokio::mpsc | tokio::broadcast |
|------|---------------|-------------|------------------|
| 创建 | `bounded(n)` | `channel(n)` | `channel(n)` |
| 发送 | `tx.send(v).await?` | `tx.send(v).await?` | `tx.send(v)?` |
| 接收 | `rx.recv().await?` | `rx.recv().await?` | `rx.recv().await?` |
| 克隆发送端 | `tx.clone()` | `tx.clone()` | `tx.clone()` |
| 克隆接收端 | `rx.clone()` ✅ | ❌ 不支持 | `tx.subscribe()` |

## 性能考虑

- **有界 vs 无界**: 生产环境推荐有界 channel,防止内存泄漏
- **容量设置**: 根据消费速度和延迟要求调整
- **Worker 数量**: 根据 CPU 核心数和 I/O 比例调整

## 参考资料

- [async-channel 文档](https://docs.rs/async-channel)
- [Tokio channels 文档](https://tokio.rs/tokio/tutorial/channels)
- [Rust 并发模式](https://rust-lang.github.io/async-book/)

## License

MIT
