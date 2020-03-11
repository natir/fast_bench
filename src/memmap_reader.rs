pub struct FastaReader<'a> {
    pub mmap: &'a memmap::Mmap,
    pos: usize,
}

impl<'a> FastaReader<'a> {
    pub fn new(file: &'a memmap::Mmap) -> Self {
        FastaReader { mmap: file, pos: 0 }
    }
}

impl<'a> Iterator for FastaReader<'a> {
    type Item = (&'a [u8], &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.mmap.len() {
            return None;
        }

        let mut end_comment: usize = 0;
        let mut in_comment = true;

        for (offset, chara) in self.mmap[self.pos..].iter().enumerate() {
            if !in_comment && *chara == b'>' {
                let comment = &self.mmap[(self.pos + 1)..end_comment];
                let sequence = &self.mmap[(end_comment + 1)..(self.pos + offset - 1)];

                self.pos += offset;
                return Some((&comment, &sequence));
            }

            if in_comment && *chara == b'\n' {
                in_comment = false;
                end_comment = self.pos + offset;
            }
        }

        let comment = &self.mmap[(self.pos + 1)..end_comment];
        let sequence = &self.mmap[(end_comment + 1)..(self.mmap.len() - 1)];
        self.pos = self.mmap.len();

        Some((&comment, &sequence))
    }
}
