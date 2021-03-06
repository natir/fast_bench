extern crate bio;
extern crate needletail;
extern crate seq_io;

use needletail::Sequence;

use seq_io::fasta::Record;

use memmap::MmapOptions;

mod fasten_like;

struct MemmapFastaReader<'a> {
    pub mmap: &'a memmap::Mmap,
    pos: usize,
}

impl<'a> MemmapFastaReader<'a> {
    pub fn new(file: &'a memmap::Mmap) -> Self {
        MemmapFastaReader { mmap: file, pos: 0 }
    }
}

impl<'a> Iterator for MemmapFastaReader<'a> {
    type Item = (&'a [u8], &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.mmap.len() {
            return None;
        }

        let mut end_comment: usize = 0;
        let mut in_comment = true;

        for (offset, chara) in self.mmap[self.pos..].iter().enumerate() {
            if !in_comment && *chara == b'>' {
                let comment = &self.mmap[(self.pos + 1)..end_comment];
                let sequence = &self.mmap[(end_comment + 1)..(self.pos + offset - 1)];

                self.pos += offset;
                return Some((&comment, &sequence));
            }

            if in_comment && *chara == b'\n' {
                in_comment = false;
                end_comment = self.pos + offset;
            }
        }

        let comment = &self.mmap[(self.pos + 1)..end_comment];
        let sequence = &self.mmap[(end_comment + 1)..(self.mmap.len() - 1)];
        self.pos = self.mmap.len();

        Some((&comment, &sequence))
    }
}

pub fn memmap(filename: &str) {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let mmap = unsafe {
        MmapOptions::new()
            .map(&file)
            .expect("Error when we try to map file in mem")
    };

    let parser = MemmapFastaReader::new(&mmap);

    for (_comment, sequence) in parser {
        for nuc in sequence {
            nuc_counter[*nuc as usize] += 1;
        }
    }
}

pub fn buf_ref_reader(filename: &str, buffer_size: usize) {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");

    let mut mmap = buf_ref_reader::BufRefReaderBuilder::new(file)
        .capacity(buffer_size)
        .build::<buf_ref_reader::MmapBuffer>()
        .unwrap();

    let mut counter = -1;
    loop {
        counter += 1;

        if counter % 2 == 0 {
            if let Ok(Some(_)) = mmap.read_until(b'\n') {
                continue;
            } else {
                break;
            }
        }

        if let Ok(Some(line)) = mmap.read_until(b'\n') {
            for nuc in line {
                nuc_counter[*nuc as usize] += 1;
            }
        } else {
            break;
        }
    }
}

pub fn rust_bio(filename: &str, buffer_size: usize) {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let reader = bio::io::fasta::Reader::with_capacity(buffer_size, file);

    for r in reader.records() {
        let result = r.expect("Error when we parse file");

        for nuc in result.seq() {
            nuc_counter[*nuc as usize] += 1;
        }
    }
}

pub fn needletail(filename: &str) {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    needletail::parse_sequence_path(
        filename,
        |_| {},
        |seq| {
            for nuc in seq.sequence() {
                nuc_counter[*nuc as usize] += 1;
            }
        },
    )
    .expect("Parsing failed");
}

pub fn seq_io(filename: &str) -> () {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let mut reader = seq_io::fasta::Reader::new(file);

    while let Some(result) = reader.next() {
        let record = result.unwrap();

        for nuc in record.seq() {
            nuc_counter[*nuc as usize] += 1;
        }
    }
}

pub fn fasten_like(filename: &str) -> () {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let mut reader = fasten_like::FastaReader::new(file);

    while let Some(record) = reader.next() {
        for nuc in record.seq.bytes() {
            nuc_counter[nuc as usize] += 1;
        }
    }
}

use std::sync::mpsc::channel;
use std::thread;

pub fn multithread(filename: &str, buffer_size: usize) {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let (sender, receiver) = channel();

    let filename2 = filename.to_string();
    thread::spawn(move || {
        buf_ref_reader_on_separate_thread(filename2, buffer_size, &sender);
    });

    while let Ok(message) = receiver.recv() {
        if let Some(line) = message {
            for nuc in line.bytes() {
                nuc_counter[nuc as usize] += 1;
            }
        }
    }
}

pub fn buf_ref_reader_on_separate_thread(
    filename: String,
    buffer_size: usize,
    sender: &std::sync::mpsc::Sender<Option<String>>,
) {
    let file = std::fs::File::open(filename).expect("Error when we try to open file");

    let mut mmap = buf_ref_reader::BufRefReaderBuilder::new(file)
        .capacity(buffer_size)
        .build::<buf_ref_reader::MmapBuffer>()
        .unwrap();

    let mut counter = -1;
    loop {
        counter += 1;

        if counter % 2 == 0 {
            if let Ok(Some(_)) = mmap.read_until(b'\n') {
                continue;
            } else {
                break;
            }
        }

        if let Ok(Some(line)) = mmap.read_until(b'\n') {
            unsafe {
                sender
                    .send(Some(String::from_utf8_unchecked(line.to_vec())))
                    .unwrap();
            }
        } else {
            break;
        }
    }

    sender.send(None).unwrap();
}
