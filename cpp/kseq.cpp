#include <zlib.h>
#include <cstdio>
#include <cstdint>
#include <iostream>

#include <chrono>

#include "kseq.h"

#ifndef BUFF_SIZE
#define BUFF_SIZE 16384
#endif // BUFF_SIZE

KSTREAM_INIT2(static, gzFile, gzread, BUFF_SIZE)
__KSEQ_TYPE(gzFile)
__KSEQ_BASIC(static, gzFile)
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
      gzFile fp;
      kseq_t *seq;
  
      uint64_t nuc_count['T' + 1] = {0};
  
      fp = gzopen(argv[1], "r");
      seq = kseq_init(fp);

      while (kseq_read(seq) >= 0) {
	for(unsigned int i = 0; i != seq->seq.l; i++) {
	  nuc_count[int(seq->seq.s[i])] += 1;
	}
      }

      kseq_destroy(seq);
      gzclose(fp);
    }
    
    std::cout<<std::chrono::nanoseconds(std::chrono::system_clock::now() - begin).count()<<std::endl;
  }

  return 0;
}
