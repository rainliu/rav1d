use crate::api::*;
use crate::util::Pixel;
use crate::frame::Frame;

use bitstream_io::{BitReader, LittleEndian};
use std::io;
use std::io::BufReader;

impl<T: Pixel> Context<T> {
    pub fn parse_obus(&mut self, offset: usize, global: bool) -> isize {
        let pkt = self.packet.as_ref().unwrap();
        let r = BufReader::new(&pkt.data[pkt.offset..]);
        let mut br = BitReader::endian(r, LittleEndian);
        self.frame = Some(Frame::new(352, 288, ChromaSampling::Cs420));
        pkt.data.len() as isize
    }
}