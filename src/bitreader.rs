use super::global::*;

pub struct BitReader<'a> {
    bit_buffer: &'a [u8],
    bit_offset: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(buffer: &[u8], offset: usize) -> BitReader {
        BitReader {
            bit_buffer: buffer,
            bit_offset: offset,
        }
    }

    pub fn bytes_read(&self) -> usize {
        (self.bit_offset + 7) >> 3
    }

    pub fn bits_read(&self) -> usize {
        self.bit_offset
    }

    pub fn read_bit(&mut self) -> Result<i32, aom_codec_err_t> {
        let off = self.bit_offset;
        let p = off >> 3;
        let q = 7 - (off & 0x7) as usize;
        if p < self.bit_buffer.len() {
            let bit = ((self.bit_buffer[p] >> q) & 1) as i32;
            self.bit_offset = off + 1;
            Ok(bit)
        } else {
            Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME)
        }
    }

    pub fn read_literal(&mut self, bits: i32) -> Result<i32, aom_codec_err_t> {
        let mut value = 0;
        for bit in (0..bits).rev() {
            match self.read_bit() {
                Ok(b) => value |= b << bit,
                Err(err) => return Err(err),
            };
        }
        Ok(value)
    }

    pub fn read_unsigned_literal(&mut self, bits: i32) -> Result<u32, aom_codec_err_t> {
        let mut value = 0u32;
        for bit in (0..bits).rev() {
            match self.read_bit() {
                Ok(b) => value |= (b as u32) << bit,
                Err(err) => return Err(err),
            };
        }
        Ok(value)
    }

    pub fn read_inv_signed_literal(&mut self, bits: i32) -> Result<i32, aom_codec_err_t> {
        let nbits = (std::mem::size_of::<u32>() as i32) * 8 - bits - 1;
        let value = match self.read_literal(bits + 1) {
            Ok(l) => (l as u32) << nbits,
            Err(err) => return Err(err),
        };
        Ok((value as i32) >> nbits)
    }
}
