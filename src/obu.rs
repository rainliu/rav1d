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
    obu_type: ObuType,
    extension_flag: bool,
    has_size_field: bool,
    temporal_id: u8,
    spatial_id: u8,
}

// Parses OBU header and stores values in 'header'
pub fn rav1d_parse_obu_header(r: &mut BitReader) -> Result<ObuHeader, aom_codec_err_t> {
    if r.f(1, "obu_forbidden_bit") != 0 {
        // forbidden_bit must be set to 0.
        return Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME);
    }

    let obu_type: ObuType;
    if let Some(t) = ObuType::from(r.f(4, "obu_type")) {
        obu_type = t;
    } else {
        return Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME);
    }

    let extension_flag = r.f(1, "obu_extension_flag") != 0;
    let has_size_field = r.f(1, "obu_has_size_field") != 0;

    if r.f(1, "obu_reserved_1bit") != 0 {
        // obu_reserved_1bit must be set to 0.
        return Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME);
    }

    let temporal_id: u8;
    let spatial_id: u8;
    if extension_flag {
        temporal_id = r.f(3, "temporal_id") as u8;
        spatial_id = r.f(2, "spatial_id") as u8;
        if r.f(3, "extension_header_reserved_3bits") != 0 {
            // extension_header_reserved_3bits must be set to 0.
            return Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME);
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

pub fn rav1d_parse_obu(dec: &Rav1Decoder, data: &[u8]) -> Result<usize, aom_codec_err_t> {
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

    let operating_point_idc = 0;
    if obu_header.obu_type != ObuType::ObuSequenceHeader
        && obu_header.obu_type != ObuType::ObuTemporalDelimiter
        && obu_header.extension_flag
        && dec.operating_point_idc != 0
    {
        let in_temporal_layer = (dec.operating_point_idc >> obu_header.temporal_id) & 1;
        let in_spatial_layer = (dec.operating_point_idc >> (obu_header.spatial_id + 8)) & 1;
        if in_temporal_layer == 0 || in_spatial_layer == 0 {
            return Ok(obu_size + start_byte_position);
        }
    }

    match obu_header.obu_type {
        ObuType::ObuSequenceHeader => rav1d_parse_sequence_header_obu(),
        ObuType::ObuTemporalDelimiter => rav1d_parse_temporal_delimiter_obu(),
        ObuType::ObuFrameHeader => rav1d_parse_frame_header_obu(),
        ObuType::ObuTileGroup => rav1d_parse_tile_group_obu(),
        ObuType::ObuMetadata => rav1d_parse_metadata_obu(),
        ObuType::ObuFrame => rav1d_parse_frame_obu(),
        ObuType::ObuRedundantFrameHeader => rav1d_parse_redundant_frame_header_obu(),
        ObuType::ObuTileList => rav1d_parse_tile_list_obu(),
        ObuType::ObuPadding => rav1d_parse_padding_obu(),
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

fn rav1d_parse_sequence_header_obu() {}

fn rav1d_parse_temporal_delimiter_obu() {}

fn rav1d_parse_frame_header_obu() {}

fn rav1d_parse_tile_group_obu() {}

fn rav1d_parse_metadata_obu() {}
fn rav1d_parse_frame_obu() {}
fn rav1d_parse_redundant_frame_header_obu() {}
fn rav1d_parse_tile_list_obu() {}
fn rav1d_parse_padding_obu() {}
