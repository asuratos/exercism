use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    content: R,
    read_count: usize,
    bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            content: _wrapped,
            read_count: 0,
            bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.content
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_count += 1;

        let read_out = self.content.read(buf);
        self.bytes_read += read_out.as_ref().unwrap();
        read_out
    }
}

pub struct WriteStats<W> {
    content: W,
    write_count: usize,
    bytes_written: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats {
            content: _wrapped,
            write_count: 0,
            bytes_written: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.content
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_count += 1;

        let write_out = self.content.write(buf);
        self.bytes_written += write_out.as_ref().unwrap();
        write_out
    }

    fn flush(&mut self) -> Result<()> {
        self.content.flush()
    }
}
