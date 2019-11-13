#include <fstream>
#include <seqan3/core/debug_stream.hpp>
#include <seqan3/io/sequence_file/input.hpp>

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

      std::ifstream input(argv[1], std::ifstream::in);
      seqan3::sequence_file_input fin{input, seqan3::format_fasta{}};
      for (auto & rec : fin) {
	for(auto nuc: seqan3::get<seqan3::field::SEQ>(rec)) {
	  nuc_count[int(nuc.to_char())] += 1;
	}
      }
    }
    
    std::cout<<std::chrono::nanoseconds(std::chrono::system_clock::now() - begin).count()<<std::endl;
  }

  return 0;
}
