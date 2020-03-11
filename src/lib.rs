extern crate bio;
extern crate needletail;
extern crate seq_io;
extern crate tempfile;

use needletail::Sequence;

use seq_io::fasta::Record;

use memmap::MmapOptions;

use futures::stream::Stream;
use futures::stream::StreamExt; // for `try_next`
 

pub mod tokio_nuc_counter;
mod fasten_like;
mod memmap_reader;
mod buf_ref_reader_stream;

pub fn memmap(filename: &str) -> [u64; 85] {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let mmap = unsafe {
        MmapOptions::new()
            .map(&file)
            .expect("Error when we try to map file in mem")
    };

    let parser = memmap_reader::FastaReader::new(&mmap);

    for (_comment, sequence) in parser {
        for nuc in sequence {
            nuc_counter[*nuc as usize] += 1;
        }
    }

    return nuc_counter;
}

pub fn buf_ref_reader(filename: &str, buffer_size: usize) -> [u64; 85] {
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
            if let Ok(Some(line)) = mmap.read_until(b'\n') {
		let comment = unsafe { String::from_utf8_unchecked(line.to_vec()) };
                continue;
            } else {
                break;
            }
        }

        if let Ok(Some(line)) = mmap.read_until(b'\n') {
	    let seq = unsafe { String::from_utf8_unchecked(line.to_vec()) };
	    for nuc in seq.bytes() {
		nuc_counter[nuc as usize] += 1;
	    }
        } else {
            break;
        }
    }
    
    
    return nuc_counter;
}

pub fn rust_bio(filename: &str, buffer_size: usize) -> [u64; 85] {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let reader = bio::io::fasta::Reader::with_capacity(buffer_size, file);

    for r in reader.records() {
        let result = r.expect("Error when we parse file");

        for nuc in result.seq() {
            nuc_counter[*nuc as usize] += 1;
        }
    }

    return nuc_counter;
}

pub fn needletail(filename: &str) -> [u64; 85] {
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

    return nuc_counter;
}

pub fn seq_io(filename: &str) -> [u64; 85] {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let mut reader = seq_io::fasta::Reader::new(file);

    while let Some(result) = reader.next() {
        let record = result.unwrap();

        for nuc in record.seq() {
            nuc_counter[*nuc as usize] += 1;
        }
    }

    return nuc_counter;
}

pub fn fasten_like(filename: &str) -> [u64; 85] {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let file = std::fs::File::open(filename).expect("Error when we try to open file");
    let mut reader = fasten_like::FastaReader::new(file);

    while let Some(record) = reader.next() {
	let mut iter = record.seq.bytes();
	
	let mut nuc = iter.next();
	let mut next = iter.next();
	while next != None {
	    nuc_counter[nuc.unwrap() as usize] += 1;
	    nuc = next;
	    next = iter.next();
	}
    }

    return nuc_counter;
}

use std::sync::mpsc::channel;
use std::thread;

pub fn multithread(filename: &str, buffer_size: usize) -> [u64; 85] {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];

    let (sender, receiver) = channel();

    let filename2 = filename.to_string();
    thread::spawn(move || {
        buf_ref_reader_on_separate_thread(filename2, buffer_size, &sender);
    });

    while let Ok(message) = receiver.recv() {
        if let Some(line) = message {
	    let mut iter = line.bytes();
	
	    let mut nuc = iter.next();
	    let mut next = iter.next();
	    while next != None {
		nuc_counter[nuc.unwrap() as usize] += 1;
		nuc = next;
		next = iter.next();
	    }
        }
    }

    return nuc_counter;
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

pub fn buf_ref_stream(filename: &str, buffer_size: usize) -> [u64; 85] {
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
    let mut reader = buf_ref_reader_stream::BufRefStream::new(&filename, buffer_size);

    futures::executor::block_on(
	reader.for_each_concurrent(10,
				   |(_, seq)| async move {
				       for nuc in seq.bytes() {
					   nuc_counter[nuc as usize] += 1
				       }
				   }
	)
    );

    nuc_counter
}


#[cfg(test)]
mod tests {

    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn mmemap() {
        let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::memmap(file.path().to_str().unwrap());

	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    /*
    #[test]
    fn buf_ref_reader() {
	let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::buf_ref_reader(file.path().to_str().unwrap(), 8 * 1024);
	
	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
    */

    #[test]
    fn buf_ref_reader_stream() {
	let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::buf_ref_stream(file.path().to_str().unwrap(), 8 * 1024);
	
	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

//	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn buf_ref_reader_multithread() {
	let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::multithread(file.path().to_str().unwrap(), 8 * 1024);
	
	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn rust_bio() {
        let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::rust_bio(file.path().to_str().unwrap(), 8192);

	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn needletail() {
        let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::needletail(file.path().to_str().unwrap());

	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
    
    #[test]
    fn seq_io() {
        let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::seq_io(file.path().to_str().unwrap());

	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn fasten_like() {
        let file = NamedTempFile::new().expect("Can't create tmpfile");

	{
	    let mut wfile = file.reopen().expect("Can't create tmpfile");
	    writeln!(wfile, ">1\nACTG").unwrap();
	}

	let count = super::fasten_like(file.path().to_str().unwrap());

	let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
	nuc_counter['A' as usize] += 1;
	nuc_counter['C' as usize] += 1;
	nuc_counter['G' as usize] += 1;
	nuc_counter['T' as usize] += 1;

	assert!(count.iter().zip(nuc_counter.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}
