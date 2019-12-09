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

macro_rules! create_command {
    ($path:expr, $command:ident, $process:ident, $stdin:ident, $stdout:ident, $($args:expr), *) => (
        let mut $command = std::process::Command::new($path);
        $(
            $command.arg($args);
        )*
        
        let $process = $command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect(&format!("Unable to start {} process", $path));

        let mut $stdin = $process
            .stdin
            .expect("Unable to get stdin for child process");
        let $stdout = $process
            .stdout
            .expect("Unable to get stdout for child process");
        let mut $stdout = std::io::BufReader::new($stdout);
    );
}

macro_rules! add_in_group {
    ($name:expr, $group:ident, $stdin:ident, $stdout:ident) => (
        $group.bench_function($name, |b| {
            b.iter_custom(|iters| {
                writeln!($stdin, "{}", iters).expect("Unable to send iteration count to child process");

                let mut line = String::new();
                $stdout.read_line(&mut line).expect("Unable to read time from child process");

                Duration::from_nanos(u64::from_str(line.trim()).expect("Unable to parse time from child process"))
            })
        });
    )
}

macro_rules! add_in_group_input {
    ($name:expr, $input:ident, $group:ident, $stdin:ident, $stdout:ident) => (
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

macro_rules! setup_group {
    ($group:ident) => (
        $group.warm_up_time(warmup_time());
        $group.sample_size(sample_size());
        $group.throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64));

        if std::path::Path::new("cpp/bin/kseq_16384").is_file() {
            create_command!("cpp/bin/kseq_16384", kseq_command, kseq_process, kseq_stdin, kseq_stdout, FILENAME);
            add_in_group!("kseq", $group, kseq_stdin, kseq_stdout);
        }

        if std::path::Path::new("cpp/bin/kseqpp").is_file() {
            create_command!("cpp/bin/kseqpp", kseqpp_command, kseqpp_process, kseqpp_stdin, kseqpp_stdout, FILENAME, "131072");
            add_in_group!("kseqpp", $group, kseqpp_stdin, kseqpp_stdout);
        } 

        if std::path::Path::new("cpp/bin/seqan").is_file() {
            create_command!("cpp/bin/seqan", seqan_command, seqan_process, seqan_stdin, seqan_stdout, FILENAME);
            add_in_group!("seqan", $group, seqan_stdin, seqan_stdout);
        }

        if std::path::Path::new("cpp/bin/bioparser").is_file() {
            create_command!("cpp/bin/bioparser", bioparser_command, bioparser_process, bioparser_stdin, bioparser_stdout, FILENAME);
            add_in_group!("bioparser", $group, bioparser_stdin, bioparser_stdout);
        }

        if std::path::Path::new("golang/bin/go_bio").is_file() {
            create_command!("golang/bin/go_bio", go_bio_command, go_bio_process, go_bio_stdin, go_bio_stdout, FILENAME);
            add_in_group!("go_bio", $group, go_bio_stdin, go_bio_stdout);
        }
        
        $group.bench_function("rust_bio",    |b| {b.iter(|| rust_bio(FILENAME, 8192));});
        $group.bench_function("memmap", |b| {b.iter(|| memmap(FILENAME));});
        $group.bench_function("buf_ref_map", |b| {b.iter(|| buf_ref_reader(FILENAME, 8*1024));});
        $group.bench_function("rust_needletail", |b| {b.iter(|| rust_needletail(FILENAME));});
    );
}

fn reference(c: &mut Criterion) {
    static FILENAME: &str = "sequences/reference.fasta";

    let mut group = c.benchmark_group("reference");

    setup_group!(group);
}

fn illumina(c: &mut Criterion) {
    static FILENAME: &str = "sequences/illumina.fasta";

    let mut group = c.benchmark_group("illumina");

    setup_group!(group);
}

fn nanopore(c: &mut Criterion) {
    static FILENAME: &str = "sequences/nanopore.fasta";

    let mut group = c.benchmark_group("nanopore");

    setup_group!(group);
}

fn buffer_size(c: &mut Criterion) {
    static FILENAME: &str = "sequences/nanopore.fasta";

    let mut group = c.benchmark_group("buffer_size");
    group.warm_up_time(warmup_time());
    group.sample_size(sample_size());
    group.throughput(Throughput::Bytes(std::fs::metadata(FILENAME).unwrap().len() as u64));
    
    for i in 5..20 {
        let buffer_size: usize = 1 << i;


        if std::path::Path::new(&format!("cpp/bin/kseq_{}", buffer_size)).is_file() {    
            create_command!(&format!("cpp/bin/kseq_{}", buffer_size), kseq_command, kseq_process, kseq_stdin, kseq_stdout, FILENAME);
            add_in_group_input!("kseq", buffer_size, group, kseq_stdin, kseq_stdout);
        }
        if std::path::Path::new("cpp/bin/kseqpp").is_file() {    
            create_command!("cpp/bin/kseqpp", kseq_command, kseq_process, kseq_stdin, kseq_stdout, FILENAME, buffer_size.to_string());
            add_in_group_input!("kseqpp", buffer_size, group, kseq_stdin, kseq_stdout);
        }
        group.bench_with_input(BenchmarkId::new("rust_bio", buffer_size), &buffer_size, |b, &buffer_size| {
            b.iter(|| rust_bio(FILENAME, buffer_size) );
        });
        group.bench_with_input(BenchmarkId::new("rust_bufref_map", buffer_size), &buffer_size, |b, &buffer_size| {
            b.iter(|| buf_ref_reader(FILENAME, buffer_size));
        });
    }
}

criterion_group!(b, reference, illumina, nanopore, buffer_size);
criterion_main!(b);

