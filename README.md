# Aether Core

Aether Core 是一个基于 eBPF 技术的轻量级内核监控工具，专注于实时捕获和分析系统 IO 操作的延迟。

## 项目结构：
- **aether-ebpf**：eBPF 程序，运行在内核空间，用于捕获 IO 操作的开始和结束时间
- **aether-loader**：用户空间程序，用于加载 eBPF 程序并实时读取延迟数据

## 功能特性

-  **实时监控**：实时捕获和显示 IO 操作的延迟
-  **细粒度分析**：按进程 PID 跟踪 IO 延迟
-  **低开销**：基于 eBPF 技术，对系统性能影响最小
-  **跨平台**：支持 Linux 系统

## 技术栈

- **Rust**：主要开发语言
- **Aya**：eBPF 开发库
- **Tokio**：异步运行时

## 安装

### 前置依赖

- Rust 环境（推荐使用 rustup 安装）
- LLVM 和 Clang（eBPF 编译需要）
- Linux 内核 4.18+（支持 eBPF）

### 构建步骤

1. 克隆仓库：

```bash
git clone <repository-url>
cd aether-core
```

2. 构建 eBPF 程序：

```bash
cd aether-ebpf
cargo xtask build-ebpf
```

3. 构建用户空间程序：

```bash
cd ../aether-loader
cargo build --release
```

## 使用

1. 运行监控程序（需要 root 权限）：

```bash
sudo ./target/release/aether-loader
```

2. 程序会实时显示捕获到的 IO 操作及其延迟：

```
🚀 Aether 内核监控已就绪！
正在实时读取 IO_LATENCY Map 数据...
🎯 捕获 IO! PID: 635    | 延迟:       5594 ns
🎯 捕获 IO! PID: 4565   | 延迟:        644 ns
🎯 捕获 IO! PID: 4592   | 延迟:       3747 ns
🎯 捕获 IO! PID: 4597   | 延迟:       6283 ns
🎯 捕获 IO! PID: 4577   | 延迟:        715 ns
```

## 工作原理

1. **eBPF 程序**：
   - 通过 kprobe 挂载到 `vfs_read` 函数入口，记录开始时间
   - 通过 kretprobe 挂载到 `vfs_read` 函数返回，记录结束时间并计算延迟
   - 将延迟数据存储在 eBPF Map 中

2. **用户空间程序**：
   - 加载 eBPF 程序并挂载到相应的内核函数
   - 定期读取 eBPF Map 中的延迟数据
   - 实时显示捕获到的 IO 操作及其延迟

## 项目结构

```
aether-core/
├── aether-ebpf/          # eBPF 程序
│   ├── src/
│   │   └── main.rs       # eBPF 核心代码
│   ├── Cargo.toml        # eBPF 项目配置
│   └── .cargo/           # Cargo 配置
├── aether-loader/        # 用户空间程序
│   ├── src/
│   │   └── main.rs       # 加载器核心代码
│   └── Cargo.toml        # 加载器项目配置
└── README.md             # 项目文档
```

## 注意事项

- 本工具需要 root 权限运行，因为 eBPF 程序需要挂载到内核
- 目前仅监控 `vfs_read` 函数的 IO 操作，可根据需要扩展到其他 IO 函数
- 长时间运行可能会产生大量日志，建议根据实际需求调整输出频率

## 扩展建议

- 增加对更多 IO 函数的监控（如 `vfs_write`, `vfs_open` 等）
- 添加数据持久化功能，将延迟数据存储到数据库
- 实现 Web 界面，提供更直观的监控数据展示
- 添加告警机制，当 IO 延迟超过阈值时触发告警

## 许可证

[MIT License](LICENSE)
