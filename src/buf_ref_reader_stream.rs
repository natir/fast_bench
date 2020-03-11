use futures;
use futures::stream::Stream;

pub struct BufRefStream {
    reader: buf_ref_reader::BufRefReader<std::fs::File, buf_ref_reader::MmapBuffer>,
}

impl BufRefStream {
    pub fn new(filename: &str, buffer_size: usize) -> Self {
	let r = buf_ref_reader::BufRefReaderBuilder::new(std::fs::File::open(filename).unwrap())
		.capacity(buffer_size)
		.build::<buf_ref_reader::MmapBuffer>()
		.unwrap();
	
	BufRefStream {
	    reader: r,
	}
    }
}

impl Stream for BufRefStream {
    type Item = (String, String);
    
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut futures::task::Context) -> futures::task::Poll<Option<Self::Item>> {

	let unpin: &mut Self = std::pin::Pin::into_inner(self);

	let c;
	let s;
	
	if let Ok(Some(comment)) = unpin.reader.read_until(b'\n') {
	    c = unsafe { String::from_utf8_unchecked(comment.to_vec()) };
	} else {
            return futures::task::Poll::Ready(None);
	}
	
        if let Ok(Some(sequence)) = unpin.reader.read_until(b'\n') {
	    s = unsafe { String::from_utf8_unchecked(sequence.to_vec()) };
	} else {
	    return futures::task::Poll::Ready(None);
	}
	
	futures::task::Poll::Ready(Some((c, s)))
    }
}
