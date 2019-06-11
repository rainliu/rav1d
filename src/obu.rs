use crate::api::*;
use crate::util::Pixel;

use bitstream_io::{BitReader, LittleEndian};
use std::io;

impl<T: Pixel> Context<T> {
    pub fn parse_obus(r: &mut dyn io::Read, global: bool) -> isize {
        let mut br = BitReader::endian(r, LittleEndian);
        0
    }
}