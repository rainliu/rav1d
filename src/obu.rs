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
    operating_point: usize,
) -> io::Result<u32> {
    let mut operating_point_idc: u32 = 0;
    let init_bit_pos = gb.get_bits_pos();

    hdr.profile = gb.get_bits(3) as u8;
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

    if hdr.reduced_still_picture_header {
        hdr.timing_info_present = false;
        hdr.decoder_model_info_present = false;
        hdr.display_model_info_present = false;
        hdr.num_operating_points = 1;
        hdr.operating_points[0].idc = 0;
        hdr.operating_points[0].major_level = gb.get_bits(3);
        hdr.operating_points[0].minor_level = gb.get_bits(2);
        hdr.operating_points[0].tier = 0;
        hdr.operating_points[0].decoder_model_param_present = false;
        hdr.operating_points[0].display_model_param_present = false;
    } else {
        hdr.timing_info_present = gb.get_bits(1) != 0;
        if hdr.timing_info_present {
            hdr.num_units_in_tick = gb.get_bits(32);
            hdr.time_scale = gb.get_bits(32);
            hdr.equal_picture_interval = gb.get_bits(1) != 0;
            if hdr.equal_picture_interval {
                let num_ticks_per_picture = gb.get_vlc();
                check_error(
                    num_ticks_per_picture == 0xFFFFFFFF,
                    "num_ticks_per_picture == 0xFFFFFFFF",
                )?;
                hdr.num_ticks_per_picture = num_ticks_per_picture + 1;
            }

            hdr.decoder_model_info_present = gb.get_bits(1) != 0;
            if hdr.decoder_model_info_present {
                hdr.encoder_decoder_buffer_delay_length = gb.get_bits(5) + 1;
                hdr.num_units_in_decoding_tick = gb.get_bits(32);
                hdr.buffer_removal_delay_length = gb.get_bits(5) + 1;
                hdr.frame_presentation_delay_length = gb.get_bits(5) + 1;
            }
        } else {
            hdr.decoder_model_info_present = false;
        }

        rav1d_log!(
            "SEQHDR: post-timinginfo: off={}\n",
            gb.get_bits_pos() - init_bit_pos
        );

        hdr.display_model_info_present = gb.get_bits(1) != 0;
        hdr.num_operating_points = gb.get_bits(5) as usize + 1;
        for i in 0..hdr.num_operating_points {
            let op = &mut hdr.operating_points[i];
            let opi = &mut hdr.operating_parameter_info[i];
            op.idc = gb.get_bits(12);
            op.major_level = 2 + gb.get_bits(3);
            op.minor_level = gb.get_bits(2);
            op.tier = if op.major_level > 3 {
                gb.get_bits(1)
            } else {
                0
            };
            op.decoder_model_param_present = hdr.decoder_model_info_present && gb.get_bits(1) != 0;
            if op.decoder_model_param_present {
                opi.decoder_buffer_delay = gb.get_bits(hdr.encoder_decoder_buffer_delay_length);
                opi.encoder_buffer_delay = gb.get_bits(hdr.encoder_decoder_buffer_delay_length);
                opi.low_delay_mode = gb.get_bits(1) != 0;
            }
            op.display_model_param_present = hdr.display_model_info_present && gb.get_bits(1) != 0;
            if op.display_model_param_present {
                op.initial_display_delay = gb.get_bits(4) + 1;
            }
        }
        if operating_point < hdr.num_operating_points {
            operating_point_idc = hdr.operating_points[operating_point].idc;
        } else {
            operating_point_idc = hdr.operating_points[0].idc;
        }
        rav1d_log!(
            "SEQHDR: post-operating-points: off={}\n",
            gb.get_bits_pos() - init_bit_pos
        );
    }

    hdr.width_n_bits = gb.get_bits(4) + 1;
    hdr.height_n_bits = gb.get_bits(4) + 1;
    hdr.max_width = gb.get_bits(hdr.width_n_bits) + 1;
    hdr.max_height = gb.get_bits(hdr.height_n_bits) + 1;
    rav1d_log!(
        "SEQHDR: post-size: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );
    hdr.frame_id_numbers_present = if hdr.reduced_still_picture_header {
        false
    } else {
        gb.get_bits(1) != 0
    };
    if hdr.frame_id_numbers_present {
        hdr.delta_frame_id_n_bits = gb.get_bits(4) + 2;
        hdr.frame_id_n_bits = gb.get_bits(3) + hdr.delta_frame_id_n_bits + 1;
    }
    rav1d_log!(
        "SEQHDR: post-frame-id-numbers-present: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    hdr.sb128 = gb.get_bits(1) != 0;
    hdr.filter_intra = gb.get_bits(1) != 0;
    hdr.intra_edge_filter = gb.get_bits(1) != 0;
    if hdr.reduced_still_picture_header {
        hdr.inter_intra = false;
        hdr.masked_compound = false;
        hdr.warped_motion = false;
        hdr.dual_filter = false;
        hdr.order_hint = false;
        hdr.jnt_comp = false;
        hdr.ref_frame_mvs = false;
        hdr.order_hint_n_bits = 0;
        hdr.screen_content_tools = AdaptiveBoolean::ADAPTIVE;
        hdr.force_integer_mv = AdaptiveBoolean::ADAPTIVE;
    } else {
        hdr.inter_intra = gb.get_bits(1) != 0;
        hdr.masked_compound = gb.get_bits(1) != 0;
        hdr.warped_motion = gb.get_bits(1) != 0;
        hdr.dual_filter = gb.get_bits(1) != 0;
        hdr.order_hint = gb.get_bits(1) != 0;
        if hdr.order_hint {
            hdr.jnt_comp = gb.get_bits(1) != 0;
            hdr.ref_frame_mvs = gb.get_bits(1) != 0;
        } else {
            hdr.jnt_comp = false;
            hdr.ref_frame_mvs = false;
            hdr.order_hint_n_bits = 0;
        }
        hdr.screen_content_tools = if gb.get_bits(1) != 0 {
            AdaptiveBoolean::ADAPTIVE
        } else if gb.get_bits(1) != 0 {
            AdaptiveBoolean::ON
        } else {
            AdaptiveBoolean::OFF
        };
        rav1d_log!(
            "SEQHDR: post-screentools: off={}\n",
            gb.get_bits_pos() - init_bit_pos
        );
        hdr.force_integer_mv = if hdr.screen_content_tools != AdaptiveBoolean::OFF {
            if gb.get_bits(1) != 0 {
                AdaptiveBoolean::ADAPTIVE
            } else if gb.get_bits(1) != 0 {
                AdaptiveBoolean::ON
            } else {
                AdaptiveBoolean::OFF
            }
        } else {
            AdaptiveBoolean::ADAPTIVE
        };
        if hdr.order_hint {
            hdr.order_hint_n_bits = gb.get_bits(3) + 1;
        }
    }
    hdr.super_res = gb.get_bits(1) != 0;
    hdr.cdef = gb.get_bits(1) != 0;
    hdr.restoration = gb.get_bits(1) != 0;
    rav1d_log!(
        "SEQHDR: post-featurebits: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    hdr.hbd = gb.get_bits(1);
    if hdr.profile == 2 && hdr.hbd != 0 {
        hdr.hbd += gb.get_bits(1);
    }
    hdr.monochrome = if hdr.profile != 1 {
        gb.get_bits(1) != 0
    } else {
        false
    };
    hdr.color_description_present = gb.get_bits(1) != 0;
    if hdr.color_description_present {
        hdr.pri = match FromPrimitive::from_u32(gb.get_bits(8)) {
            Some(pri) => pri,
            None => ColorPrimaries::COLOR_PRI_UNKNOWN,
        };
        hdr.trc = match FromPrimitive::from_u32(gb.get_bits(8)) {
            Some(trc) => trc,
            None => TransferCharacteristics::TRC_UNKNOWN,
        };
        hdr.mtrx = match FromPrimitive::from_u32(gb.get_bits(8)) {
            Some(mtrx) => mtrx,
            None => MatrixCoefficients::MC_UNKNOWN,
        };
    } else {
        hdr.pri = ColorPrimaries::COLOR_PRI_UNKNOWN;
        hdr.trc = TransferCharacteristics::TRC_UNKNOWN;
        hdr.mtrx = MatrixCoefficients::MC_UNKNOWN;
    }
    if hdr.monochrome {
        hdr.color_range = gb.get_bits(1) != 0;
        hdr.layout = PixelLayout::PIXEL_LAYOUT_I400;
        hdr.ss_hor = 1;
        hdr.ss_ver = 1;
        hdr.chr = ChromaSamplePosition::CHR_UNKNOWN;
        hdr.separate_uv_delta_q = false;
    } else if hdr.pri == ColorPrimaries::COLOR_PRI_BT709
        && hdr.trc == TransferCharacteristics::TRC_SRGB
        && hdr.mtrx == MatrixCoefficients::MC_IDENTITY
    {
        hdr.layout = PixelLayout::PIXEL_LAYOUT_I444;
        hdr.ss_hor = 0;
        hdr.ss_ver = 0;
        hdr.color_range = true;
        check_error(
            hdr.profile != 1 && !(hdr.profile == 2 && hdr.hbd == 2),
            "hdr.profile != 1 && !(hdr.profile == 2 && hdr.hbd == 2)",
        )?;
    } else {
        hdr.color_range = gb.get_bits(1) != 0;
        match hdr.profile {
            0 => {
                hdr.layout = PixelLayout::PIXEL_LAYOUT_I420;
                hdr.ss_hor = 1;
                hdr.ss_ver = 1;
            }
            1 => {
                hdr.layout = PixelLayout::PIXEL_LAYOUT_I444;
                hdr.ss_hor = 0;
                hdr.ss_ver = 0;
            }
            2 => {
                if hdr.hbd == 2 {
                    hdr.ss_hor = gb.get_bits(1);
                    hdr.ss_ver = (hdr.ss_hor != 0 && gb.get_bits(1) != 0) as u32;
                } else {
                    hdr.ss_hor = 1;
                    hdr.ss_ver = 0;
                }
                hdr.layout = if hdr.ss_hor != 0 {
                    if hdr.ss_ver != 0 {
                        PixelLayout::PIXEL_LAYOUT_I420
                    } else {
                        PixelLayout::PIXEL_LAYOUT_I422
                    }
                } else {
                    PixelLayout::PIXEL_LAYOUT_I444
                };
            }
            _ => {}
        }
        hdr.chr = if hdr.ss_hor == 1 && hdr.ss_ver == 1 {
            FromPrimitive::from_u32(gb.get_bits(2)).unwrap()
        } else {
            ChromaSamplePosition::CHR_UNKNOWN
        };
    }
    hdr.separate_uv_delta_q = !hdr.monochrome && gb.get_bits(1) != 0;
    rav1d_log!(
        "SEQHDR: post-colorinfo: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    hdr.film_grain_present = gb.get_bits(1) != 0;
    rav1d_log!(
        "SEQHDR: post-filmgrain: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    gb.get_bits(1); // dummy bit

    // We needn't bother flushing the OBU here: we'll check we didn't
    // overrun in the caller and will then discard gb, so there's no
    // point in setting its position properly.

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
                    self.operating_point,
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
