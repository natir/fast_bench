extern crate fast_bench;
#[macro_use] extern crate criterion;

use criterion::*;
use fast_bench::*;

use std::time::Duration;

fn warmup_time() -> std::time::Duration {
    return match std::env::var("WARMUP_TIME") {
        Ok(val) => Duration::from_secs(val.parse::<u64>().expect("Error we can parse WARMUP_TIME in u64")),
        Err(_e) => Duration::from_secs(2),
    };
}

fn sample_size() -> usize {
    return match std::env::var("SAMPLE_SIZE") {
        Ok(val) => val.parse::<usize>().expect("Error we can parse WARMUP_TIME in u64"),
        Err(_e) => 10,
    };
}

fn basic(c: &mut Criterion) {
    static FILENAME: &str = "nanopore.fasta";

    let mut group = c.benchmark_group("default");
    group.warm_up_time(warmup_time());
    group.sample_size(sample_size());
    group.throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64));

    group.bench_function("cat",                 |b| {b.iter(|| cat(FILENAME));});
    group.bench_function("kseq",                |b| {b.iter(|| kseq(FILENAME, 16384));});
    group.bench_function("seqan",               |b| {b.iter(|| seqan(FILENAME));});
    group.bench_function("bioparser",           |b| {b.iter(|| bioparser(FILENAME));});
    group.bench_function("rust_memmap",         |b| {b.iter(|| memmap(FILENAME));});
    group.bench_function("rust_bio_buffered",   |b| {b.iter(|| rust_bio_buffered(FILENAME, 8192));});
    group.bench_function("rust_bio_unbuffered", |b| {b.iter(|| rust_bio_unbuffered(FILENAME));});
}

fn buffer_size(c: &mut Criterion) {
    static FILENAME: &str = "nanopore.fasta";

    let mut group = c.benchmark_group("buffer_size");

    group.warm_up_time(warmup_time());
    group.sample_size(sample_size());
    group.throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64));
 
    for i in 5..20 {
        let buffer_size = 1 << i;
        
        group.bench_with_input(BenchmarkId::new("kseq", buffer_size), &buffer_size, |b, &buffer_size| {
            b.iter(|| kseq(FILENAME, buffer_size));
        }).throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64));

        group.bench_with_input(BenchmarkId::new("rust_bio_buffered", buffer_size), &buffer_size, |b, &buffer_size| {
            b.iter(|| kseq(FILENAME, buffer_size));
        });
    }
}

criterion_group!(benches, basic, buffer_size);
criterion_main!(benches);
   
