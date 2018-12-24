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

pub enum ObuMetadataTye {
    ObuMetadataTypeAomReserved0 = 0,
    ObuMetadataTypeHdrCll = 1,
    ObuMetadataTypeHdrMdcv = 2,
    ObuMetadataTypeScalability = 3,
    ObuMetadataTypeItutT35 = 4,
    ObuMetadataTypeTimecode = 5,
}

pub struct ObuHeader {
    size: isize,
    obu_type: ObuType,
    has_size_field: bool,
    has_extension: bool,
    temporal_layer_id: i32,
    spatial_layer_id: i32,
}


