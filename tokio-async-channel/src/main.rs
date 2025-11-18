mod basic_example;
mod work_queue;
mod comparison;

use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    print_welcome();

    loop {
        print_menu();

        let mut input = String::new();
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);

        if reader.read_line(&mut input).await.is_err() {
            continue;
        }

        match input.trim() {
            "1" => basic_example::run_basic().await,
            "2" => basic_example::run_unbounded().await,
            "3" => work_queue::run_mev_work_queue().await,
            "4" => work_queue::run_priority_queue().await,
            "5" => comparison::compare_mpsc().await,
            "6" => comparison::compare_broadcast().await,
            "7" => {
                comparison::print_summary();
            }
            "0" => {
                println!("\n👋 再见!\n");
                break;
            }
            _ => println!("\n❌ 无效选择,请重试\n"),
        }

        println!("\n按回车继续...");
        let mut pause = String::new();
        reader.read_line(&mut pause).await.ok();
    }
}

fn print_welcome() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║         async-channel + Tokio 测试示例                     ║");
    println!("║         多生产者-多消费者 (竞争模式) Channel               ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
}

fn print_menu() {
    println!("请选择测试示例:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  基础示例:");
    println!("    1. Bounded Channel 基础用法");
    println!("    2. Unbounded Channel 示例");
    println!("");
    println!("  工作队列模式 (MEV 场景):");
    println!("    3. MEV 工作队列 (多 worker 竞争)");
    println!("    4. 优先级队列示例");
    println!("");
    println!("  对比分析:");
    println!("    5. vs tokio::mpsc (多消费者 vs 单消费者)");
    println!("    6. vs tokio::broadcast (竞争 vs 广播)");
    println!("    7. 完整对比总结");
    println!("");
    println!("    0. 退出");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    print!("请输入选项: ");
}
