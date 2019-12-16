use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub struct FastaReader<R: io::Read> {
    reader:           io::BufReader<R>,
}

impl<R: io::Read> FastaReader<R>{
    pub fn new(reader: R) -> FastaReader<R> {
        FastaReader {
            reader: BufReader::new(reader),
        }
    }
}

pub struct Seq {
    pub id:     String,
    pub seq:    String,
}

impl Seq {
    fn new (id: &String, seq: &String) -> Seq{
        Seq {
            id:     id.clone(),
            seq:    seq.clone(),
        }
    }
}

impl<R: Read> Iterator for FastaReader<R> {
    type Item = Seq;

    fn next(&mut self) -> Option<Seq> {
        let mut id=    String::new();
        let mut seq=   String::new();

        // Read the ID of the entry
        match self.reader.read_line(&mut id) {
            Ok(n) => {
                // if we're expecting an ID line, but
                // there are zero bytes read, then we are
                // at the end of the file. Break.
                if n < 1 {
                    return None;
                }
            }
            Err(error) => {
                panic!("ERROR: could not read the ID line: {}",error);
            }
        };

        self.reader.read_line(&mut seq).expect("ERROR: could not read sequence line");
        
        let seq = Seq::new(&id, &seq);

        return Some(seq);
    }
}

