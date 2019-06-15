use crate::api::*;
use crate::frame::Frame;
use crate::getbits::*;
use crate::headers::*;
use crate::levels::*;
use crate::util::Pixel;

use std::io;
use std::rc::Rc;

use crate::headers::SequenceHeader;
use num_traits::FromPrimitive;

#[inline(always)]
fn check_error(condition: bool, msg: &str) -> io::Result<()> {
    if condition {
        Err(io::Error::new(io::ErrorKind::InvalidInput, msg))
    } else {
        Ok(())
    }
}

fn parse_seq_hdr(
    gb: &mut GetBits,
    hdr: &mut SequenceHeader,
    operating_point_idc: u32,
) -> io::Result<u32> {
    let init_bit_pos = gb.get_bits_pos();

    hdr.profile = gb.get_bits(3);
    check_error(hdr.profile > 2, "hdr.profile > 2")?;
    rav1d_log!(
        "SEQHDR: post-profile: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    hdr.still_picture = gb.get_bits(1) != 0;
    hdr.reduced_still_picture_header = gb.get_bits(1) != 0;
    check_error(
        hdr.reduced_still_picture_header && !hdr.still_picture,
        "hdr.reduced_still_picture_header && !hdr.still_picture",
    )?;
    rav1d_log!(
        "SEQHDR: post-stillpicture_flags: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    Ok(operating_point_idc)
}

impl<T: Pixel> Context<T> {
    pub fn parse_obus(&mut self, offset: usize, global: bool) -> io::Result<usize> {
        let data = &self.packet.as_ref().unwrap().data[offset..];
        let mut gb = GetBits::new(data);

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
        let len = if has_length_field {
            gb.get_uleb128() as usize
        } else {
            (data.len() as isize - 1 - has_extension as isize) as usize
        };
        gb.check_error()?;

        let init_bit_pos = gb.get_bits_pos() as usize;
        let init_byte_pos = init_bit_pos >> 3;
        let pkt_bytelen = init_byte_pos + len;

        // We must have read a whole number of bytes at this point (1 byte
        // for the header and whole bytes at a time when reading the
        // leb128 length field).
        debug_assert!((init_bit_pos & 7) == 0);

        // We also know that we haven't tried to read more than in->sz
        // bytes yet (otherwise the error flag would have been set by the
        // code in getbits.c)
        debug_assert!(data.len() >= init_byte_pos);

        // Make sure that there are enough bits left in the buffer for the
        // rest of the OBU.
        if len > data.len() - init_byte_pos {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Error parsing frame header",
            ));
        }

        if obu_type != ObuType::OBU_SEQ_HDR as u32
            && obu_type != ObuType::OBU_TD as u32
            && has_extension
            && self.operating_point_idc != 0
        {
            let in_temporal_layer = (self.operating_point_idc >> temporal_id) & 1;
            let in_spatial_layer = (self.operating_point_idc >> (spatial_id + 8)) & 1;
            if in_temporal_layer == 0 || in_spatial_layer == 0 {
                return Ok(len + init_byte_pos);
            }
        }

        match FromPrimitive::from_u32(obu_type) {
            Some(ObuType::OBU_SEQ_HDR) => {
                let mut seq_hdr = Rc::new(SequenceHeader::new());
                self.operating_point_idc = parse_seq_hdr(
                    &mut gb,
                    Rc::get_mut(&mut seq_hdr).unwrap(),
                    self.operating_point_idc,
                )?;
                gb.check_overrun(init_bit_pos as u32, len as u32)?;
                if self.seq_hdr.is_none() {
                    self.frame_hdr = None;
                } else if seq_hdr == *self.seq_hdr.as_ref().unwrap() {
                    self.frame_hdr = None;
                    //TODO:
                    /*
                    c->mastering_display = NULL;
                    c->content_light = NULL;
                    dav1d_ref_dec(&c->mastering_display_ref);
                    dav1d_ref_dec(&c->content_light_ref);
                    for (int i = 0; i < 8; i++) {
                        if (c->refs[i].p.p.data[0])
                            dav1d_thread_picture_unref(&c->refs[i].p);
                        dav1d_ref_dec(&c->refs[i].segmap);
                        dav1d_ref_dec(&c->refs[i].refmvs);
                        dav1d_cdf_thread_unref(&c->cdf[i]);
                    }
                    */
                }
                self.seq_hdr = Some(seq_hdr);
            }
            Some(ObuType::OBU_REDUNDANT_FRAME_HDR) => {}
            _ => {
                // print a warning but don't fail for unknown types
                // log(c, "Unknown OBU type %d of size %u\n", type, len);
            }
        }

        //self.frame = Some(Frame::new(352, 288, ChromaSampling::Cs420));
        Ok(len + init_byte_pos)
    }
}
