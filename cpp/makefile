CXX=g++

CXX_FLAG=-Wredundant-decls -Wcast-align -Wmissing-declarations -Wmissing-include-dirs -Wswitch-enum -Wswitch-default -Wextra -Wall -Werror -Winvalid-pch -Wredundant-decls -Wformat=2 -Wmissing-format-attribute -Wformat-nonliteral -O3 -flto -march=native -mtune=native

LINK_FLAG=-lpthread

KSEQ_PATH=3party/readfq
KSEQ_INC=-I $(KSEQ_PATH)
KSEQ_FLAGS=$(KSEQ_INC)
KSEQ_TARGET=bin/kseq_32 bin/kseq_64 bin/kseq_128 bin/kseq_256 bin/kseq_512 bin/kseq_1024 bin/kseq_2048 bin/kseq_4096 bin/kseq_8192 bin/kseq_16384 bin/kseq_32768 bin/kseq_65536 bin/kseq_131072 bin/kseq_262144 bin/kseq_524288 

KSEQPP_PATH=3party/kseqpp
KSEQPP_INC=-I $(KSEQPP_PATH)/src/
KSEQPP_FLAGS=$(KSEQPP_INC)

BIOPARSER_PATH=3party/bioparser
BIOPARSER_INC=-I $(BIOPARSER_PATH)/include/
BIOPARSER_FLAGS=$(BIOPARSER_INC) -lz

SEQAN_PATH=3party/seqan
SEQAN_INC=-I $(SEQAN_PATH)/include/ -isystem $(SEQAN_PATH)/submodules/range-v3/include -isystem $(SEQAN_PATH)/submodules/sdsl-lite/include -isystem $(SEQAN_PATH)/submodules/cereal/include
SEQAN_FLAGS=$(SEQAN_INC) -DNDEBUG -std=c++17 -fconcepts -Wno-error=switch-default -Wno-error=switch-enum

all: $(KSEQ_TARGET) bin/kseqpp bin/bioparser bin/seqan

bin/kseq_%: kseq.cpp
	$(CXX) $(CXX_FLAG) $(LINK_FLAG) $(KSEQ_FLAGS) -DBUFF_SIZE=$* $< -o $@

bin/kseqpp: kseqpp.cpp
	$(CXX) $(CXX_FLAG) $(LINK_FLAG) $(KSEQPP_FLAGS) $< -o $@

bin/bioparser: bioparser.cpp
	$(CXX) $(CXX_FLAG) $(LINK_FLAG) $(BIOPARSER_FLAGS) $< -o $@

bin/seqan: seqan.cpp
	$(CXX) $(CXX_FLAG) $(LINK_FLAG) $(SEQAN_FLAGS) $< -o $@	
