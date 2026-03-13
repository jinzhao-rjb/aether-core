# Aether Core

Aether Core is a lightweight kernel monitoring tool based on eBPF technology, focused on real-time capture and analysis of system IO operation latency.

## Project Structure:
- **aether-ebpf**: eBPF program running in kernel space, used to capture start and end times of IO operations
- **aether-loader**: User space program used to load eBPF programs and read latency data in real-time

## Features

- 🚀 **Real-time Monitoring**: Capture and display IO operation latency in real-time
- 🔍 **Fine-grained Analysis**: Track IO latency by process PID
- 📊 **Low Overhead**: Based on eBPF technology, minimal impact on system performance
- 💻 **Cross-platform**: Supports Linux systems

## Technology Stack

- **Rust**: Main development language
- **Aya**: eBPF development library
- **Tokio**: Asynchronous runtime

## Installation

### Prerequisites

- Rust environment (recommended to install via rustup)
- LLVM and Clang (required for eBPF compilation)
- Linux kernel 4.18+ (supports eBPF)

### Build Steps

1. Clone the repository:

```bash
git clone <repository-url>
cd aether-core
```

2. Build the eBPF program:

```bash
cd aether-ebpf
cargo xtask build-ebpf
```

3. Build the user space program:

```bash
cd ../aether-loader
cargo build --release
```

## Usage

1. Run the monitoring program (requires root privileges):

```bash
sudo ./target/release/aether-loader
```

2. The program will display captured IO operations and their latency in real-time:

```
🚀 Aether kernel monitoring is ready!
Reading IO_LATENCY Map data in real-time...
🎯 Captured IO! PID: 635    | Latency:       5594 ns
🎯 Captured IO! PID: 4565   | Latency:        644 ns
🎯 Captured IO! PID: 4592   | Latency:       3747 ns
🎯 Captured IO! PID: 4597   | Latency:       6283 ns
🎯 Captured IO! PID: 4577   | Latency:        715 ns
```

## Working Principle

1. **eBPF Program**:
   - Mounts to `vfs_read` function entry via kprobe, records start time
   - Mounts to `vfs_read` function return via kretprobe, records end time and calculates latency
   - Stores latency data in eBPF Map

2. **User Space Program**:
   - Loads eBPF program and mounts it to corresponding kernel functions
   - Periodically reads latency data from eBPF Map
   - Displays captured IO operations and their latency in real-time

## Project Structure

```
aether-core/
├── aether-ebpf/          # eBPF program
│   ├── src/
│   │   └── main.rs       # eBPF core code
│   ├── Cargo.toml        # eBPF project configuration
│   └── .cargo/           # Cargo configuration
├── aether-loader/        # User space program
│   ├── src/
│   │   └── main.rs       # Loader core code
│   └── Cargo.toml        # Loader project configuration
└── README.md             # Project documentation
```

## Notes

- This tool requires root privileges to run, as eBPF programs need to be mounted to the kernel
- Currently only monitors IO operations of the `vfs_read` function, can be extended to other IO functions as needed
- Long-term operation may generate a large amount of logs, it is recommended to adjust the output frequency according to actual needs

## Extension Suggestions

- Add monitoring for more IO functions (such as `vfs_write`, `vfs_open`, etc.)
- Add data persistence functionality to store latency data in a database
- Implement a web interface to provide more intuitive monitoring data display
- Add alert mechanism to trigger alerts when IO latency exceeds thresholds

## License

[MIT License](LICENSE)
