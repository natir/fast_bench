#include <iostream>

#include <unistd.h>
#include <fcntl.h>

#include "kseq++.h"

int main(int argc, char* argv[]) {
  unsigned int buffer_size = 131072;

  if(argc > 3 && argc < 2) {
    std::cerr<<"Usage kseqpp <fasta file> [buffer size]"<<std::endl;
    return -1;
  }

  if(argc == 3) {
    buffer_size = std::stoul(argv[2]);
  }

  for (std::string line; std::getline(std::cin, line);) {
    unsigned long iters = std::stoul(line);

    auto begin = std::chrono::system_clock::now();

    for(long unsigned i = 0; i != iters; i++) {
      uint64_t nuc_count['T' + 1] = {0};

      klibpp::KSeq record;
      int fp = open(argv[1], O_RDONLY);
      auto ks = klibpp::make_kstream(fp, read, klibpp::mode::in, buffer_size);

      while (ks >> record) {
	for(auto nuc: record.seq) {
	  nuc_count[int(nuc)] += 1;
	}
      }

      close(fp);
    }

    std::cout<<std::chrono::nanoseconds(std::chrono::system_clock::now() - begin).count()<<std::endl;
  }
}
