use std::io;

pub struct GetBits<'a> {
    error: bool,
    eof: bool,
    state: u64,
    bits_left: u32,
    data: &'a [u8],
    ptr: usize,
    ptr_end: usize,
}

impl<'a> GetBits<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        GetBits {
            error: false,
            eof: false,
            state: 0,
            bits_left: 0,
            data,
            ptr: 0,
            ptr_end: data.len(),
        }
    }

    fn refill(&mut self, n: u32) {
        debug_assert!(self.bits_left <= 56);
        let mut state: u64 = 0;
        let mut more = true;
        while more {
            state <<= 8;
            self.bits_left += 8;
            if !self.eof {
                state |= self.data[self.ptr] as u64;
                self.ptr += 1;
            }
            if self.ptr >= self.ptr_end {
                self.error = self.eof;
                self.eof = true;
            }
            more = n > self.bits_left;
        }
        self.state |= state << (64 - self.bits_left as u64);
    }

    pub fn get_bits(&mut self, n: u32) -> u32 {
        debug_assert!(n <= 32 /* can go up to 57 if we change return type */);
        debug_assert!(n != 0 /* can't shift state by 64 */);

        if n > self.bits_left {
            self.refill(n);
        }

        let state = self.state;
        self.bits_left -= n;
        self.state <<= n as u64;

        return (state >> (64 - n as u64)) as u32;
    }

    pub fn get_sbits(&mut self, n: u32) -> i32 {
        let shift = 31 - n as i32;
        let res = (self.get_bits(n + 1) as i32) << shift;
        return res >> shift;
    }

    pub fn get_uleb128(&mut self) -> u32 {
        let (mut val, mut more, mut i) = (0, 1, 0);
        while more != 0 {
            more = self.get_bits(1);
            let bits = self.get_bits(7);
            if i <= 3 || (i == 4 && bits < 1 << 4) {
                val |= bits << (i * 7);
            } else if bits != 0 {
                self.error = true;
                return 0;
            }
            i += 1;
            if more != 0 && i == 8 {
                self.error = true;
                return 0;
            }
        }

        return val;
    }

    pub fn check_error(&self) -> io::Result<()> {
        if self.error {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Error parsing frame header",
            ))
        } else {
            Ok(())
        }
    }
}
