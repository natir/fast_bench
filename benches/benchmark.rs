extern crate fast_bench;
#[macro_use]
extern crate criterion;

use criterion::*;
use fast_bench::*;

use std::time::Duration;

fn basic(c: &mut Criterion) {
    static FILENAME: &str = "nanopore.fasta";

    c.bench(
        "nanopore",
        Benchmark::new("rust_bio_unbuffered", |b| {b.iter(|| rust_bio_unbuffered(FILENAME));})
            .sample_size(10)
            .measurement_time(Duration::from_secs(60))
            .warm_up_time(Duration::new(2, 0))
            .throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64))
            .with_function("cat",  |b| {b.iter(|| cat(FILENAME));})
            .with_function("kseq",  |b| {b.iter(|| kseq(FILENAME));})
            .with_function("rust_bio_buffered", |b| {b.iter(|| rust_bio_buffered(FILENAME));})    
    );
    
}

criterion_group!(benches, basic);
criterion_main!(benches);
   
