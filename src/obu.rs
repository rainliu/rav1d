use crate::api::*;
use crate::frame::Frame;
use crate::getbits::*;
use crate::headers::*;
use crate::levels::*;
use crate::util::Pixel;

use std::rc::Rc;
use std::{cmp, io};

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

#[inline(always)]
fn tile_log2(sz: i32, tgt: i32) -> i32 {
    let mut k = 0;
    while (sz << k) < tgt {
        k += 1;
    }
    k
}

fn parse_seq_hdr(
    gb: &mut GetBits,
    hdr: &mut SequenceHeader,
    operating_point: usize,
) -> io::Result<u32> {
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

    let mut operating_point_idc: u32 = 0;

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

fn parse_frame_size(
    gb: &mut GetBits,
    seqhdr: &SequenceHeader,
    hdr: &mut FrameHeader,
    use_ref: bool,
) -> io::Result<()> {
    if use_ref {
        for i in 0..7 {
            if gb.get_bits(1) != 0 {
                unimplemented!();
                /*Dav1dThreadPicture *const r =
                    &c->refs[c->frame_hdr.refidx[i]].p;
                if (!ref->p.data[0]) return -1;
                // FIXME render_* may be wrong
                hdr.render_width = hdr.width[1] = r->p.p.w;
                hdr.render_height = hdr.height = r->p.p.h;
                hdr.super_res.enabled = seqhdr.super_res && gb.get_bits(1) != 0;
                if hdr.super_res.enabled {
                    hdr.super_res.width_scale_denominator =
                        9 + gb.get_bits( 3);
                    let d = hdr.super_res.width_scale_denominator;
                    hdr.width[0] = cmp::max((hdr.width[1] * 8 + (d >> 1)) / d,
                                     cmp::min(16, hdr.width[1]));
                } else {
                    hdr.super_res.width_scale_denominator = 8;
                    hdr.width[0] = hdr.width[1];
                }*/
                return Ok(());
            }
        }
    }

    if hdr.frame_size_override {
        hdr.width[1] = gb.get_bits(seqhdr.width_n_bits) + 1;
        hdr.height = gb.get_bits(seqhdr.height_n_bits) + 1;
    } else {
        hdr.width[1] = seqhdr.max_width;
        hdr.height = seqhdr.max_height;
    }
    hdr.super_res.enabled = seqhdr.super_res && gb.get_bits(1) != 0;
    if hdr.super_res.enabled {
        hdr.super_res.width_scale_denominator = 9 + gb.get_bits(3);
        let d = hdr.super_res.width_scale_denominator;
        hdr.width[0] = cmp::max(
            (hdr.width[1] * 8 + (d >> 1)) / d,
            cmp::min(16, hdr.width[1]),
        );
    } else {
        hdr.super_res.width_scale_denominator = 8;
        hdr.width[0] = hdr.width[1];
    }
    hdr.have_render_size = gb.get_bits(1) != 0;
    if hdr.have_render_size {
        hdr.render_width = gb.get_bits(16) + 1;
        hdr.render_height = gb.get_bits(16) + 1;
    } else {
        hdr.render_width = hdr.width[1];
        hdr.render_height = hdr.height;
    }
    Ok(())
}

fn parse_frame_hdr(
    gb: &mut GetBits,
    seqhdr: &SequenceHeader,
    hdr: &mut FrameHeader,
) -> io::Result<()> {
    let init_bit_pos = gb.get_bits_pos();

    hdr.show_existing_frame = !seqhdr.reduced_still_picture_header && gb.get_bits(1) != 0;
    rav1d_log!(
        "HDR: post-show_existing_frame: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    if hdr.show_existing_frame {
        hdr.existing_frame_idx = gb.get_bits(3);
        if seqhdr.decoder_model_info_present && !seqhdr.equal_picture_interval {
            hdr.frame_presentation_delay = gb.get_bits(seqhdr.frame_presentation_delay_length);
        }
        if seqhdr.frame_id_numbers_present {
            hdr.frame_id = gb.get_bits(seqhdr.frame_id_n_bits);
        }
        return Ok(());
    }

    hdr.frame_type = if seqhdr.reduced_still_picture_header {
        FrameType::FRAME_TYPE_KEY
    } else {
        FromPrimitive::from_u32(gb.get_bits(2)).unwrap()
    };
    hdr.show_frame = seqhdr.reduced_still_picture_header || gb.get_bits(1) != 0;
    if hdr.show_frame {
        if seqhdr.decoder_model_info_present && !seqhdr.equal_picture_interval {
            hdr.frame_presentation_delay = gb.get_bits(seqhdr.frame_presentation_delay_length);
        }
    } else {
        hdr.showable_frame = gb.get_bits(1) != 0;
    }
    hdr.error_resilient_mode = (hdr.frame_type == FrameType::FRAME_TYPE_KEY && hdr.show_frame)
        || hdr.frame_type == FrameType::FRAME_TYPE_SWITCH
        || seqhdr.reduced_still_picture_header
        || gb.get_bits(1) != 0;

    rav1d_log!(
        "HDR: post-frametype_bits: off={}\n",
        gb.get_bits_pos() - init_bit_pos,
    );

    hdr.disable_cdf_update = gb.get_bits(1) != 0;
    hdr.allow_screen_content_tools = if seqhdr.screen_content_tools == AdaptiveBoolean::ADAPTIVE {
        if gb.get_bits(1) == 0 {
            AdaptiveBoolean::OFF
        } else {
            AdaptiveBoolean::ON
        }
    } else {
        seqhdr.screen_content_tools
    };
    if hdr.allow_screen_content_tools != AdaptiveBoolean::OFF {
        hdr.force_integer_mv = if seqhdr.force_integer_mv == AdaptiveBoolean::ADAPTIVE {
            if gb.get_bits(1) == 0 {
                AdaptiveBoolean::OFF
            } else {
                AdaptiveBoolean::ON
            }
        } else {
            seqhdr.force_integer_mv
        };
    } else {
        hdr.force_integer_mv = AdaptiveBoolean::OFF;
    }
    if (hdr.frame_type as u8 & 1) == 0 {
        hdr.force_integer_mv = AdaptiveBoolean::ON;
    }

    if seqhdr.frame_id_numbers_present {
        hdr.frame_id = gb.get_bits(seqhdr.frame_id_n_bits);
    }

    hdr.frame_size_override = if seqhdr.reduced_still_picture_header {
        false
    } else if hdr.frame_type == FrameType::FRAME_TYPE_SWITCH {
        true
    } else {
        gb.get_bits(1) != 0
    };

    rav1d_log!(
        "HDR: post-frame_size_override_flag: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    hdr.frame_offset = if seqhdr.order_hint {
        gb.get_bits(seqhdr.order_hint_n_bits)
    } else {
        0
    };
    hdr.primary_ref_frame = if !hdr.error_resilient_mode && (hdr.frame_type as u32 & 1) != 0 {
        gb.get_bits(3)
    } else {
        PRIMARY_REF_NONE as u32
    };

    if seqhdr.decoder_model_info_present {
        hdr.buffer_removal_time_present = gb.get_bits(1) != 0;
        if hdr.buffer_removal_time_present {
            for i in 0..seqhdr.num_operating_points {
                let seqop = &seqhdr.operating_points[i];
                let op = &mut hdr.operating_points[i];
                if seqop.decoder_model_param_present {
                    let in_temporal_layer = (seqop.idc >> hdr.temporal_id) & 1;
                    let in_spatial_layer = (seqop.idc >> (hdr.spatial_id + 8)) & 1;
                    if seqop.idc == 0 || (in_temporal_layer != 0 && in_spatial_layer != 0) {
                        op.buffer_removal_time = gb.get_bits(seqhdr.buffer_removal_delay_length);
                    }
                }
            }
        }
    }

    if hdr.frame_type == FrameType::FRAME_TYPE_KEY || hdr.frame_type == FrameType::FRAME_TYPE_INTRA
    {
        hdr.refresh_frame_flags = if hdr.frame_type == FrameType::FRAME_TYPE_KEY && hdr.show_frame {
            0xff
        } else {
            gb.get_bits(8)
        };
        if hdr.refresh_frame_flags != 0xff && hdr.error_resilient_mode && seqhdr.order_hint {
            for _ in 0..8 {
                gb.get_bits(seqhdr.order_hint_n_bits);
            }
        }

        parse_frame_size(gb, seqhdr, hdr, false)?;
        hdr.allow_intrabc = hdr.allow_screen_content_tools != AdaptiveBoolean::OFF
            && !hdr.super_res.enabled
            && gb.get_bits(1) != 0;
        hdr.use_ref_frame_mvs = 0;
    } else {
        unimplemented!();
    }
    rav1d_log!(
        "HDR: post-frametype-specific-bits: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    hdr.refresh_context =
        !seqhdr.reduced_still_picture_header && !hdr.disable_cdf_update && !gb.get_bits(1) != 0;
    rav1d_log!(
        "HDR: post-refresh_context: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    // tile data
    hdr.tiling.uniform = gb.get_bits(1) != 0;
    let sbsz_min1 = (64 << seqhdr.sb128 as i32) - 1;
    let sbsz_log2 = 6 + seqhdr.sb128 as i32;
    let sbw = (hdr.width[0] as i32 + sbsz_min1) >> sbsz_log2;
    let sbh = (hdr.height as i32 + sbsz_min1) >> sbsz_log2;
    let max_tile_width_sb = 4096 >> sbsz_log2;
    let max_tile_area_sb = 4096 * 2304 >> (2 * sbsz_log2);
    hdr.tiling.min_log2_cols = tile_log2(max_tile_width_sb, sbw);
    hdr.tiling.max_log2_cols = tile_log2(1, cmp::min(sbw, MAX_TILE_COLS as i32));
    hdr.tiling.max_log2_rows = tile_log2(1, cmp::min(sbh, MAX_TILE_ROWS as i32));
    let min_log2_tiles = cmp::max(
        tile_log2(max_tile_area_sb, sbw * sbh),
        hdr.tiling.min_log2_cols,
    );
    if hdr.tiling.uniform {
        hdr.tiling.log2_cols = hdr.tiling.min_log2_cols;
        while hdr.tiling.log2_cols < hdr.tiling.max_log2_cols && gb.get_bits(1) != 0 {
            hdr.tiling.log2_cols += 1;
        }
        let tile_w = 1 + ((sbw - 1) >> hdr.tiling.log2_cols);
        hdr.tiling.cols = 0;
        let mut sbx = 0;
        while sbx < sbw {
            hdr.tiling.col_start_sb[hdr.tiling.cols as usize] = sbx as u16;
            sbx += tile_w;
            hdr.tiling.cols += 1;
        }
        hdr.tiling.min_log2_rows = cmp::max(min_log2_tiles - hdr.tiling.log2_cols, 0);

        hdr.tiling.log2_rows = hdr.tiling.min_log2_rows;
        while hdr.tiling.log2_rows < hdr.tiling.max_log2_rows && gb.get_bits(1) != 0 {
            hdr.tiling.log2_rows += 1;
        }
        let tile_h = 1 + ((sbh - 1) >> hdr.tiling.log2_rows);
        hdr.tiling.rows = 0;
        let mut sby = 0;
        while sby < sbh {
            hdr.tiling.row_start_sb[hdr.tiling.rows as usize] = sby as u16;
            sby += tile_h;
            hdr.tiling.rows += 1;
        }
    } else {
        hdr.tiling.cols = 0;
        let mut widest_tile = 0;
        let mut max_tile_area_sb = sbw * sbh;
        let mut sbx = 0;
        while sbx < sbw && hdr.tiling.cols < MAX_TILE_COLS as i32 {
            let tile_width_sb = cmp::min(sbw - sbx, max_tile_width_sb);
            let tile_w = if tile_width_sb > 1 {
                1 + gb.get_uniform(tile_width_sb as u32)
            } else {
                1
            } as i32;
            hdr.tiling.col_start_sb[hdr.tiling.cols as usize] = sbx as u16;
            sbx += tile_w;
            widest_tile = cmp::max(widest_tile, tile_w);
            hdr.tiling.cols += 1;
        }
        hdr.tiling.log2_cols = tile_log2(1, hdr.tiling.cols);
        if min_log2_tiles != 0 {
            max_tile_area_sb >>= min_log2_tiles + 1;
        }
        let max_tile_height_sb = cmp::max(max_tile_area_sb / widest_tile, 1);

        hdr.tiling.rows = 0;
        let mut sby = 0;
        while sby < sbh && hdr.tiling.rows < MAX_TILE_ROWS as i32 {
            let tile_height_sb = cmp::min(sbh - sby, max_tile_height_sb);
            let tile_h = if tile_height_sb > 1 {
                1 + gb.get_uniform(tile_height_sb as u32)
            } else {
                1
            } as i32;
            hdr.tiling.row_start_sb[hdr.tiling.rows as usize] = sby as u16;
            sby += tile_h;
            hdr.tiling.rows += 1;
        }
        hdr.tiling.log2_rows = tile_log2(1, hdr.tiling.rows);
    }
    hdr.tiling.col_start_sb[hdr.tiling.cols as usize] = sbw as u16;
    hdr.tiling.row_start_sb[hdr.tiling.rows as usize] = sbh as u16;
    if hdr.tiling.log2_cols != 0 || hdr.tiling.log2_rows != 0 {
        hdr.tiling.update =
            gb.get_bits((hdr.tiling.log2_cols + hdr.tiling.log2_rows) as u32) as i32;
        check_error(
            hdr.tiling.update >= hdr.tiling.cols * hdr.tiling.rows,
            "hdr.tiling.update >= hdr.tiling.cols * hdr.tiling.rows",
        )?;
        hdr.tiling.n_bytes = gb.get_bits(2) + 1;
    } else {
        hdr.tiling.n_bytes = 0;
        hdr.tiling.update = 0;
    }

    rav1d_log!(
        "HDR: post-tiling: off={}\n",
        gb.get_bits_pos() - init_bit_pos
    );

    Ok(())
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
                self.operating_point_idc =
                    parse_seq_hdr(&mut gb, Rc::make_mut(&mut seq_hdr), self.operating_point)?;
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
            Some(t @ ObuType::OBU_REDUNDANT_FRAME_HDR)
            | Some(t @ ObuType::OBU_FRAME)
            | Some(t @ ObuType::OBU_FRAME_HDR) => matched_block!({
                if t == ObuType::OBU_REDUNDANT_FRAME_HDR && self.frame_hdr.is_some() {
                    break;
                }
                if global {
                    break;
                }
                check_error(self.seq_hdr.is_none(), "seq_hdr.is_none()")?;
                if self.frame_hdr.is_none() {
                    self.frame_hdr = Some(Rc::new(FrameHeader::new()));
                }
                if let Some(frame_hdr) = self.frame_hdr.as_mut() {
                    parse_frame_hdr(
                        &mut gb,
                        self.seq_hdr.as_ref().unwrap(),
                        Rc::make_mut(frame_hdr),
                    )?;
                }
            }),
            _ => {
                // print a warning but don't fail for unknown types
                rav1d_log!("Unknown OBU type {} of size {}\n", obu_type, len);
            }
        }

        //self.frame = Some(Frame::new(352, 288, ChromaSampling::Cs420));
        Ok(len + init_byte_pos)
    }
}
