extern crate bio;

use std::process::Command;

use memmap::MmapOptions;

pub fn memmap(filename: &str) -> () {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
    
    let file = std::fs::File::open(filename).expect("Error when we try to open file");

    let mmap = unsafe { MmapOptions::new().map(&file).expect("Error when we try to map file in mem") };

    let mut in_comment = true;
    for chara in mmap.iter() {
        if in_comment && *chara == b'\n' {
            in_comment = false;
        }

        if !in_comment && *chara == b'>' {
            in_comment = true;
        }

        if in_comment {
            continue;
        } else {
            nuc_counter[*chara as usize] += 1;
        }
    }
}

pub fn rust_bio_buffered(filename: &str, buffer_size: usize) -> () {
    let file = std::io::BufReader::with_capacity(buffer_size, std::fs::File::open(filename).expect("Error when we try to open file"));

    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
    
    let reader = bio::io::fasta::Reader::new(file);
    for r in reader.records() {
        let result = r.expect("Error when we parse file");

        for nuc in result.seq() {
            nuc_counter[*nuc as usize] += 1;
        }
    }
}

pub fn rust_bio_unbuffered(filename: &str) -> () {
    let file = std::fs::File::open(filename).expect("Error when we try to open file");

    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
    
    let reader = bio::io::fasta::Reader::new(file);
    for r in reader.records() {
        let result = r.expect("Error when we parse file");

        for nuc in result.seq() {
            nuc_counter[*nuc as usize] += 1;
        }
    }
}

pub fn cat(filename: &str) -> () {
    let mut command = Command::new("cat");
    command.arg(filename);
    command.stdout(std::process::Stdio::null());

    command.spawn().expect("Error in subcommand launch").wait().expect("Error in subcommand execution");
}

pub fn kseq(filename: &str, buffer_size: usize) -> () {
    let mut command = Command::new(format!("./cpp/kseq_{}", buffer_size));
    command.arg(filename);
    command.stdout(std::process::Stdio::null());
    
    command.spawn().expect("Error in subcommand launch").wait().expect("Error in subcommand execution");
}

pub fn seqan(filename: &str) -> () {
    let mut command = Command::new("./cpp/seqan");
    command.arg(filename);
    command.stdout(std::process::Stdio::null());
    
    command.spawn().expect("Error in subcommand launch").wait().expect("Error in subcommand execution");
}

pub fn bioparser(filename: &str) -> () {
    let mut command = Command::new("./cpp/bioparser");
    command.arg(filename);
    command.stdout(std::process::Stdio::null());

    command.spawn().expect("Error in subcommand launch").wait().expect("Error in subcommand execution");
}
