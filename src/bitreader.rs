pub struct BitReader<'a> {
    buffer: &'a [u8],
    length: usize,
    offset: usize,
    state: u64,
    bits_left: u32,
    error: bool,
    eof: bool,
}

impl<'a> BitReader<'a> {
    pub fn new(buffer: &[u8]) -> BitReader {
        BitReader {
            buffer: buffer,
            length: buffer.len(),
            offset: 0,
            state: 0,
            bits_left: 0,
            error: false,
            eof: false,
        }
    }

    fn refill(&mut self, n: u32) {
        debug_assert!(self.bits_left <= 56);
        let mut state: u64 = 0;
        while n > self.bits_left {
            state <<= 8;
            self.bits_left += 8;
            if self.offset < self.length {
                state |= self.buffer[self.offset] as u64;
                self.offset += 1;
            }
            if self.offset >= self.length {
                self.error = self.eof;
                self.eof = true
            }
        }
        self.state |= state << (64 - self.bits_left);
    }

    pub fn get_position(&self) -> usize {
        self.offset * 8 - (self.bits_left as usize)
    }

    pub fn is_eof(&self) -> bool {
        self.eof
    }

    pub fn is_error(&self) -> bool {
        self.error
    }

    pub fn get_bits(&mut self, n: u32) -> u32 {
        debug_assert!(n <= 32);
        if n > self.bits_left {
            self.refill(n);
        }

        let state = self.state;
        self.bits_left -= n;
        self.state <<= n;

        (state >> (64 - n)) as u32
    }

    pub fn f(&mut self, n: u32, se: &str) -> u32 {
        self.get_bits(n)
    }

    pub fn uvlc(&mut self, se: &str) -> u32 {
        let mut leading_zeros = 0;
        loop {
            let done = self.get_bits(1);
            if done != 0 {
                break;
            }
            leading_zeros += 1;
        }
        if leading_zeros >= 32 {
            return std::u32::MAX;
        }
        let value = self.get_bits(leading_zeros);
        value + (1 << leading_zeros) - 1
    }

    pub fn le(&mut self, n: u32, se: &str) -> usize {
        if self.get_position() & 0x7 != 0 {
            self.error = true;
            return 0;
        }
        let mut t: usize = 0;
        for i in 0..n {
            let byte = self.get_bits(8);
            t += (byte as usize) << (i * 8);
        }
        t
    }

    pub fn leb128(&mut self, se: &str) -> u64 {
        let mut value: u64 = 0;
        for i in 0..8 {
            let leb128_byte = self.get_bits(8);
            value |= ((leb128_byte & 0x7f) as u64) << (i * 7);
            if (leb128_byte & 0x80) == 0 {
                break;
            }
        }
        value
    }

    pub fn su(&mut self, n: u32, se: &str) -> i32 {
        debug_assert!(n <= 31);
        let shift = (31 - n) as i32;
        let res = (self.get_bits(n + 1) as i32) << shift;
        res >> shift
    }

    pub fn ns(&mut self, n: u32, se: &str) -> u32 {
        // TODO
        0
    }

    pub fn trailing_bits(&mut self, n: usize) {
        let mut nb_bits = n;
        let trailing_one_bit = self.f(1, "trailing_one_bit");
        nb_bits -= 1;
        while nb_bits > 0 {
            let trailing_zero_bit = self.f(1, "trailing_zero_bit");
            nb_bits -= 1;
        }
    }

    pub fn byte_alignment(&mut self) {
        while self.get_position() & 0x7 != 0 {
            self.f(1, "zero_bit");
        }
    }
}
