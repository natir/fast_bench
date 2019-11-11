extern crate fast_bench;
#[macro_use] extern crate criterion;

use criterion::*;
use fast_bench::*;

use std::time::Duration;

fn basic(c: &mut Criterion) {
    static FILENAME: &str = "nanopore.fasta";

    c.bench(
        "default",
        Benchmark::new("cat", |b| {b.iter(|| cat(FILENAME));})
            .warm_up_time(Duration::from_secs(2))
            .sample_size(10)
            .throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64))
            .with_function("kseq",                |b| {b.iter(|| kseq(FILENAME, 16384));})
            .with_function("seqan",               |b| {b.iter(|| seqan(FILENAME));})
            .with_function("bioparser",           |b| {b.iter(|| bioparser(FILENAME));})
            .with_function("rust_bio_buffered",   |b| {b.iter(|| rust_bio_buffered(FILENAME, 8192));})
            .with_function("rust_bio_unbuffered", |b| {b.iter(|| rust_bio_unbuffered(FILENAME));})
    );
}

fn buffer_size(c: &mut Criterion) {
    static FILENAME: &str = "nanopore.fasta";

    let mut group = c.benchmark_group("buffer_size");

    group.warm_up_time(Duration::from_secs(2));
    group.sample_size(10);
    group.throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64));
 
    for i in 10..20 {
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
   
