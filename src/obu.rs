use super::bitreader::*;
use super::decoder::*;
use super::global::*;

#[derive(Debug, PartialEq)]
pub enum ObuType {
    ObuSequenceHeader = 1,
    ObuTemporalDelimiter = 2,
    ObuFrameHeader = 3,
    ObuTileGroup = 4,
    ObuMetadata = 5,
    ObuFrame = 6,
    ObuRedundantFrameHeader = 7,
    ObuTileList = 8,
    ObuPadding = 15,
}

impl ObuType {
    pub fn from(obu_type: u32) -> Option<Self> {
        match obu_type {
            1 => Some(ObuType::ObuSequenceHeader),
            2 => Some(ObuType::ObuTemporalDelimiter),
            3 => Some(ObuType::ObuFrameHeader),
            4 => Some(ObuType::ObuTileGroup),
            5 => Some(ObuType::ObuMetadata),
            6 => Some(ObuType::ObuFrame),
            7 => Some(ObuType::ObuRedundantFrameHeader),
            8 => Some(ObuType::ObuTileList),
            15 => Some(ObuType::ObuPadding),
            _ => None,
        }
    }
}

pub enum ObuMetadataTye {
    ObuMetadataTypeAomReserved0 = 0,
    ObuMetadataTypeHdrCll = 1,
    ObuMetadataTypeHdrMdcv = 2,
    ObuMetadataTypeScalability = 3,
    ObuMetadataTypeItutT35 = 4,
    ObuMetadataTypeTimecode = 5,
}

pub struct ObuHeader {
    pub obu_type: ObuType,
    pub extension_flag: bool,
    pub has_size_field: bool,
    pub temporal_id: u8,
    pub spatial_id: u8,
}

// Parses OBU header and stores values in 'header'
pub fn rav1d_parse_obu_header(r: &mut BitReader) -> Result<ObuHeader, Rav1dCodecErr> {
    if r.f(1, "obu_forbidden_bit") != 0 {
        // forbidden_bit must be set to 0.
        return Err(Rav1dCodecErr::Rav1dCodecCorruptFrame);
    }

    let obu_type: ObuType;
    if let Some(t) = ObuType::from(r.f(4, "obu_type")) {
        obu_type = t;
    } else {
        return Err(Rav1dCodecErr::Rav1dCodecCorruptFrame);
    }

    let extension_flag = r.f(1, "obu_extension_flag") != 0;
    let has_size_field = r.f(1, "obu_has_size_field") != 0;

    if r.f(1, "obu_reserved_1bit") != 0 {
        // obu_reserved_1bit must be set to 0.
        return Err(Rav1dCodecErr::Rav1dCodecCorruptFrame);
    }

    let temporal_id: u8;
    let spatial_id: u8;
    if extension_flag {
        temporal_id = r.f(3, "temporal_id") as u8;
        spatial_id = r.f(2, "spatial_id") as u8;
        if r.f(3, "extension_header_reserved_3bits") != 0 {
            // extension_header_reserved_3bits must be set to 0.
            return Err(Rav1dCodecErr::Rav1dCodecCorruptFrame);
        }
    } else {
        temporal_id = 0;
        spatial_id = 0;
    }

    return Ok(ObuHeader {
        obu_type,
        extension_flag,
        has_size_field,
        temporal_id,
        spatial_id,
    });
}

pub fn rav1d_parse_obu(d: &mut Rav1Decoder, data: &[u8]) -> Result<usize, Rav1dCodecErr> {
    let mut r = BitReader::new(data);

    let obu_header = rav1d_parse_obu_header(&mut r)?;
    let obu_size = if obu_header.has_size_field {
        r.leb128("obu_size") as usize
    } else {
        data.len() - 1 - (obu_header.extension_flag as usize)
    };

    let start_position = r.get_position();
    let start_byte_position = start_position >> 3;
    assert_eq!((start_position & 7), 0);

    if obu_header.obu_type != ObuType::ObuSequenceHeader
        && obu_header.obu_type != ObuType::ObuTemporalDelimiter
        && obu_header.extension_flag
        && d.operating_point_idc != 0
    {
        let in_temporal_layer = (d.operating_point_idc >> obu_header.temporal_id) & 1;
        let in_spatial_layer = (d.operating_point_idc >> (obu_header.spatial_id + 8)) & 1;
        if in_temporal_layer == 0 || in_spatial_layer == 0 {
            return Ok(obu_size + start_byte_position);
        }
    }

    match obu_header.obu_type {
        ObuType::ObuSequenceHeader => rav1d_parse_sequence_header_obu(d, &mut r, &obu_header)?,
        ObuType::ObuTemporalDelimiter => rav1d_parse_temporal_delimiter_obu()?,
        ObuType::ObuFrameHeader => rav1d_parse_frame_header_obu()?,
        ObuType::ObuTileGroup => rav1d_parse_tile_group_obu()?,
        ObuType::ObuMetadata => rav1d_parse_metadata_obu()?,
        ObuType::ObuFrame => rav1d_parse_frame_obu()?,
        ObuType::ObuRedundantFrameHeader => rav1d_parse_redundant_frame_header_obu()?,
        ObuType::ObuTileList => rav1d_parse_tile_list_obu()?,
        ObuType::ObuPadding => rav1d_parse_padding_obu()?,
    };

    let current_position = r.get_position();
    let payload_bits = current_position - start_position;
    if obu_size > 0
        && obu_header.obu_type != ObuType::ObuTileGroup
        && obu_header.obu_type != ObuType::ObuFrame
    {
        r.trailing_bits(obu_size * 8 - payload_bits);
    }

    return Ok(obu_size + start_byte_position);
}

fn rav1d_parse_sequence_header_obu(
    d: &mut Rav1Decoder,
    r: &mut BitReader,
    oh: &ObuHeader,
) -> Result<(), Rav1dCodecErr> {
    let mut sh: SequenceHeader = Default::default();

    sh.seq_profile = r.f(3, "seq_profile") as u8;
    if sh.seq_profile > 2 {
        return Err(Rav1dCodecErr::Rav1dCodecUnsupFeature);
    }

    sh.still_picture = r.f(1, "still_picture") != 0;
    sh.reduced_still_picture_header = r.f(1, "reduced_still_picture_header") != 0;
    if sh.reduced_still_picture_header && !sh.still_picture {
        return Err(Rav1dCodecErr::Rav1dCodecUnsupBitstream);
    }

    if sh.reduced_still_picture_header {
        sh.timing_info_present_flag = false;
        sh.decoder_model_info_present_flag = false;
        sh.initial_display_delay_present_flag = false;
        sh.operating_points_cnt_minus_1 = 0;
        sh.operating_points[0].operating_point_idc = 0;
        sh.operating_points[0].seq_level_idx = r.f(5, "seq_level_idx") as u8;
        sh.operating_points[0].seq_tier = false;
        sh.operating_points[0].decoder_model_present_for_this_op = false;
        sh.operating_points[0].initial_display_delay_present_for_this_op = false;
    } else {
        sh.timing_info_present_flag = r.f(1, "timing_info_present_flag") != 0;
        if sh.timing_info_present_flag {
            sh.timing_info.num_units_in_display_tick = r.f(32, "num_units_in_display_tick");
            sh.timing_info.time_scale = r.f(32, "time_scale");
            sh.timing_info.equal_picture_interval = r.f(1, "equal_picture_interval") != 0;
            if sh.timing_info.equal_picture_interval {
                sh.timing_info.num_ticks_per_picture_minus_1 =
                    r.uvlc("num_ticks_per_picture_minus_1");
                if sh.timing_info.num_ticks_per_picture_minus_1 == 0xFFFFFFFF {
                    return Err(Rav1dCodecErr::Rav1dCodecCorruptFrame);
                }
            }

            sh.decoder_model_info_present_flag = r.f(1, "decoder_model_info_present_flag") != 0;
            if sh.decoder_model_info_present_flag {
                sh.decoder_model_info.buffer_delay_length_minus_1 =
                    r.f(5, "buffer_delay_length_minus_1") as u8;
                sh.decoder_model_info.num_units_in_decoding_tick =
                    r.f(32, "num_units_in_decoding_tick");
                sh.decoder_model_info.buffer_removal_time_length_minus_1 =
                    r.f(5, "buffer_removal_time_length_minus_1") as u8;
                sh.decoder_model_info.frame_presentation_time_length =
                    r.f(5, "frame_presentation_time_length") as u8;
            }
        } else {
            sh.decoder_model_info_present_flag = false;
        }

        sh.initial_display_delay_present_flag = r.f(1, "initial_display_delay_present_flag") != 0;
        sh.operating_points_cnt_minus_1 = r.f(5, "operating_points_cnt_minus_1") as u8;
        for i in 0..(sh.operating_points_cnt_minus_1 + 1) as usize {
            sh.operating_points[i].operating_point_idc = r.f(12, "operating_point_idc") as u16;
            sh.operating_points[i].seq_level_idx = r.f(5, "seq_level_idx") as u8;
            if sh.operating_points[i].seq_level_idx > 7 {
                sh.operating_points[i].seq_tier = r.f(1, "seq_tier") != 0;
            } else {
                sh.operating_points[i].seq_tier = false;
            }
            if sh.decoder_model_info_present_flag {
                sh.operating_points[i].decoder_model_present_for_this_op =
                    r.f(1, "decoder_model_present_for_this_op") != 0;
                if sh.operating_points[i].decoder_model_present_for_this_op {
                    let n = (sh.decoder_model_info.buffer_delay_length_minus_1 + 1) as u32;
                    sh.operating_points[i]
                        .operating_parameters_info
                        .decoder_buffer_delay = r.f(n, "decoder_buffer_delay");
                    sh.operating_points[i]
                        .operating_parameters_info
                        .encoder_buffer_dealy = r.f(n, "encoder_buffer_dealy");
                    sh.operating_points[i]
                        .operating_parameters_info
                        .low_delay_mode_flag = r.f(1, "low_delay_mode_flag") != 0;
                }
            } else {
                sh.operating_points[i].decoder_model_present_for_this_op = false;
            }
            if sh.initial_display_delay_present_flag {
                sh.operating_points[i].initial_display_delay_present_for_this_op =
                    r.f(1, "initial_display_delay_present_for_this_op") != 0;
                if sh.operating_points[i].initial_display_delay_present_for_this_op {
                    sh.operating_points[i].initial_display_delay_minus_1 =
                        r.f(4, "initial_display_delay_minus_1") as u8;
                }
            }
        }
    }
    if d.operating_point < (sh.operating_points_cnt_minus_1 + 1) as usize {
        d.operating_point_idc = sh.operating_points[d.operating_point].operating_point_idc;
    } else {
        d.operating_point_idc = sh.operating_points[0].operating_point_idc;
    }

    sh.frame_width_bits_minus_1 = r.f(4, "frame_width_bits_minus_1") as u8;
    sh.frame_height_bits_minus_1 = r.f(4, "frame_height_bits_minus_1") as u8;
    sh.max_frame_width_minus_1 = r.f(
        (sh.frame_width_bits_minus_1 + 1) as u32,
        "max_frame_width_minus_1",
    ) as u16;
    sh.max_frame_height_minus_1 = r.f(
        (sh.frame_height_bits_minus_1 + 1) as u32,
        "max_frame_height_minus_1",
    ) as u16;

    if sh.reduced_still_picture_header {
        sh.frame_id_numbers_present_flag = false;
    } else {
        sh.frame_id_numbers_present_flag = r.f(1, "frame_id_numbers_present_flag") != 0;
    }
    if sh.frame_id_numbers_present_flag {
        sh.delta_frame_id_length_minus_2 = r.f(4, "delta_frame_id_length_minus_2") as u8;
        sh.additional_frame_id_length_minus_1 = r.f(3, "additional_frame_id_length_minus_1") as u8;
        sh.frame_id_length = sh.additional_frame_id_length_minus_1 + 1 + sh.delta_frame_id_length_minus_2 + 2;
    }

    sh.use_128x128_superblock = r.f(1, "use_128x128_superblock") != 0;
    sh.enable_filter_intra = r.f(1, "enable_filter_intra") != 0;
    sh.enable_intra_edge_filter = r.f(1, "enable_intra_edge_filter") != 0;
    if sh.reduced_still_picture_header {
        sh.enable_interintra_compound = false;
        sh.enable_masked_compound = false;
        sh.enable_warped_motion = false;
        sh.enable_dual_filter = false;
        sh.enable_order_hint = false;
        sh.enable_jnt_comp = false;
        sh.enable_ref_frame_mvs = false;
    // sh.seq_force_screen_content_tools
    // sh.seq_force_integer_mv
    } else {
        sh.enable_interintra_compound = r.f(1, "enable_interintra_compound") != 0;
        sh.enable_masked_compound = r.f(1, "enable_masked_compound") != 0;
        sh.enable_warped_motion = r.f(1, "enable_warped_motion") != 0;
        sh.enable_dual_filter = r.f(1, "enable_dual_filter") != 0;
        sh.enable_order_hint = r.f(1, "enable_order_hint") != 0;
        if sh.enable_order_hint {
            sh.enable_jnt_comp = r.f(1, "enable_jnt_comp") != 0;
            sh.enable_ref_frame_mvs = r.f(1, "enable_ref_frame_mvs") != 0;
        } else {
            sh.enable_jnt_comp = false;
            sh.enable_ref_frame_mvs = false;
        }
        sh.seq_choose_screen_content_tools = r.f(1, "seq_choose_screen_content_tools") != 0;
        if sh.seq_choose_screen_content_tools {
            // sh.seq_force_screen_content_tools
        } else {
            sh.seq_force_screen_content_tools = r.f(1, "seq_force_screen_content_tools") as u8;
        }

        if sh.seq_force_screen_content_tools > 0 {
            sh.seq_choose_integer_mv = r.f(1, "seq_choose_integer_mv") != 0;
            if sh.seq_choose_integer_mv {
                // sh.seq_force_integer_mv =
            } else {
                sh.seq_force_integer_mv = r.f(1, "seq_force_integer_mv") as u8;
            }
        } else {
            // sh.seq_force_integer_mv = 0;
        }

        if sh.enable_order_hint {
            sh.order_hint_bits = 1 + r.f(3, "order_hint_bits_minus_1") as u8;
        } else {
            sh.order_hint_bits = 0;
        }
    }

    sh.enable_superres = r.f(1, "enable_superres") != 0;
    sh.enable_cdef = r.f(1, "enable_cdef") != 0;
    sh.enable_restoration = r.f(1, "enable_restoration") != 0;

    //color_config
    sh.color_config.high_bitdepth = r.f(1, "high_bitdepth") != 0;
    if sh.seq_profile == 2 && sh.color_config.high_bitdepth {
        sh.color_config.twelve_bit = r.f(1, "twelve_bit") != 0;
        sh.color_config.bit_depth = if sh.color_config.twelve_bit { 12 } else { 10 };
    } else if sh.seq_profile <= 2 {
        sh.color_config.bit_depth = if sh.color_config.high_bitdepth { 10 } else { 8 };
    }
    if sh.seq_profile == 1 {
        sh.color_config.mono_chrome = false;
    } else {
        sh.color_config.mono_chrome = r.f(1, "mono_chrome") != 0;
    }
    sh.color_config.color_description_present_flag = r.f(1, "color_description_present_flag") != 0;
    if sh.color_config.color_description_present_flag {
        sh.color_config.color_primaries = r.f(8, "color_primaries") as u8;
        sh.color_config.transfer_characteristics = r.f(8, "transfer_characteristics") as u8;
        sh.color_config.matrix_coefficients = r.f(8, "matrix_coefficients") as u8;
    } else {
        sh.color_config.color_primaries = 2; //CP_UNSPECIFIED
        sh.color_config.transfer_characteristics = 2; //TC_UNSPECIFIED
        sh.color_config.matrix_coefficients = 2; //MC_UNSPECIFIED
    }
    if sh.color_config.mono_chrome {
        sh.color_config.color_range = r.f(1, "color_range") != 0;
        sh.color_config.subsampling_x = 1;
        sh.color_config.subsampling_y = 1;
        sh.color_config.chroma_sample_position = 0; //CSP_UNKNOWN
        sh.color_config.separate_uv_delta_q = false;
    } else {
        if sh.color_config.color_primaries == 1 /*CP_BT_709*/ &&
            sh.color_config.transfer_characteristics == 13 /*TC_SRGB*/ &&
            sh.color_config.matrix_coefficients == 0
        /*MC_IDENTITY*/
        {
            sh.color_config.color_range = true;
            sh.color_config.subsampling_x = 0;
            sh.color_config.subsampling_y = 0;
        } else {
            sh.color_config.color_range = r.f(1, "color_range") != 0;
            if sh.seq_profile == 0 {
                sh.color_config.subsampling_x = 1;
                sh.color_config.subsampling_y = 1;
            } else if sh.seq_profile == 1 {
                sh.color_config.subsampling_x = 0;
                sh.color_config.subsampling_y = 0;
            } else {
                if sh.color_config.bit_depth == 12 {
                    sh.color_config.subsampling_x = r.f(1, "subsampling_x") as u8;
                    if sh.color_config.subsampling_x != 0 {
                        sh.color_config.subsampling_y = r.f(1, "subsampling_y") as u8;
                    } else {
                        sh.color_config.subsampling_y = 0;
                    }
                } else {
                    sh.color_config.subsampling_x = 1;
                    sh.color_config.subsampling_y = 0;
                }
            }
            if sh.color_config.subsampling_x != 0 && sh.color_config.subsampling_y != 0 {
                sh.color_config.chroma_sample_position = r.f(2, "chroma_sample_position") as u8;
            }
        }
        sh.color_config.separate_uv_delta_q = r.f(1, "separate_uv_delta_q") != 0;
    }

    sh.film_grain_params_present = r.f(1, "film_grain_params_present") != 0;

    Ok(())
}

fn rav1d_parse_temporal_delimiter_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}

fn rav1d_parse_frame_header_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}

fn rav1d_parse_tile_group_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}

fn rav1d_parse_metadata_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}

fn rav1d_parse_frame_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}

fn rav1d_parse_redundant_frame_header_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}

fn rav1d_parse_tile_list_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}

fn rav1d_parse_padding_obu() -> Result<(), Rav1dCodecErr> {
    Ok(())
}
