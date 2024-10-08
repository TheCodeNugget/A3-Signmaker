use std::io;
use std::io::{Read, Seek, Write, Stdout, Cursor};
use std::fs::File;

pub enum Input {
    File(File),
    Cursor(Cursor<Box<[u8]>>),
}

pub enum Output {
    File(File),
    Standard(Stdout),
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut f)   => f.read(buf),
            Input::Cursor(ref mut c) => c.read(buf),
        }
    }
}

impl Seek for Input {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match *self {
            Input::File(ref mut f)   => f.seek(pos),
            Input::Cursor(ref mut c) => c.seek(pos),
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Output::File(ref mut f)     => f.write(buf),
            Output::Standard(ref mut s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Output::File(ref mut f)     => f.flush(),
            Output::Standard(ref mut s) => s.flush(),
        }
    }
}

pub trait ReadExt: Read {
    fn read_cstring(&mut self) -> io::Result<String>;
    fn read_compressed_int(&mut self) -> io::Result<u32>;
}

impl<T: Read> ReadExt for T {
    fn read_cstring(&mut self) -> io::Result<String> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte in self.bytes() {
            let b = byte?;
            if b == 0 {
                break;
            } else {
                bytes.push(b);
            }

        }

        Ok(String::from_utf8(bytes).unwrap())
    }

    fn read_compressed_int(&mut self) -> io::Result<u32> {
        let mut i = 0;
        let mut result: u32 = 0;

        for byte in self.bytes() {
            let b: u32 = byte?.into();
            result = result | ((b & 0x7f) << (i * 7));

            if b < 0x80 {
                break;
            }

            i += 1;
        }

        Ok(result)
    }
}

pub trait WriteExt: Write {
    fn write_cstring<S: AsRef<[u8]>>(&mut self, s: S) -> io::Result<()>;
    fn write_compressed_int(&mut self, x: u32) -> io::Result<usize>;
}

impl<T: Write> WriteExt for T {
    fn write_cstring<S: AsRef<[u8]>>(&mut self, s: S) -> io::Result<()> {
        self.write_all(s.as_ref())?;
        self.write_all(b"\0")?;
        Ok(())
    }

    fn write_compressed_int(&mut self, x: u32) -> io::Result<usize> {
        let mut temp = x;
        let mut len = 0;

        while temp > 0x7f {
            self.write(&[(0x80 | temp & 0x7f) as u8])?;
            len += 1;
            temp &= !0x7f;
            temp >>= 7;
        }

        self.write(&[temp as u8])?;
        Ok(len + 1)
    }
}

pub fn compressed_int_len(x: u32) -> usize {
    let mut temp = x;
    let mut len = 0;

    while temp > 0x7f {
        len += 1;
        temp &= !0x7f;
        temp >>= 7;
    }

    len + 1
}

