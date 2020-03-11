extern crate criterion;
extern crate fast_bench;

use criterion::*;
use fast_bench::*;

use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;
use std::time::Duration;

fn warmup_time() -> std::time::Duration {
    return match std::env::var("WARMUP_TIME") {
        Ok(val) => Duration::from_secs(
            val.parse::<u64>()
                .expect("Error we can parse WARMUP_TIME in u64"),
        ),
        Err(_e) => Duration::from_secs(2),
    };
}

fn sample_size() -> usize {
    return match std::env::var("SAMPLE_SIZE") {
        Ok(val) => val
            .parse::<usize>()
            .expect("Error we can parse WARMUP_TIME in u64"),
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
    ($name:expr, $group:ident, $stdin:ident, $stdout:ident) => {
        $group.bench_function($name, |b| {
            b.iter_custom(|iters| {
                writeln!($stdin, "{}", iters)
                    .expect("Unable to send iteration count to child process");

                let mut line = String::new();
                $stdout
                    .read_line(&mut line)
                    .expect("Unable to read time from child process");

                Duration::from_nanos(
                    u64::from_str(line.trim()).expect("Unable to parse time from child process"),
                )
            })
        });
    };
}

macro_rules! add_in_group_input {
    ($name:expr, $input:ident, $group:ident, $stdin:ident, $stdout:ident) => {
        $group.bench_with_input(BenchmarkId::new($name, $input), &$input, |b, &$input| {
            b.iter_custom(|iters| {
                writeln!($stdin, "{}", iters)
                    .expect("Unable to send iteration count to child process");

                let mut line = String::new();
                $stdout
                    .read_line(&mut line)
                    .expect("Unable to read time from child process");

                Duration::from_nanos(
                    u64::from_str(line.trim()).expect("Unable to parse time from child process"),
                )
            })
        });
    };
}

macro_rules! setup_group {
    ($group:ident) => {
        $group.warm_up_time(warmup_time());
        $group.sample_size(sample_size());
        $group.throughput(Throughput::Bytes(
            std::fs::metadata(FILENAME).unwrap().len() as u64,
        ));

        if std::path::Path::new("cpp/bin/kseq_16384").is_file() {
            create_command!(
                "cpp/bin/kseq_16384",
                kseq_command,
                kseq_process,
                kseq_stdin,
                kseq_stdout,
                FILENAME
            );
            add_in_group!("kseq", $group, kseq_stdin, kseq_stdout);
        }

        if std::path::Path::new("cpp/bin/kseqpp").is_file() {
            create_command!(
                "cpp/bin/kseqpp",
                kseqpp_command,
                kseqpp_process,
                kseqpp_stdin,
                kseqpp_stdout,
                FILENAME,
                "131072"
            );
            add_in_group!("kseqpp", $group, kseqpp_stdin, kseqpp_stdout);
        }

        if std::path::Path::new("cpp/bin/seqan").is_file() {
            create_command!(
                "cpp/bin/seqan",
                seqan_command,
                seqan_process,
                seqan_stdin,
                seqan_stdout,
                FILENAME
            );
            add_in_group!("seqan", $group, seqan_stdin, seqan_stdout);
        }

        if std::path::Path::new("cpp/bin/bioparser").is_file() {
            create_command!(
                "cpp/bin/bioparser",
                bioparser_command,
                bioparser_process,
                bioparser_stdin,
                bioparser_stdout,
                FILENAME
            );
            add_in_group!("bioparser", $group, bioparser_stdin, bioparser_stdout);
        }

        if std::path::Path::new("golang/bin/go_bio").is_file() {
            create_command!(
                "golang/bin/go_bio",
                go_bio_command,
                go_bio_process,
                go_bio_stdin,
                go_bio_stdout,
                FILENAME
            );
            add_in_group!("go_bio", $group, go_bio_stdin, go_bio_stdout);
        }

        $group.bench_function("rust_bio", |b| {
            b.iter(|| rust_bio(FILENAME, 8192));
        });
        $group.bench_function("memmap", |b| {
            b.iter(|| memmap(FILENAME));
        });
        $group.bench_function("buf_ref_map", |b| {
            b.iter(|| buf_ref_reader(FILENAME, 8 * 1024));
        });
        $group.bench_function("needletail", |b| {
            b.iter(|| needletail(FILENAME));
        });
        $group.bench_function("seq_io", |b| {
            b.iter(|| seq_io(FILENAME));
        });
        $group.bench_function("fasten", |b| {
            b.iter(|| seq_io(FILENAME));
        });
        $group.bench_function("multithread", |b| {
            b.iter(|| multithread(FILENAME, 8 * 1024));
        });
	$group.bench_function("tokio", |b| {
            b.iter(|| tokio_nuc_counter::read(FILENAME));
        });
	$group.bench_function("futures_stream", |b| {
            b.iter(|| buf_ref_stream(FILENAME, 8 * 1024));
        });
    };
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
    group.throughput(Throughput::Bytes(
        std::fs::metadata(FILENAME).unwrap().len() as u64,
    ));

    for i in 5..20 {
        let buffer_size: usize = 1 << i;

        if std::path::Path::new(&format!("cpp/bin/kseq_{}", buffer_size)).is_file() {
            create_command!(
                &format!("cpp/bin/kseq_{}", buffer_size),
                kseq_command,
                kseq_process,
                kseq_stdin,
                kseq_stdout,
                FILENAME
            );
            add_in_group_input!("kseq", buffer_size, group, kseq_stdin, kseq_stdout);
        }
        if std::path::Path::new("cpp/bin/kseqpp").is_file() {
            create_command!(
                "cpp/bin/kseqpp",
                kseq_command,
                kseq_process,
                kseq_stdin,
                kseq_stdout,
                FILENAME,
                buffer_size.to_string()
            );
            add_in_group_input!("kseqpp", buffer_size, group, kseq_stdin, kseq_stdout);
        }

        group.bench_with_input(
            BenchmarkId::new("rust_bio", buffer_size),
            &buffer_size,
            |b, &buffer_size| {
                b.iter(|| rust_bio(FILENAME, buffer_size));
            },
        );
	
        group.bench_with_input(
            BenchmarkId::new("rust_bufref_map", buffer_size),
            &buffer_size,
            |b, &buffer_size| {
                b.iter(|| buf_ref_reader(FILENAME, buffer_size));
            },
        );
    }
}

fn rust_bio_cache_nanopore(c: &mut Criterion) {
    let mut g1 = c.benchmark_group("rust_bio");
    g1.warm_up_time(warmup_time());
    g1.sample_size(sample_size());
    g1.throughput(Throughput::Bytes(
        std::fs::metadata("sequences/nanopore.fasta").unwrap().len() as u64,
    ));

    for i in 8..24 {
        let buffer_size: usize = 1 << i;

	g1.bench_with_input(
            BenchmarkId::new("nanopore", buffer_size),
            &buffer_size,
            |b, &buffer_size| {
                b.iter(|| rust_bio("sequences/nanopore.fasta", buffer_size));
            },
        );
    }
}

fn rust_bio_cache_illumina(c: &mut Criterion) {
    let mut g2 = c.benchmark_group("rust_bio");
    g2.warm_up_time(warmup_time());
    g2.sample_size(sample_size());
    g2.throughput(Throughput::Bytes(
        std::fs::metadata("sequences/illumina.fasta").unwrap().len() as u64,
    ));

    for i in 8..24 {
        let buffer_size: usize = 1 << i;

	g2.bench_with_input(
            BenchmarkId::new("illumina", buffer_size),
            &buffer_size,
            |b, &buffer_size| {
                b.iter(|| rust_bio("sequences/illumina.fasta", buffer_size));
            },
        );
    }
}

fn rust_bio_cache_reference(c: &mut Criterion) {
    let mut g3 = c.benchmark_group("rust_bio");
    g3.warm_up_time(warmup_time());
    g3.sample_size(sample_size());
    g3.throughput(Throughput::Bytes(
        std::fs::metadata("sequences/reference.fasta").unwrap().len() as u64,
    ));

    for i in 8..24 {
        let buffer_size: usize = 1 << i;
	
	g3.bench_with_input(
	    BenchmarkId::new("reference", buffer_size),
            &buffer_size,
            |b, &buffer_size| {
                b.iter(|| rust_bio("sequences/reference.fasta", buffer_size));
            },
        );
    }
}

//criterion_group!(b, reference, illumina, nanopore, buffer_size, rust_bio_cache);
criterion_group!(b, rust_bio_cache_reference, rust_bio_cache_nanopore, rust_bio_cache_illumina);
criterion_main!(b);
