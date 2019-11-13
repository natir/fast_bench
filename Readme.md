# Fasta bench

A fasta parsing benchmark.

## Setup

You need a [Rust](https://rustup.rs/)

```sh
git clone --recurse-submodules -j8 https://github.com/natir/fast_bench.git

./dl_data.sh

cargo build
```

## Run

```
WARMUP_TIME=2 SAMPLE_SIZE=10 cargo bench
```
