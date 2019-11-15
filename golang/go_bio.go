package main

import (
	"io"
	"os"
	"fmt"
	"time"
	"bufio"
	
	"github.com/shenwei356/bio/seq"
	"github.com/shenwei356/bio/seqio/fastx"
)

func main() {

	if len(os.Args) != 2 {
		panic("Usage: go_bio <fasta file>")
	}
	seq.ValidateSeq = false

	var d int
	stdin := bufio.NewReader(os.Stdin)
	for {
		_, err := fmt.Fscan(stdin, &d)
		if err == io.EOF {
			break
		}

		var begin = time.Now()
		for i := 0; i != d; i++ {
			reader, err := fastx.NewDefaultReader(os.Args[1])
			checkError(err)
			var nuc_counter [86]uint64
			for {
				record, err := reader.Read()
				if err != nil {
					if err == io.EOF {
						break
					}
					checkError(err)
					break
				}

				for _, nuc := range record.Seq.Seq {
					nuc_counter[nuc] += 1
				}
			}
		}
		
		var end = time.Now()
		fmt.Printf("%d\n", end.Sub(begin).Nanoseconds())
	}
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
