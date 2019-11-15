#include <cstdio>
#include <cstdint>
#include <iostream>

#include <unistd.h>
#include <fcntl.h>

#include <chrono>

#include "kseq.h"

#ifndef BUFF_SIZE
#define BUFF_SIZE 16384
#endif // BUFF_SIZE

KSTREAM_INIT2(static, int, read, BUFF_SIZE)
__KSEQ_TYPE(int)
__KSEQ_BASIC(static, int)
__KSEQ_READ(static)

int main(int argc, char *argv[]) {
  if(argc != 2) {
    std::cerr<<"Usage kseq <fasta file>"<<std::endl;
    return -1;
  }

  for (std::string line; std::getline(std::cin, line);) {
    unsigned long iters = std::stoul(line);

    auto begin = std::chrono::system_clock::now();

    for(long unsigned i = 0; i != iters; i++) {
      kseq_t *seq;
  
      uint64_t nuc_count['T' + 1] = {0};
  
      int fp = open(argv[1], O_RDONLY);
      seq = kseq_init(fp);

      while (kseq_read(seq) >= 0) {
	for(unsigned int i = 0; i != seq->seq.l; i++) {
	  nuc_count[int(seq->seq.s[i])] += 1;
	}
      }

      kseq_destroy(seq);
      close(fp);
    }
    
    std::cout<<std::chrono::nanoseconds(std::chrono::system_clock::now() - begin).count()<<std::endl;
  }

  return 0;
}
