use crate::api::*;
use crate::frame::Frame;
use crate::levels::*;
use crate::util::Pixel;

use bitstream_io::{BitReader, LittleEndian};
use std::io;
use std::io::Cursor;

impl<T: Pixel> Context<T> {
    pub fn parse_obus(&mut self, offset: usize, global: bool) -> io::Result<usize> {
        let pkt = self.packet.as_ref().unwrap();
        let r = Cursor::new(&pkt.data[pkt.offset..]);
        let mut br = BitReader::endian(r, LittleEndian);

        // obu header
        br.read_bit()?; // obu_forbidden_bit
        let obu_type: u8 = br.read(4)?;
        let has_extension = br.read_bit()?;
        let has_length_field = br.read_bit()?;
        br.read_bit()?; // reserved

        let (mut temporal_id, mut spatial_id) = (0, 0);
        if has_extension {
            temporal_id = br.read(3)?;
            spatial_id = br.read(2)?;
            let _: u8 = br.read(3)?; // reserved
        }

        // obu length field
        /*let mut len = 0;
        if has_length_field {
            len = br.(&gb);
        }else {
            len = (int) in -> sz - 1 - has_extension;
        }
        if (gb.error) goto error;*/

        if obu_type != ObuType::OBU_SEQ_HDR as u8
            && obu_type != ObuType::OBU_TD as u8
            && has_extension
            && self.operating_point_idc != 0
        {}

        self.frame = Some(Frame::new(352, 288, ChromaSampling::Cs420));
        Ok(pkt.data.len())
    }
}
