extern crate fast_bench;
#[macro_use] extern crate criterion;

use criterion::*;
use fast_bench::*;

use std::time::Duration;

fn basic(c: &mut Criterion) {
    static FILENAME: &str = "nanopore.fasta";

    c.bench(
        "nanopore",
        Benchmark::new("cat", |b| {b.iter(|| cat(FILENAME));})
            .warm_up_time(Duration::from_secs(2))
            .sample_size(10)
            .throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64))
            .with_function("kseq",  |b| {b.iter(|| kseq(FILENAME));})
            .with_function("bioparser",  |b| {b.iter(|| bioparser(FILENAME));})
            .with_function("rust_bio_buffered", |b| {b.iter(|| rust_bio_buffered(FILENAME));})
            .with_function("rust_bio_unbuffered", |b| {b.iter(|| rust_bio_unbuffered(FILENAME));})
    );
    
}

criterion_group!(benches, basic);
criterion_main!(benches);
   
