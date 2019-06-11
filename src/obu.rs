use crate::api::*;
use crate::util::Pixel;

use bitstream_io::{BitReader, LittleEndian};
use std::io;
use std::io::BufReader;

impl<T: Pixel> Context<T> {
    pub fn parse_obus(&mut self, offset: usize, global: bool) -> isize {
        let pkt = self.packet.as_ref().unwrap();
        let r = BufReader::new(&pkt.data[pkt.offset..]);
        let mut br = BitReader::endian(r, LittleEndian);
        0
    }
}