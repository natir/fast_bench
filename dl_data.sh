#!/bin/bash

mkdir -p sequences

curl https://www.ebi.ac.uk/ena/browser/api/fasta/CP028309?download=true | seqtk seq -A - > sequences/reference.fasta

curl ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR934/SRR934621/SRR934621.fastq.gz | seqtk seq -A - > sequences/illumina.fasta

curl ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR849/000/SRR8494940/SRR8494940_1.fastq.gz | seqtk seq -A - > sequences/nanopore.fasta

