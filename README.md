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


# Notes on Computing Approaches

While this guide focuses on running **Qiner** on CPUs, cryptographic performance can be significantly improved by leveraging hardware acceleration — particularly for hash-based workloads like those involving **SHA-2**, **SHA-3**, and **Keccak**.

---

## Crypto-Accelerated Computing Options

### CUDA GPUs
Ideal for parallel hash computations using SHA-2 or Keccak-f permutations. GPUs excel in workloads where millions of hashes can be executed in parallel, such as:
- Mining
- Brute-force nonce searches
- Merkle tree updates

---

### FPGAs
Offer reconfigurable logic blocks that can implement SHA-2 and SHA-3 hash pipelines directly in hardware.

- **Keccak-f[1600] permutations** — the core of SHA-3 — benefit from pipelined round-function implementations.
- Enables **high-throughput** and **low-latency** hashing.

Key characteristics:
- Up to 25 rounds per hash
- Bitwise XOR, θ, ρ, π, χ, and ι steps mapped efficiently to LUTs and DSPs

---

### ASICs
When a specific hashing algorithm (e.g., SHA-256 or Keccak) is locked in, ASICs offer:
- The best hash rate per watt
- Industrial-scale mining capabilities

⚠️ Not adaptable for protocols that evolve over time.

---

### Heterogeneous Architectures
In a cryptographic workload:

- **CPU**: Handles control flow, job assignment, and communication
- **GPU**: Runs thousands of hash kernels for general-purpose load
- **FPGA or crypto coprocessor**: Accelerates hash pipelines like Keccak-f, SHA-256, or SHA3-512 for mission-critical verification

---

## 🔐 CECCAC for Cryptography

**CECCAC** (*Custom Embedded Crypto-Compute Acceleration Cores*) are hardware logic blocks optimized for cryptographic primitives — particularly hashing, modular math, and permutation functions.

In the context of **SHA-2/SHA-3** and **Keccak**, CECCAC provides:

---

### ✅ Hardware-accelerated Keccak-f[1600] permutation
- All 25 rounds in a pipelined or unrolled structure
- Supports SHA3-224, SHA3-256, SHA3-384, SHA3-512, and SHAKE variants
- Removes instruction decoding and memory access bottlenecks

---

### ✅ Optimized SHA-2 Cores
- Implements full SHA-256/SHA-512 logic with feedback paths and carry-save adders
- Highly parallelizable for Merkle hashing or tree-based commitments

---

### ✅ Low-latency Sponge Construction
- Ideal for Proof-of-Work systems using sponge-based hash functions (e.g., Qubic, Ethereum’s Keccak)
- Deterministic performance across batches of input blocks

---

### Use Cases for Qiner
- Fast **nonce search** using Keccak or SHA-3  
- Accelerated **verification of solution thresholds**  
- Real-time **block digest computation** on embedded or edge nodes  

---

### ⚡ TL;DR
If Qiner uses **SHA-3 or Keccak** under the hood, deploying it on **CECCAC-enabled hardware** (or even a mid-range **FPGA** with a Keccak pipeline) can yield **10x–100x performance improvement per watt** over CPUs — while preserving flexibility not available with ASICs.
