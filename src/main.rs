fn main() {
    fast_bench::buf_ref_stream("sequences/illumina.fasta", 8 * 4096);
} 
