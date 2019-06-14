use crate::api::*;
use crate::frame::Frame;
use crate::levels::*;
use crate::util::Pixel;
use crate::getbits::*;

use std::io;

impl<T: Pixel> Context<T> {
    pub fn parse_obus(&mut self, offset: usize, global: bool) -> io::Result<usize> {
        let pkt = self.packet.as_ref().unwrap();
        let mut gb = GetBits::new(&pkt.data[pkt.offset..]);

        // obu header
        gb.get_bits(1); // obu_forbidden_bit
        let obu_type = gb.get_bits(4);
        let has_extension = gb.get_bits(1) != 0;
        let has_length_field = gb.get_bits(1) != 0;
        gb.get_bits(1); // reserved

        let (mut temporal_id, mut spatial_id) = (0, 0);
        if has_extension {
            temporal_id = gb.get_bits(3);
            spatial_id = gb.get_bits(2);
            gb.get_bits(3); // reserved
        }

        // obu length field
        let len =  if has_length_field {
            gb.get_uleb128()
        }else {
            let l = pkt.data.len() as isize - pkt.offset as isize - 1 - has_extension as isize;
            if l <= 0 {
                0
            } else {
                l as u32
            }
        };
        gb.check_error()?;

        if obu_type != ObuType::OBU_SEQ_HDR as u32
            && obu_type != ObuType::OBU_TD as u32
            && has_extension
            && self.operating_point_idc != 0
        {}

        self.frame = Some(Frame::new(352, 288, ChromaSampling::Cs420));
        Ok(pkt.data.len())
    }
}
