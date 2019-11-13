#include <seqan/seq_io.h>

#include <chrono>

int main(int argc, char *argv[])
{
  if(argc != 2) {
    std::cerr<<"Usage kseq <fasta file>"<<std::endl;
    return -1;
  }

  for (std::string line; std::getline(std::cin, line);) {
    unsigned long iters = std::stoul(line);

    auto begin = std::chrono::system_clock::now();
    for(long unsigned i = 0; i != iters; i++) {
	
      uint64_t nuc_count['T' + 1] = {0};

      seqan::CharString seqFileName = argv[1];
      seqan::StringSet<seqan::CharString> ids;
      seqan::StringSet<seqan::Dna5String> seqs;

      seqan::SeqFileIn file_in(seqan::toCString(seqFileName));
      seqan::readRecords(ids, seqs, file_in);

      for(auto seq: seqs) {
	for(auto nuc: seq) {
	  nuc_count[int(nuc)] += 1;
	}
      }
    }
    
    std::cout<<std::chrono::nanoseconds(std::chrono::system_clock::now() - begin).count()<<std::endl;
  }

  return 0;
}
