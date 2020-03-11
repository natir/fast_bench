use tokio::prelude::*;

async fn read_async(filename: &str, nuc_counter: &mut [u64; 85]) {
    let reader = tokio::fs::File::open(filename).await.unwrap();
    let bufreader = tokio::io::BufStream::new(reader);
    
    let mut iterator = bufreader.lines();
    
    while let Ok(Some(comment)) = iterator.next_line().await {
        if let Ok(Some(seq)) = iterator.next_line().await {
	    for nuc in seq.bytes() {
		nuc_counter[nuc as usize] += 1;
	    }
        }
    }
}

pub fn read(filename: &str) {
    let mut rt = tokio::runtime::Builder::new()
	.threaded_scheduler()
	.build()
	.unwrap();
    let mut nuc_counter: [u64; 85] = [0; ('T' as usize) + 1];
    
    rt.block_on(read_async(filename, &mut nuc_counter));
}

pub fn kmer_counter(filename: &str) {
    let mut rt = tokio::runtime::Builder::new()
	.threaded_scheduler()
	.build()
	.unwrap();

    let mut hash: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    rt.block_on(cmpt_async(filename, &mut hash));
}

async fn cmpt_async(filename: &str, hash: &mut std::collections::HashMap<String, usize>) {
        let reader = tokio::fs::File::open(filename).await.unwrap();
    let bufreader = tokio::io::BufStream::new(reader);
    
    let mut iterator = bufreader.lines();
    
    while let Ok(Some(comment)) = iterator.next_line().await {
        if let Ok(Some(seq)) = iterator.next_line().await {
	    for kmer in seq.as_bytes().windows(11) {
		*hash.entry(String::from_utf8(kmer.to_vec()).unwrap()).or_default() += 1;
	    }
        }
    }
}
