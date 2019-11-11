// to compile: gcc this_prog.c -lz
#include <zlib.h>
#include <cstdio>
#include "kseq.h"
KSEQ_INIT(gzFile, gzread)

int main(int argc, char *argv[])
{
  gzFile fp;
  kseq_t *seq;
  int l;

  uint64_t nuc_count['T' + 1] = {0};
  
  fp = gzopen(argv[1], "r");
  seq = kseq_init(fp);

  while (kseq_read(seq) >= 0) {
    for(auto i = 0; i != seq->l; i++) {
      nuc_count[seq->s[i]] += 1;
    }
  }

  kseq_destroy(seq);
  gzclose(fp);
  return 0;
}
