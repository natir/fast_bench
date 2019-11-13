extern crate fast_bench;
extern crate criterion;

use criterion::*;
use fast_bench::*;

use std::io::Write;
use std::io::BufRead;
use std::str::FromStr;
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

macro_rules! add_in_group {
    ($name:expr, $path:expr, $filename:ident, $group:ident, $command:ident, $process:ident, $stdin:ident, $stdout:ident) => (
        let mut $command = std::process::Command::new($path);
        $command.arg($filename);
            
        let $process = $command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect(&format!("Unable to start {} process", $name));

        let mut $stdin = $process
            .stdin
            .expect("Unable to get stdin for child process");
        let $stdout = $process
            .stdout
            .expect("Unable to get stdout for child process");
        let mut $stdout = std::io::BufReader::new($stdout);
        
        $group.bench_function($name, |b| {
            b.iter_custom(|iters| {
                writeln!($stdin, "{}", iters).expect("Unable to send iteration count to child process");

                let mut line = String::new();
                $stdout.read_line(&mut line).expect("Unable to read time from child process");

                Duration::from_nanos(u64::from_str(line.trim()).expect("Unable to parse time from child process"))
            })
        });
    );
}

macro_rules! add_in_group_input {
    ($name:expr, $path:expr, $filename:ident, $input:ident, $group:ident, $command:ident, $process:ident, $stdin:ident, $stdout:ident) => (
        let mut $command = std::process::Command::new($path);
        $command.arg($filename);
            
        let $process = $command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect(&format!("Unable to start {} process", $name));

        let mut $stdin = $process
            .stdin
            .expect("Unable to get stdin for child process");
        let $stdout = $process
            .stdout
            .expect("Unable to get stdout for child process");
        let mut $stdout = std::io::BufReader::new($stdout);
        
        $group.bench_with_input(BenchmarkId::new($name, $input), &$input, |b, &$input| {
            b.iter_custom(|iters| {
                writeln!($stdin, "{}", iters).expect("Unable to send iteration count to child process");

                let mut line = String::new();
                $stdout.read_line(&mut line).expect("Unable to read time from child process");

                Duration::from_nanos(u64::from_str(line.trim()).expect("Unable to parse time from child process"))
            })
        });
    );
}

fn basic(c: &mut Criterion) {
    static FILENAME: &str = "nanopore.fasta";

    let mut group = c.benchmark_group("default");
    group.warm_up_time(warmup_time());
    group.sample_size(sample_size());
    group.throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64));
    
    add_in_group!("kseq", "cpp/kseq_16384", FILENAME, group, kseq_command, kseq_process, kseq_stdin, kseq_stdout);
    add_in_group!("seqan", "cpp/seqan", FILENAME, group, seqan_command, seqan_process, seqan_stdin, seqan_stdout);
    add_in_group!("bioparser", "cpp/bioparser", FILENAME, group, bioparser_command, bioparser_process, bioparser_stdin, bioparser_stdout);

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

    
        add_in_group_input!("kseq", &format!("cpp/kseq_{}", buffer_size), FILENAME, buffer_size, group, kseq_command, kseq_process, kseq_stdin, kseq_stdout);
        
        group.bench_with_input(BenchmarkId::new("rust_bio_buffered", buffer_size), &buffer_size, |b, &buffer_size| {
            b.iter(|| rust_bio_buffered(FILENAME, buffer_size) );
        });
    }
}

criterion_group!(b, basic, buffer_size);
criterion_main!(b);

