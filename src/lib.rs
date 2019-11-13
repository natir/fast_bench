extern crate bio;

use memmap::MmapOptions;

struct MemmapFastaReader {
    pub mmap: memmap::Mmap,
    pub comment_slice: Vec<std::ops::Range<usize>>,
    pub sequence_slice: Vec<std::ops::Range<usize>>,
}

impl MemmapFastaReader {
    pub fn new(file: &std::fs::File) -> Self {
        MemmapFastaReader{
            mmap: unsafe { MmapOptions::new().map(file).expect("Error when we try to map file in mem") },
            comment_slice: Vec::new(),
            sequence_slice: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut begin_comment: usize = 1;
        let mut end_comment: usize = 0;
        let mut begin_sequence: usize = 0;
        let mut end_sequence: usize;
        
        let mut in_comment = true;
        for (offset, chara) in self.mmap.iter().enumerate() {
            if in_comment && *chara == b'\n' {
                end_comment = offset;
                begin_sequence = offset + 1; // Sequence begin is next character
                in_comment = false;
            }

            if !in_comment && *chara == b'>' {
                if begin_comment != end_comment {
                    end_sequence = offset - 1; // Sequence end is two next character
                    
                    self.comment_slice.push(begin_comment..end_comment);
                    self.sequence_slice.push(begin_sequence..end_sequence);

                    begin_comment = offset + 1; // Comment begin in next character
                }
                
                in_comment = true;
            }
        }

        // The last end_sequence wasn't set
        end_sequence = self.mmap.len() - 1;
        self.comment_slice.push(begin_comment..end_comment);
        self.sequence_slice.push(begin_sequence..end_sequence);
    } 
}

pub fn memmap(filename: &str) -> () {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
    
    let file = std::fs::File::open(filename).expect("Error when we try to open file");

    let mut mmap = MemmapFastaReader::new(&file);

    mmap.parse();

    for (com, seq) in mmap.comment_slice.into_iter().zip(mmap.sequence_slice) {
        let _comment = &mmap.mmap.as_ref()[com];
        let sequence = &mmap.mmap.as_ref()[seq];
        
        for nuc in sequence {
            nuc_counter[*nuc as usize] += 1;
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
