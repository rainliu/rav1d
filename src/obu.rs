use super::bitreader::*;
use super::global::*;

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
    pub fn from(obu_type: i32) -> Option<Self> {
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
pub fn rav1d_parse_obu_header(br: &mut BitReader) -> Result<ObuHeader, aom_codec_err_t> {
    if br.read_bit()? != 0 {
        // forbidden_bit must be set to 0.
        return Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME);
    }

    let obu_type: ObuType;
    if let Some(t) = ObuType::from(br.read_literal(4)?) {
        obu_type = t;
    } else {
        return Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME);
    }

    let extension_flag = br.read_bit()? != 0;
    let has_size_field = br.read_bit()? != 0;

    if br.read_bit()? != 0 {
        // obu_reserved_1bit must be set to 0.
        return Err(aom_codec_err_t::AOM_CODEC_CORRUPT_FRAME);
    }

    let temporal_id: u8;
    let spatial_id: u8;
    if extension_flag {
        temporal_id = br.read_literal(3)? as u8;
        spatial_id = br.read_literal(2)? as u8;
        if br.read_literal(3)? != 0 {
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

// Parses OBU size
pub fn rav1d_parse_obu_size(br: &mut BitReader) -> Result<usize, aom_codec_err_t> {
    let mut value: u64 = 0;
    for i in 0..8 {
        let mut leb128_byte = br.read_unsigned_literal(8)?;
        value |= ((leb128_byte & 0x7f) as u64) << (i * 7);
        leb128_byte += 1;
        if (leb128_byte & 0x80) == 0 {
            break;
        }
    }
    Ok(value as usize)
}

pub fn rav1d_parse_obu(data: &[u8]) -> Result<(), aom_codec_err_t> {
    let mut br = BitReader::new(data, 0);

    let obu_header = rav1d_parse_obu_header(&mut br)?;
    let obu_size = if obu_header.has_size_field {
        rav1d_parse_obu_size(&mut br)?
    } else {
        data.len() - 1 - (obu_header.extension_flag as usize)
    };

    return Ok(());
}
