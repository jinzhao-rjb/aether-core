use aya::{Bpf, programs::KProbe, maps::HashMap};
use std::convert::TryInto;
use std::{fs, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 加载你编译好的 2KB 核武
    let data = fs::read("../aether-ebpf/target/bpfel-unknown-none/release/aether-ebpf")?;
    let mut bpf = Bpf::load(&data)?;
    
    // 2. 挂载 kprobe (开始点) 和 kretprobe (结束点)
    let p1: &mut KProbe = bpf.program_mut("aether_io_trace").unwrap().try_into()?;
    p1.load()?; p1.attach("vfs_read", 0)?;
    
    let p2: &mut KProbe = bpf.program_mut("aether_io_ret").unwrap().try_into()?;
    p2.load()?; p2.attach("vfs_read", 0)?;

    println!("🚀 Aether 内核监控已就绪！");
    println!("正在实时读取 IO_LATENCY Map 数据...");

    // 3. 获取内核中的 Map 句柄
    // 这里的类型要跟你 eBPF 代码里的 HashMap<u32, u64> 对应
    let mut latency_map: HashMap<_, u32, u64> = HashMap::try_from(bpf.map_mut("IO_LATENCY").unwrap())?;

    // 4. 循环读取并清空 Map，实现实时刷新
    loop {
        for item in latency_map.iter().flatten() {
            let (pid, latency) = item;
            if latency > 0 {
                println!("🎯 捕获 IO! PID: {:<6} | 延迟: {:>10} ns", pid, latency);
            }
        }
        // 读取完后可以考虑清理 map 避免数据堆积，但演示时直接循环即可
        thread::sleep(Duration::from_millis(200));
    }
}
