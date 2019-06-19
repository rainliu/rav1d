use crate::headers::*;

struct Av1FilterLUT {
    e: [u8; 64],
    i: [u8; 64],
    sharp: [u64; 2],
}

struct Av1RestorationUnit {
    t: RestorationType,
    filter_h: [i16; 3],
    filter_v: [i16; 3],
    sgr_idx: u8,
    sgr_weights: [i16; 2],
}

// each struct describes one 128x128 area (1 or 4 SBs), pre-superres-scaling
struct Av1Filter {
    // each bit is 1 col
    filter_y: [[[[u16; 2]; 3]; 32]; 2],
    filter_uv: [[[[u16; 2]; 2]; 32]; 2],
    cdef_idx: [i8; 4], // -1 means "unset"
    noskip_mask: [[u16; 2]; 32],
}

// each struct describes one 128x128 area (1 or 4 SBs), post-superres-scaling
struct Av1Restoration {
    lr: [[Av1RestorationUnit; 4]; 3],
}
