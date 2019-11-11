#include <zlib.h>
#include <cstdio>
#include <cstdint>
#include <iostream>

#include "bioparser.hpp"

// define a class for sequences in FASTA format
namespace bioparser {
  template<class T>
  class FastaParser;
}

class Sequence {
public:
  ~Sequence() = default;

  const std::string& name() const {
    return name_;
  }

  const std::string& data() const {
    return data_;
  }

  friend bioparser::FastaParser<Sequence>;
  friend std::unique_ptr<Sequence> createSequence(const std::string& name,
						  const std::string& data);
private:
  Sequence(const char* name, uint32_t name_length, const char* data,
	   uint32_t data_length)
    : name_(name, name_length), data_(){

    data_.reserve(data_length);
    for (uint32_t i = 0; i < data_length; ++i) {
      data_ += toupper(data[i]);
    }
  }

  
  Sequence(const std::string& name, const std::string& data)
    : name_(name), data_(data){}
  
  std::string name_;
  std::string data_;
};

int main(int argc, char *argv[]) {
  uint64_t nuc_count['T' + 1] = {0};

  if(argc != 2) {
    std::cerr<<"Usage kseq <fasta file>"<<std::endl;
    return -1;
  }
    
  std::vector<std::unique_ptr<Sequence>> fasta_objects;
  auto fasta_parser = bioparser::createParser<bioparser::FastaParser, Sequence>(argv[1]);

  while(true) {    
    auto status = fasta_parser->parse(fasta_objects, -1);
    
    for(auto const& seq: fasta_objects) {
      for(auto nuc: seq->data()) {
	nuc_count[int(nuc)] += 1;
      }
    }

    if(!status) {
      break;
    }
  }
}
