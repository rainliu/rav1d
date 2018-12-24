use std::io;
use std::io::{Error, ErrorKind};

struct BitReader<'a> {
    bit_buffer: &'a mut [u8],
    bit_offset: usize,
}

impl<'a> BitReader<'a> {
    fn bytes_read(&self) -> usize {
        return (self.bit_offset + 7) >> 3;
    }

    fn read_bit(&mut self) -> io::Result<i32> {
        let off = self.bit_offset;
        let p = off >> 3;
        let q = 7 - (off & 0x7) as usize;
        if p < self.bit_buffer.len() {
            let bit = ((self.bit_buffer[p] >> q) & 1) as i32;
            self.bit_offset = off + 1;
            Ok(bit)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "read bit beyond buffer length",
            ))
        }
    }

    fn read_literal(&mut self, bits: i32) -> io::Result<i32> {
        let mut value = 0;
        for bit in (0..bits).rev() {
            match self.read_bit() {
                Ok(b) => value |= b << bit,
                Err(err) => return Err(err),
            };
        }
        Ok(value)
    }

    fn read_unsigned_literal(&mut self, bits: i32) -> io::Result<u32> {
        let mut value = 0u32;
        for bit in (0..bits).rev() {
            match self.read_bit() {
                Ok(b) => value |= (b as u32) << bit,
                Err(err) => return Err(err),
            };
        }
        Ok(value)
    }

    fn read_inv_signed_literal(&mut self, bits: i32) -> io::Result<i32> {
        let nbits = (std::mem::size_of::<u32>() as i32) * 8 - bits - 1;
        let value = match self.read_literal(bits + 1) {
            Ok(l) => (l as u32) << nbits,
            Err(err) => return Err(err),
        };
        return Ok((value as i32) >> nbits);
    }
}
