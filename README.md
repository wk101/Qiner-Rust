# Qiner on Rust

Qiner is a high-performance application written in Rust that leverages CPU-specific instructions and optimizations for computational tasks. While this guide focuses on deploying Qiner using CPUs, it's worth noting that there are other approaches that utilize CUDA GPUs, FPGAs, and heterogeneous computing for enhanced performance.

## Deploy

### Download Rust

#### Windows

1. [Rust x64](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) / [Rust x32](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)
2. [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

#### Linux

1. `sudo apt update`
2. `sudo apt install build-essential -y`
3. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Building Qiner

1. Open the terminal in the folder containing Qiner (the folder where the `Cargo.toml` file is located)
2. Run `cargo build --release`

The built Qiner executable will be located at `./target/release/`

### Starting Qiner

#### .env

The options to run Qiner are specified in the `.env` file.

1. Create a `.env` file next to the built Qiner executable.
2. Fill in the following options: `RUST_LOG`, `NUMBER_OF_THREADS`, `ID`, `SERVER_IP`, `SERVER_PORT`, `VERSION`, `RANDOM_SEED`, `SOLUTION_THRESHOLD`

#### RUST_LOG

Set to `INFO` to see the output in the console. Read more at the [env_logger documentation](https://docs.rs/env_logger/0.10.0/env_logger/#enabling-logging).

#### NUMBER_OF_THREADS

Specifies the number of threads to be used for mining.

#### ID

Qiner ID consisting of 60 characters.

#### SERVER_IP and SERVER_PORT

The IP and port to which Qiner will connect.

#### VERSION

The version of Qubic.

##### Example
```
RUST_LOG=INFO
NUMBER_OF_THREADS=8
ID=UBAZRCVPOZTDKGCBNPGYFUPLZXDDNHSEGJRTAJKWJBHJDKHMAKVVFAKCZGRI
SERVER_IP=8.8.8.8
SERVER_PORT=21841
VERSION=1.142.1
RANDOM_SEED=1,0,233,9,136,69,43,139
SOLUTION_THRESHOLD=22

```


## Notes on Computing Approaches

While this guide covers deploying Qiner with a focus on using CPUs, there are other approaches that can be utilized for enhanced performance:
- **CUDA GPUs**: Leveraging NVIDIA's CUDA framework for parallel computing on GPUs.
- **FPGAs**: Using Field-Programmable Gate Arrays for highly efficient and customizable hardware acceleration.
- **Heterogeneous Computing**: Combining different types of processors (e.g., CPUs, GPUs, and FPGAs) to optimize performance for specific tasks.

