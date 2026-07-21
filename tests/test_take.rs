#![warn(rust_2018_idioms)]

use bytes::buf::Buf;
use bytes::Bytes;

#[test]
fn long_take() {
    // Tests that get a take with a size greater than the buffer length will not
    // overrun the buffer. Regression test for #138.
    let buf = b"hello world".take(100);
    assert_eq!(11, buf.remaining());
    assert_eq!(b"hello world", buf.chunk());
}

#[test]
fn take_copy_to_bytes() {
    let mut abcd = Bytes::copy_from_slice(b"abcd");
    let abcd_ptr = abcd.as_ptr();
    let mut take = (&mut abcd).take(2);
    let a = take.copy_to_bytes(1);
    assert_eq!(Bytes::copy_from_slice(b"a"), a);
    // assert `to_bytes` did not allocate
    assert_eq!(abcd_ptr, a.as_ptr());
    assert_eq!(Bytes::copy_from_slice(b"bcd"), abcd);
}

#[test]
#[should_panic]
fn take_copy_to_bytes_panics() {
    let abcd = Bytes::copy_from_slice(b"abcd");
    abcd.take(2).copy_to_bytes(3);
}

#[cfg(feature = "std")]
#[test]
fn take_chunks_vectored() {
    fn chain() -> impl Buf {
        Bytes::from([1, 2, 3].to_vec()).chain(Bytes::from([4, 5, 6].to_vec()))
    }

    {
        let mut dst = [std::io::IoSlice::new(&[]); 2];
        let take = chain().take(0);
        assert_eq!(take.chunks_vectored(&mut dst), 0);
    }

    {
        let mut dst = [std::io::IoSlice::new(&[]); 2];
        let take = chain().take(1);
        assert_eq!(take.chunks_vectored(&mut dst), 1);
        assert_eq!(&*dst[0], &[1]);
    }

    {
        let mut dst = [std::io::IoSlice::new(&[]); 2];
        let take = chain().take(3);
        assert_eq!(take.chunks_vectored(&mut dst), 1);
        assert_eq!(&*dst[0], &[1, 2, 3]);
    }

    {
        let mut dst = [std::io::IoSlice::new(&[]); 2];
        let take = chain().take(4);
        assert_eq!(take.chunks_vectored(&mut dst), 2);
        assert_eq!(&*dst[0], &[1, 2, 3]);
        assert_eq!(&*dst[1], &[4]);
    }

    {
        let mut dst = [std::io::IoSlice::new(&[]); 2];
        let take = chain().take(6);
        assert_eq!(take.chunks_vectored(&mut dst), 2);
        assert_eq!(&*dst[0], &[1, 2, 3]);
        assert_eq!(&*dst[1], &[4, 5, 6]);
    }

    {
        let mut dst = [std::io::IoSlice::new(&[]); 2];
        let take = chain().take(7);
        assert_eq!(take.chunks_vectored(&mut dst), 2);
        assert_eq!(&*dst[0], &[1, 2, 3]);
        assert_eq!(&*dst[1], &[4, 5, 6]);
    }
}

#[cfg(feature = "std")]
#[test]
fn take_chunks_vectored_respects_prefix_limits() {
    struct ManyChunks(std::collections::VecDeque<Bytes>);

    impl Buf for ManyChunks {
        fn remaining(&self) -> usize {
            self.0.iter().map(Buf::remaining).sum()
        }

        fn chunk(&self) -> &[u8] {
            self.0.front().map_or(&[], Buf::chunk)
        }

        fn advance(&mut self, mut count: usize) {
            assert!(count <= self.remaining());
            while count != 0 {
                let front = self.0.front_mut().unwrap();
                let advance = count.min(front.remaining());
                front.advance(advance);
                count -= advance;
                if !front.has_remaining() {
                    self.0.pop_front();
                }
            }
        }

        fn chunks_vectored<'a>(&'a self, dst: &mut [std::io::IoSlice<'a>]) -> usize {
            let count = dst.len().min(self.0.len());
            for (slot, chunk) in dst[..count].iter_mut().zip(self.0.iter()) {
                *slot = std::io::IoSlice::new(chunk.chunk());
            }
            count
        }
    }

    fn many_chunks() -> impl Buf {
        ManyChunks((0_u8..20).map(|byte| Bytes::from(vec![byte])).collect())
    }

    fn returned_bytes(buf: impl Buf, limit: usize, capacity: usize) -> (usize, Vec<u8>) {
        let take = buf.take(limit);
        let mut dst = vec![std::io::IoSlice::new(&[]); capacity];
        let count = take.chunks_vectored(&mut dst);
        let bytes = dst[..count]
            .iter()
            .flat_map(|slice| slice.iter().copied())
            .collect();
        (count, bytes)
    }

    for limit in 0..=20 {
        for capacity in 0..=20 {
            let expected_count = limit.min(capacity).min(16);
            assert_eq!(
                returned_bytes(many_chunks(), limit, capacity),
                (expected_count, (0..expected_count as u8).collect())
            );
        }
    }

    let chunks = Bytes::from_static(b"abc").chain(Bytes::from_static(b"defg"));
    assert_eq!(returned_bytes(chunks, 5, 2), (2, b"abcde".to_vec()));

    let chunks = Bytes::from_static(b"abc").chain(Bytes::from_static(b"defg"));
    assert_eq!(returned_bytes(chunks, 3, 2), (1, b"abc".to_vec()));
}
