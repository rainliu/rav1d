use self::BlockSize::*;
use self::PredictionMode::*;
use self::TxSize::*;
use self::TxType::*;

use crate::plane::*;


pub const PLANES: usize = 3;

const PARTITION_PLOFFSET: usize = 4;
const PARTITION_BLOCK_SIZES: usize = 4 + 1;
const PARTITION_CONTEXTS_PRIMARY: usize = PARTITION_BLOCK_SIZES * PARTITION_PLOFFSET;
pub const PARTITION_CONTEXTS: usize = PARTITION_CONTEXTS_PRIMARY;
pub const PARTITION_TYPES: usize = 4;

pub const MI_SIZE_LOG2: usize = 2;
pub const MI_SIZE: usize = (1 << MI_SIZE_LOG2);
const MAX_MIB_SIZE_LOG2: usize = (MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2);
pub const MAX_MIB_SIZE: usize = (1 << MAX_MIB_SIZE_LOG2);
pub const MAX_MIB_MASK: usize = (MAX_MIB_SIZE - 1);

const MAX_SB_SIZE_LOG2: usize = 6;
pub const MAX_SB_SIZE: usize = (1 << MAX_SB_SIZE_LOG2);
const MAX_SB_SQUARE: usize = (MAX_SB_SIZE * MAX_SB_SIZE);

pub const MAX_TX_SIZE: usize = 64;
const MAX_TX_SQUARE: usize = MAX_TX_SIZE * MAX_TX_SIZE;

pub const INTRA_MODES: usize = 13;
pub const UV_INTRA_MODES: usize = 14;

pub const CFL_JOINT_SIGNS: usize = 8;
pub const CFL_ALPHA_CONTEXTS: usize = 6;
pub const CFL_ALPHABET_SIZE: usize = 16;
pub const SKIP_MODE_CONTEXTS: usize = 3;
pub const COMP_INDEX_CONTEXTS: usize = 6;
pub const COMP_GROUP_IDX_CONTEXTS: usize = 6;

pub const BLOCK_SIZE_GROUPS: usize = 4;
pub const MAX_ANGLE_DELTA: usize = 3;
pub const DIRECTIONAL_MODES: usize = 8;
pub const KF_MODE_CONTEXTS: usize = 5;

pub const EXT_PARTITION_TYPES: usize = 10;

pub const TX_SIZE_SQR_CONTEXTS: usize = 4; // Coded tx_size <= 32x32, so is the # of CDF contexts from tx sizes

pub const TX_SETS: usize = 9;
pub const TX_SETS_INTRA: usize = 3;
pub const TX_SETS_INTER: usize = 4;
pub const TXFM_PARTITION_CONTEXTS: usize = ((TxSize::TX_SIZES - TxSize::TX_8X8 as usize) * 6 - 3);
const MAX_REF_MV_STACK_SIZE: usize = 8;
pub const REF_CAT_LEVEL: u32 = 640;

pub const FRAME_LF_COUNT: usize = 4;
pub const MAX_LOOP_FILTER: usize = 63;
const DELTA_LF_SMALL: u32 = 3;
pub const DELTA_LF_PROBS: usize = DELTA_LF_SMALL as usize;

const DELTA_Q_SMALL: u32 = 3;
pub const DELTA_Q_PROBS: usize = DELTA_Q_SMALL as usize;

// Number of transform types in each set type
static num_tx_set: [usize; TX_SETS] = [1, 2, 5, 7, 7, 10, 12, 16, 16];
pub static av1_tx_used: [[usize; TX_TYPES]; TX_SETS] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// Maps set types above to the indices used for intra
static tx_set_index_intra: [i8; TX_SETS] = [0, -1, 2, -1, 1, -1, -1, -1, -16];
// Maps set types above to the indices used for inter
static tx_set_index_inter: [i8; TX_SETS] = [0, 3, -1, -1, -1, -1, 2, -1, 1];

static av1_tx_ind: [[usize; TX_TYPES]; TX_SETS] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 3, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 5, 6, 4, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0],
    [1, 5, 6, 4, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0],
    [1, 2, 3, 6, 4, 5, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0],
    [3, 4, 5, 8, 6, 7, 9, 10, 11, 0, 1, 2, 0, 0, 0, 0],
    [7, 8, 9, 12, 10, 11, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6],
    [7, 8, 9, 12, 10, 11, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6],
];

pub static max_txsize_rect_lookup: [TxSize; BlockSize::BLOCK_SIZES_ALL] = [
    // 4X4
    TX_4X4, // 4X8,    8X4,      8X8
    TX_4X8, TX_8X4, TX_8X8, // 8X16,   16X8,     16X16
    TX_8X16, TX_16X8, TX_16X16, // 16X32,  32X16,    32X32
    TX_16X32, TX_32X16, TX_32X32, // 32X64,  64X32,
    TX_32X64, TX_64X32, // 64X64
    TX_64X64, // 64x128, 128x64,   128x128
    TX_64X64, TX_64X64, TX_64X64, // 4x16,   16x4,
    TX_4X16, TX_16X4, // 8x32,   32x8
    TX_8X32, TX_32X8, // 16x64,  64x16
    TX_16X64, TX_64X16,
];

pub static sub_tx_size_map: [TxSize; TxSize::TX_SIZES_ALL] = [
    TX_4X4,   // TX_4X4
    TX_4X4,   // TX_8X8
    TX_8X8,   // TX_16X16
    TX_16X16, // TX_32X32
    TX_32X32, // TX_64X64
    TX_4X4,   // TX_4X8
    TX_4X4,   // TX_8X4
    TX_8X8,   // TX_8X16
    TX_8X8,   // TX_16X8
    TX_16X16, // TX_16X32
    TX_16X16, // TX_32X16
    TX_32X32, // TX_32X64
    TX_32X32, // TX_64X32
    TX_4X8,   // TX_4X16
    TX_8X4,   // TX_16X4
    TX_8X16,  // TX_8X32
    TX_16X8,  // TX_32X8
    TX_16X32, // TX_16X64
    TX_32X16, // TX_64X16
];

static ss_size_lookup: [[[BlockSize; 2]; 2]; BlockSize::BLOCK_SIZES_ALL] = [
    //  ss_x == 0    ss_x == 0        ss_x == 1      ss_x == 1
    //  ss_y == 0    ss_y == 1        ss_y == 0      ss_y == 1
    [[BLOCK_4X4, BLOCK_4X4], [BLOCK_4X4, BLOCK_4X4]],
    [[BLOCK_4X8, BLOCK_4X4], [BLOCK_4X4, BLOCK_4X4]],
    [[BLOCK_8X4, BLOCK_4X4], [BLOCK_4X4, BLOCK_4X4]],
    [[BLOCK_8X8, BLOCK_8X4], [BLOCK_4X8, BLOCK_4X4]],
    [[BLOCK_8X16, BLOCK_8X8], [BLOCK_4X16, BLOCK_4X8]],
    [[BLOCK_16X8, BLOCK_16X4], [BLOCK_8X8, BLOCK_8X4]],
    [[BLOCK_16X16, BLOCK_16X8], [BLOCK_8X16, BLOCK_8X8]],
    [[BLOCK_16X32, BLOCK_16X16], [BLOCK_8X32, BLOCK_8X16]],
    [[BLOCK_32X16, BLOCK_32X8], [BLOCK_16X16, BLOCK_16X8]],
    [[BLOCK_32X32, BLOCK_32X16], [BLOCK_16X32, BLOCK_16X16]],
    [[BLOCK_32X64, BLOCK_32X32], [BLOCK_16X64, BLOCK_16X32]],
    [[BLOCK_64X32, BLOCK_64X16], [BLOCK_32X32, BLOCK_32X16]],
    [[BLOCK_64X64, BLOCK_64X32], [BLOCK_32X64, BLOCK_32X32]],
    [[BLOCK_64X128, BLOCK_64X64], [BLOCK_INVALID, BLOCK_32X64]],
    [[BLOCK_128X64, BLOCK_INVALID], [BLOCK_64X64, BLOCK_64X32]],
    [[BLOCK_128X128, BLOCK_128X64], [BLOCK_64X128, BLOCK_64X64]],
    [[BLOCK_4X16, BLOCK_4X8], [BLOCK_4X16, BLOCK_4X8]],
    [[BLOCK_16X4, BLOCK_16X4], [BLOCK_8X4, BLOCK_8X4]],
    [[BLOCK_8X32, BLOCK_8X16], [BLOCK_INVALID, BLOCK_4X16]],
    [[BLOCK_32X8, BLOCK_INVALID], [BLOCK_16X8, BLOCK_16X4]],
    [[BLOCK_16X64, BLOCK_16X32], [BLOCK_INVALID, BLOCK_8X32]],
    [[BLOCK_64X16, BLOCK_INVALID], [BLOCK_32X16, BLOCK_32X8]],
];

pub fn get_plane_block_size(
    bsize: BlockSize,
    subsampling_x: usize,
    subsampling_y: usize,
) -> BlockSize {
    ss_size_lookup[bsize as usize][subsampling_x][subsampling_y]
}

// Generates 4 bit field in which each bit set to 1 represents
// a blocksize partition  1111 means we split 64x64, 32x32, 16x16
// and 8x8.  1000 means we just split the 64x64 to 32x32
static partition_context_lookup: [[u8; 2]; BlockSize::BLOCK_SIZES_ALL] = [
    [31, 31], // 4X4   - {0b11111, 0b11111}
    [31, 30], // 4X8   - {0b11111, 0b11110}
    [30, 31], // 8X4   - {0b11110, 0b11111}
    [30, 30], // 8X8   - {0b11110, 0b11110}
    [30, 28], // 8X16  - {0b11110, 0b11100}
    [28, 30], // 16X8  - {0b11100, 0b11110}
    [28, 28], // 16X16 - {0b11100, 0b11100}
    [28, 24], // 16X32 - {0b11100, 0b11000}
    [24, 28], // 32X16 - {0b11000, 0b11100}
    [24, 24], // 32X32 - {0b11000, 0b11000}
    [24, 16], // 32X64 - {0b11000, 0b10000}
    [16, 24], // 64X32 - {0b10000, 0b11000}
    [16, 16], // 64X64 - {0b10000, 0b10000}
    [16, 0],  // 64X128- {0b10000, 0b00000}
    [0, 16],  // 128X64- {0b00000, 0b10000}
    [0, 0],   // 128X128-{0b00000, 0b00000}
    [31, 28], // 4X16  - {0b11111, 0b11100}
    [28, 31], // 16X4  - {0b11100, 0b11111}
    [30, 24], // 8X32  - {0b11110, 0b11000}
    [24, 30], // 32X8  - {0b11000, 0b11110}
    [28, 16], // 16X64 - {0b11100, 0b10000}
    [16, 28], // 64X16 - {0b10000, 0b11100}
];

static size_group_lookup: [u8; BlockSize::BLOCK_SIZES_ALL] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 0, 0, 1, 1, 2, 2,
];

static num_pels_log2_lookup: [u8; BlockSize::BLOCK_SIZES_ALL] = [
    4, 5, 5, 6, 7, 7, 8, 9, 9, 10, 11, 11, 12, 13, 13, 14, 6, 6, 8, 8, 10, 10,
];

pub const PLANE_TYPES: usize = 2;
const REF_TYPES: usize = 2;
pub const SKIP_CONTEXTS: usize = 3;
pub const INTRA_INTER_CONTEXTS: usize = 4;
pub const INTER_MODE_CONTEXTS: usize = 8;
pub const DRL_MODE_CONTEXTS: usize = 3;
pub const COMP_INTER_CONTEXTS: usize = 5;
pub const COMP_REF_TYPE_CONTEXTS: usize = 5;
pub const UNI_COMP_REF_CONTEXTS: usize = 3;

// Level Map
pub const TXB_SKIP_CONTEXTS: usize = 13;

pub const EOB_COEF_CONTEXTS: usize = 9;

const SIG_COEF_CONTEXTS_2D: usize = 26;
const SIG_COEF_CONTEXTS_1D: usize = 16;
pub const SIG_COEF_CONTEXTS_EOB: usize = 4;
pub const SIG_COEF_CONTEXTS: usize = SIG_COEF_CONTEXTS_2D + SIG_COEF_CONTEXTS_1D;

const COEFF_BASE_CONTEXTS: usize = SIG_COEF_CONTEXTS;
pub const DC_SIGN_CONTEXTS: usize = 3;

const BR_TMP_OFFSET: usize = 12;
const BR_REF_CAT: usize = 4;
pub const LEVEL_CONTEXTS: usize = 21;

pub const NUM_BASE_LEVELS: usize = 2;

pub const BR_CDF_SIZE: usize = 4;
const COEFF_BASE_RANGE: usize = 4 * (BR_CDF_SIZE - 1);

const COEFF_CONTEXT_BITS: usize = 6;
const COEFF_CONTEXT_MASK: usize = (1 << COEFF_CONTEXT_BITS) - 1;
const MAX_BASE_BR_RANGE: usize = COEFF_BASE_RANGE + NUM_BASE_LEVELS + 1;

const BASE_CONTEXT_POSITION_NUM: usize = 12;

// Pad 4 extra columns to remove horizontal availability check.
const TX_PAD_HOR_LOG2: usize = 2;
const TX_PAD_HOR: usize = 4;
// Pad 6 extra rows (2 on top and 4 on bottom) to remove vertical availability
// check.
const TX_PAD_TOP: usize = 2;
const TX_PAD_BOTTOM: usize = 4;
const TX_PAD_VER: usize = (TX_PAD_TOP + TX_PAD_BOTTOM);
// Pad 16 extra bytes to avoid reading overflow in SIMD optimization.
const TX_PAD_END: usize = 16;
const TX_PAD_2D: usize = ((MAX_TX_SIZE + TX_PAD_HOR) * (MAX_TX_SIZE + TX_PAD_VER) + TX_PAD_END);

const TX_CLASSES: usize = 3;

#[derive(Copy, Clone, PartialEq)]
pub enum TxClass {
    TX_CLASS_2D = 0,
    TX_CLASS_HORIZ = 1,
    TX_CLASS_VERT = 2,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SegLvl {
    SEG_LVL_ALT_Q = 0,      /* Use alternate Quantizer .... */
    SEG_LVL_ALT_LF_Y_V = 1, /* Use alternate loop filter value on y plane vertical */
    SEG_LVL_ALT_LF_Y_H = 2, /* Use alternate loop filter value on y plane horizontal */
    SEG_LVL_ALT_LF_U = 3,   /* Use alternate loop filter value on u plane */
    SEG_LVL_ALT_LF_V = 4,   /* Use alternate loop filter value on v plane */
    SEG_LVL_REF_FRAME = 5,  /* Optional Segment reference frame */
    SEG_LVL_SKIP = 6,       /* Optional Segment (0,0) + skip mode */
    SEG_LVL_GLOBALMV = 7,
    SEG_LVL_MAX = 8,
}

pub const seg_feature_bits: [u32; SegLvl::SEG_LVL_MAX as usize] = [8, 6, 6, 6, 6, 3, 0, 0];

pub const seg_feature_is_signed: [bool; SegLvl::SEG_LVL_MAX as usize] =
    [true, true, true, true, true, false, false, false];

use crate::context::TxClass::*;

static tx_type_to_class: [TxClass; TX_TYPES] = [
    TX_CLASS_2D,    // DCT_DCT
    TX_CLASS_2D,    // ADST_DCT
    TX_CLASS_2D,    // DCT_ADST
    TX_CLASS_2D,    // ADST_ADST
    TX_CLASS_2D,    // FLIPADST_DCT
    TX_CLASS_2D,    // DCT_FLIPADST
    TX_CLASS_2D,    // FLIPADST_FLIPADST
    TX_CLASS_2D,    // ADST_FLIPADST
    TX_CLASS_2D,    // FLIPADST_ADST
    TX_CLASS_2D,    // IDTX
    TX_CLASS_VERT,  // V_DCT
    TX_CLASS_HORIZ, // H_DCT
    TX_CLASS_VERT,  // V_ADST
    TX_CLASS_HORIZ, // H_ADST
    TX_CLASS_VERT,  // V_FLIPADST
    TX_CLASS_HORIZ, // H_FLIPADST
];

static eob_to_pos_small: [u8; 33] = [
    0, 1, 2, // 0-2
    3, 3, // 3-4
    4, 4, 4, 4, // 5-8
    5, 5, 5, 5, 5, 5, 5, 5, // 9-16
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, // 17-32
];

static eob_to_pos_large: [u8; 17] = [
    6, // place holder
    7, // 33-64
    8, 8, // 65-128
    9, 9, 9, 9, // 129-256
    10, 10, 10, 10, 10, 10, 10, 10, // 257-512
    11, // 513-
];

static k_eob_group_start: [u16; 12] = [0, 1, 2, 3, 5, 9, 17, 33, 65, 129, 257, 513];
static k_eob_offset_bits: [u16; 12] = [0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn clip_max3(x: u8) -> u8 {
    if x > 3 {
        3
    } else {
        x
    }
}

// The ctx offset table when TX is TX_CLASS_2D.
// TX col and row indices are clamped to 4

#[rustfmt::skip]
static av1_nz_map_ctx_offset: [[[i8; 5]; 5]; TxSize::TX_SIZES_ALL] = [
    // TX_4X4
    [
        [ 0,  1,  6,  6, 0],
        [ 1,  6,  6, 21, 0],
        [ 6,  6, 21, 21, 0],
        [ 6, 21, 21, 21, 0],
        [ 0,  0,  0,  0, 0]
    ],
    // TX_8X8
    [
        [ 0,  1,  6,  6, 21],
        [ 1,  6,  6, 21, 21],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_16X16
    [
        [ 0,  1,  6,  6, 21],
        [ 1,  6,  6, 21, 21],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_32X32
    [
        [ 0,  1,  6,  6, 21],
        [ 1,  6,  6, 21, 21],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_64X64
    [
        [ 0,  1,  6,  6, 21],
        [ 1,  6,  6, 21, 21],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_4X8
    [
        [ 0, 11, 11, 11, 0],
        [11, 11, 11, 11, 0],
        [ 6,  6, 21, 21, 0],
        [ 6, 21, 21, 21, 0],
        [21, 21, 21, 21, 0]
    ],
    // TX_8X4
    [
        [ 0, 16,  6,  6, 21],
        [16, 16,  6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [ 0,  0,  0,  0, 0]
    ],
    // TX_8X16
    [
        [ 0, 11, 11, 11, 11],
        [11, 11, 11, 11, 11],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_16X8
    [
        [ 0, 16,  6,  6, 21],
        [16, 16,  6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21]
    ],
    // TX_16X32
    [
        [ 0, 11, 11, 11, 11],
        [11, 11, 11, 11, 11],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_32X16
    [
        [ 0, 16,  6,  6, 21],
        [16, 16,  6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21]
    ],
    // TX_32X64
    [
        [ 0, 11, 11, 11, 11],
        [11, 11, 11, 11, 11],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_64X32
    [
        [ 0, 16,  6,  6, 21],
        [16, 16,  6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21]
    ],
    // TX_4X16
    [
        [ 0, 11, 11, 11, 0],
        [11, 11, 11, 11, 0],
        [ 6,  6, 21, 21, 0],
        [ 6, 21, 21, 21, 0],
        [21, 21, 21, 21, 0]
    ],
    // TX_16X4
    [
        [ 0, 16,  6,  6, 21],
        [16, 16,  6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [ 0,  0,  0,  0, 0]
    ],
    // TX_8X32
    [
        [ 0, 11, 11, 11, 11],
        [11, 11, 11, 11, 11],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_32X8
    [
        [ 0, 16,  6,  6, 21],
        [16, 16,  6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21]
    ],
    // TX_16X64
    [
        [ 0, 11, 11, 11, 11],
        [11, 11, 11, 11, 11],
        [ 6,  6, 21, 21, 21],
        [ 6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21]
    ],
    // TX_64X16
    [
        [ 0, 16,  6,  6, 21],
        [16, 16,  6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21]
    ]
];

const NZ_MAP_CTX_0: usize = SIG_COEF_CONTEXTS_2D;
const NZ_MAP_CTX_5: usize = (NZ_MAP_CTX_0 + 5);
const NZ_MAP_CTX_10: usize = (NZ_MAP_CTX_0 + 10);

static nz_map_ctx_offset_1d: [usize; 32] = [
    NZ_MAP_CTX_0,
    NZ_MAP_CTX_5,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
    NZ_MAP_CTX_10,
];

const CONTEXT_MAG_POSITION_NUM: usize = 3;

static mag_ref_offset_with_txclass: [[[usize; 2]; CONTEXT_MAG_POSITION_NUM]; 3] = [
    [[0, 1], [1, 0], [1, 1]],
    [[0, 1], [1, 0], [0, 2]],
    [[0, 1], [1, 0], [2, 0]],
];

// End of Level Map

pub fn has_chroma(
    bo: BlockOffset,
    bsize: BlockSize,
    subsampling_x: usize,
    subsampling_y: usize,
) -> bool {
    let bw = bsize.width_mi();
    let bh = bsize.height_mi();

    ((bo.x & 0x01) == 1 || (bw & 0x01) == 0 || subsampling_x == 0)
        && ((bo.y & 0x01) == 1 || (bh & 0x01) == 0 || subsampling_y == 0)
}

pub fn get_tx_set(tx_size: TxSize, is_inter: bool, use_reduced_set: bool) -> TxSet {
    let tx_size_sqr_up = tx_size.sqr_up();
    let tx_size_sqr = tx_size.sqr();

    if tx_size.width() >= 64 || tx_size.height() >= 64 {
        return TxSet::TX_SET_DCTONLY;
    }

    if tx_size_sqr_up == TxSize::TX_32X32 {
        return if is_inter {
            TxSet::TX_SET_DCT_IDTX
        } else {
            TxSet::TX_SET_DCTONLY
        };
    }

    if use_reduced_set {
        return if is_inter {
            TxSet::TX_SET_DCT_IDTX
        } else {
            TxSet::TX_SET_DTT4_IDTX
        };
    }

    if is_inter {
        return if tx_size_sqr == TxSize::TX_16X16 {
            TxSet::TX_SET_DTT9_IDTX_1DDCT
        } else {
            TxSet::TX_SET_ALL16
        };
    }

    if tx_size_sqr == TxSize::TX_16X16 {
        TxSet::TX_SET_DTT4_IDTX
    } else {
        TxSet::TX_SET_DTT4_IDTX_1DDCT
    }
}

fn get_tx_set_index(tx_size: TxSize, is_inter: bool, use_reduced_set: bool) -> i8 {
    let set_type = get_tx_set(tx_size, is_inter, use_reduced_set);

    if is_inter {
        tx_set_index_inter[set_type as usize]
    } else {
        tx_set_index_intra[set_type as usize]
    }
}

static intra_mode_to_tx_type_context: [TxType; INTRA_MODES] = [
    DCT_DCT,   // DC
    ADST_DCT,  // V
    DCT_ADST,  // H
    DCT_DCT,   // D45
    ADST_ADST, // D135
    ADST_DCT,  // D117
    DCT_ADST,  // D153
    DCT_ADST,  // D207
    ADST_DCT,  // D63
    ADST_ADST, // SMOOTH
    ADST_DCT,  // SMOOTH_V
    DCT_ADST,  // SMOOTH_H
    ADST_ADST, // PAETH
];

static uv2y: [PredictionMode; UV_INTRA_MODES] = [
    DC_PRED,       // UV_DC_PRED
    V_PRED,        // UV_V_PRED
    H_PRED,        // UV_H_PRED
    D45_PRED,      // UV_D45_PRED
    D135_PRED,     // UV_D135_PRED
    D117_PRED,     // UV_D117_PRED
    D153_PRED,     // UV_D153_PRED
    D207_PRED,     // UV_D207_PRED
    D63_PRED,      // UV_D63_PRED
    SMOOTH_PRED,   // UV_SMOOTH_PRED
    SMOOTH_V_PRED, // UV_SMOOTH_V_PRED
    SMOOTH_H_PRED, // UV_SMOOTH_H_PRED
    PAETH_PRED,    // UV_PAETH_PRED
    DC_PRED,       // CFL_PRED
];

pub fn uv_intra_mode_to_tx_type_context(pred: PredictionMode) -> TxType {
    intra_mode_to_tx_type_context[uv2y[pred as usize] as usize]
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum PartitionType {
    PARTITION_NONE,
    PARTITION_HORZ,
    PARTITION_VERT,
    PARTITION_SPLIT,
    PARTITION_HORZ_A, // HORZ split and the top partition is split again
    PARTITION_HORZ_B, // HORZ split and the bottom partition is split again
    PARTITION_VERT_A, // VERT split and the left partition is split again
    PARTITION_VERT_B, // VERT split and the right partition is split again
    PARTITION_HORZ_4, // 4:1 horizontal partition
    PARTITION_VERT_4, // 4:1 vertical partition
    PARTITION_INVALID
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum BlockSize {
    BLOCK_4X4,
    BLOCK_4X8,
    BLOCK_8X4,
    BLOCK_8X8,
    BLOCK_8X16,
    BLOCK_16X8,
    BLOCK_16X16,
    BLOCK_16X32,
    BLOCK_32X16,
    BLOCK_32X32,
    BLOCK_32X64,
    BLOCK_64X32,
    BLOCK_64X64,
    BLOCK_64X128,
    BLOCK_128X64,
    BLOCK_128X128,
    BLOCK_4X16,
    BLOCK_16X4,
    BLOCK_8X32,
    BLOCK_32X8,
    BLOCK_16X64,
    BLOCK_64X16,
    BLOCK_INVALID,
}

impl BlockSize {
    pub const BLOCK_SIZES_ALL: usize = 22;

    const BLOCK_SIZE_WIDTH_LOG2: [usize; BlockSize::BLOCK_SIZES_ALL] = [
        2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 2, 4, 3, 5, 4, 6,
    ];

    const BLOCK_SIZE_HEIGHT_LOG2: [usize; BlockSize::BLOCK_SIZES_ALL] = [
        2, 3, 2, 3, 4, 3, 4, 5, 4, 5, 6, 5, 6, 7, 6, 7, 4, 2, 5, 3, 6, 4,
    ];

    pub fn from_width_and_height(w: usize, h: usize) -> BlockSize {
        match (w, h) {
            (4, 4) => BLOCK_4X4,
            (4, 8) => BLOCK_4X8,
            (8, 4) => BLOCK_8X4,
            (8, 8) => BLOCK_8X8,
            (8, 16) => BLOCK_8X16,
            (16, 8) => BLOCK_16X8,
            (16, 16) => BLOCK_16X16,
            (16, 32) => BLOCK_16X32,
            (32, 16) => BLOCK_32X16,
            (32, 32) => BLOCK_32X32,
            (32, 64) => BLOCK_32X64,
            (64, 32) => BLOCK_64X32,
            (64, 64) => BLOCK_64X64,
            (64, 128) => BLOCK_64X128,
            (128, 64) => BLOCK_128X64,
            (128, 128) => BLOCK_128X128,
            (4, 16) => BLOCK_4X16,
            (16, 4) => BLOCK_16X4,
            (8, 32) => BLOCK_8X32,
            (32, 8) => BLOCK_32X8,
            (16, 64) => BLOCK_16X64,
            (64, 16) => BLOCK_64X16,
            _ => unreachable!(),
        }
    }

    pub fn cfl_allowed(self) -> bool {
        // TODO: fix me when enabling EXT_PARTITION_TYPES
        self <= BlockSize::BLOCK_32X32
    }

    pub fn width(self) -> usize {
        1 << self.width_log2()
    }

    pub fn width_log2(self) -> usize {
        BlockSize::BLOCK_SIZE_WIDTH_LOG2[self as usize]
    }

    pub fn width_mi(self) -> usize {
        self.width() >> MI_SIZE_LOG2
    }

    pub fn height(self) -> usize {
        1 << self.height_log2()
    }

    pub fn height_log2(self) -> usize {
        BlockSize::BLOCK_SIZE_HEIGHT_LOG2[self as usize]
    }

    pub fn height_mi(self) -> usize {
        self.height() >> MI_SIZE_LOG2
    }

    pub fn tx_size(self) -> TxSize {
        match self {
            BLOCK_4X4 => TX_4X4,
            BLOCK_4X8 => TX_4X8,
            BLOCK_8X4 => TX_8X4,
            BLOCK_8X8 => TX_8X8,
            BLOCK_8X16 => TX_8X16,
            BLOCK_16X8 => TX_16X8,
            BLOCK_16X16 => TX_16X16,
            BLOCK_16X32 => TX_16X32,
            BLOCK_32X16 => TX_32X16,
            BLOCK_32X32 => TX_32X32,
            BLOCK_32X64 => TX_32X64,
            BLOCK_64X32 => TX_64X32,
            BLOCK_4X16 => TX_4X16,
            BLOCK_16X4 => TX_16X4,
            BLOCK_8X32 => TX_8X32,
            BLOCK_32X8 => TX_32X8,
            BLOCK_16X64 => TX_16X64,
            BLOCK_64X16 => TX_64X16,
            BLOCK_INVALID => unreachable!(),
            _ => TX_64X64,
        }
    }

    pub fn largest_uv_tx_size(self, xdec: usize, ydec: usize) -> TxSize {
        let plane_bsize = get_plane_block_size(self, xdec, ydec);
        debug_assert!((plane_bsize as usize) < BlockSize::BLOCK_SIZES_ALL);
        let uv_tx = max_txsize_rect_lookup[plane_bsize as usize];

        av1_get_coded_tx_size(uv_tx)
    }

    pub fn is_sqr(self) -> bool {
        self.width_log2() == self.height_log2()
    }

    pub fn is_sub8x8(self, xdec: usize, ydec: usize) -> bool {
        xdec != 0 && self.width_log2() == 2 || ydec != 0 && self.height_log2() == 2
    }

    pub fn sub8x8_offset(self, xdec: usize, ydec: usize) -> (isize, isize) {
        let offset_x = if xdec != 0 && self.width_log2() == 2 {
            -1
        } else {
            0
        };
        let offset_y = if ydec != 0 && self.height_log2() == 2 {
            -1
        } else {
            0
        };

        (offset_x, offset_y)
    }

    pub fn greater_than(self, other: BlockSize) -> bool {
        (self.width() > other.width() && self.height() >= other.height())
            || (self.width() >= other.width() && self.height() > other.height())
    }

    pub fn gte(self, other: BlockSize) -> bool {
        self.greater_than(other)
            || (self.width() == other.width() && self.height() == other.height())
    }

    #[rustfmt::skip]
    const SUBSIZE_LOOKUP: [[BlockSize; BlockSize::BLOCK_SIZES_ALL];
        EXT_PARTITION_TYPES] = [
        // PARTITION_NONE
        [
            //                            4X4
            BLOCK_4X4,
            // 4X8,        8X4,           8X8
            BLOCK_4X8,     BLOCK_8X4,     BLOCK_8X8,
            // 8X16,       16X8,          16X16
            BLOCK_8X16,    BLOCK_16X8,    BLOCK_16X16,
            // 16X32,      32X16,         32X32
            BLOCK_16X32,   BLOCK_32X16,   BLOCK_32X32,
            // 32X64,      64X32,         64X64
            BLOCK_32X64,   BLOCK_64X32,   BLOCK_64X64,
            // 64x128,     128x64,        128x128
            BLOCK_64X128,  BLOCK_128X64,  BLOCK_128X128,
            // 4X16,       16X4,          8X32
            BLOCK_4X16,    BLOCK_16X4,    BLOCK_8X32,
            // 32X8,       16X64,         64X16
            BLOCK_32X8,    BLOCK_16X64,   BLOCK_64X16
        ],
        // PARTITION_HORZ
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X4,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X8,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X16,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X32,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_128X64,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ],
        // PARTITION_VERT
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_4X8,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X16,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X32,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X64,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X128,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ],
        // PARTITION_SPLIT
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_4X4,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X8,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X16,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X32,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X64,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ],
        // PARTITION_HORZ_A
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X4,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X8,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X16,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X32,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_128X64,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
        ],
        // PARTITION_HORZ_B
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X4,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X8,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X16,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X32,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_128X64,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ],
        // PARTITION_VERT_A
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_4X8,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X16,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X32,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X64,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X128,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ],
        // PARTITION_VERT_B
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_4X8,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X16,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X32,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X64,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X128,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ],
        // PARTITION_HORZ_4
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X4,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_32X8,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_64X16,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ],
        // PARTITION_VERT_4
        [
            //                            4X4
            BLOCK_INVALID,
            // 4X8,        8X4,           8X8
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 8X16,       16X8,          16X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_4X16,
            // 16X32,      32X16,         32X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_8X32,
            // 32X64,      64X32,         64X64
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_16X64,
            // 64x128,     128x64,        128x128
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 4X16,       16X4,          8X32
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID,
            // 32X8,       16X64,         64X16
            BLOCK_INVALID, BLOCK_INVALID, BLOCK_INVALID
        ]
    ];

    pub fn subsize(self, partition: PartitionType) -> BlockSize {
        BlockSize::SUBSIZE_LOOKUP[partition as usize][self as usize]
    }

    pub fn is_rect_tx_allowed(self) -> bool {
        static LUT: [u8; BlockSize::BLOCK_SIZES_ALL] = [
            0, // BLOCK_4X4
            1, // BLOCK_4X8
            1, // BLOCK_8X4
            0, // BLOCK_8X8
            1, // BLOCK_8X16
            1, // BLOCK_16X8
            0, // BLOCK_16X16
            1, // BLOCK_16X32
            1, // BLOCK_32X16
            0, // BLOCK_32X32
            1, // BLOCK_32X64
            1, // BLOCK_64X32
            0, // BLOCK_64X64
            0, // BLOCK_64X128
            0, // BLOCK_128X64
            0, // BLOCK_128X128
            1, // BLOCK_4X16
            1, // BLOCK_16X4
            1, // BLOCK_8X32
            1, // BLOCK_32X8
            1, // BLOCK_16X64
            1, // BLOCK_64X16
        ];

        LUT[self as usize] == 1
    }
}

/// Transform Size
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(C)]
pub enum TxSize {
    TX_4X4,
    TX_8X8,
    TX_16X16,
    TX_32X32,
    TX_64X64,

    TX_4X8,
    TX_8X4,
    TX_8X16,
    TX_16X8,
    TX_16X32,
    TX_32X16,
    TX_32X64,
    TX_64X32,

    TX_4X16,
    TX_16X4,
    TX_8X32,
    TX_32X8,
    TX_16X64,
    TX_64X16,
}

impl TxSize {
    /// Number of square transform sizes
    pub const TX_SIZES: usize = 5;

    /// Number of transform sizes (including non-square sizes)
    pub const TX_SIZES_ALL: usize = 14 + 5;

    pub fn width(self) -> usize {
        1 << self.width_log2()
    }

    pub fn width_log2(self) -> usize {
        match self {
            TX_4X4 | TX_4X8 | TX_4X16 => 2,
            TX_8X8 | TX_8X4 | TX_8X16 | TX_8X32 => 3,
            TX_16X16 | TX_16X8 | TX_16X32 | TX_16X4 | TX_16X64 => 4,
            TX_32X32 | TX_32X16 | TX_32X64 | TX_32X8 => 5,
            TX_64X64 | TX_64X32 | TX_64X16 => 6,
        }
    }

    pub fn width_index(self) -> usize {
        self.width_log2() - TX_4X4.width_log2()
    }

    pub fn height(self) -> usize {
        1 << self.height_log2()
    }

    pub fn height_log2(self) -> usize {
        match self {
            TX_4X4 | TX_8X4 | TX_16X4 => 2,
            TX_8X8 | TX_4X8 | TX_16X8 | TX_32X8 => 3,
            TX_16X16 | TX_8X16 | TX_32X16 | TX_4X16 | TX_64X16 => 4,
            TX_32X32 | TX_16X32 | TX_64X32 | TX_8X32 => 5,
            TX_64X64 | TX_32X64 | TX_16X64 => 6,
        }
    }

    pub fn height_index(self) -> usize {
        self.height_log2() - TX_4X4.height_log2()
    }

    pub fn width_mi(self) -> usize {
        self.width() >> MI_SIZE_LOG2
    }

    pub fn area(self) -> usize {
        1 << self.area_log2()
    }

    pub fn area_log2(self) -> usize {
        self.width_log2() + self.height_log2()
    }

    pub fn height_mi(self) -> usize {
        self.height() >> MI_SIZE_LOG2
    }

    pub fn block_size(self) -> BlockSize {
        match self {
            TX_4X4 => BLOCK_4X4,
            TX_8X8 => BLOCK_8X8,
            TX_16X16 => BLOCK_16X16,
            TX_32X32 => BLOCK_32X32,
            TX_64X64 => BLOCK_64X64,
            TX_4X8 => BLOCK_4X8,
            TX_8X4 => BLOCK_8X4,
            TX_8X16 => BLOCK_8X16,
            TX_16X8 => BLOCK_16X8,
            TX_16X32 => BLOCK_16X32,
            TX_32X16 => BLOCK_32X16,
            TX_32X64 => BLOCK_32X64,
            TX_64X32 => BLOCK_64X32,
            TX_4X16 => BLOCK_4X16,
            TX_16X4 => BLOCK_16X4,
            TX_8X32 => BLOCK_8X32,
            TX_32X8 => BLOCK_32X8,
            TX_16X64 => BLOCK_16X64,
            TX_64X16 => BLOCK_64X16,
        }
    }

    pub fn sqr(self) -> TxSize {
        match self {
            TX_4X4 | TX_4X8 | TX_8X4 | TX_4X16 | TX_16X4 => TX_4X4,
            TX_8X8 | TX_8X16 | TX_16X8 | TX_8X32 | TX_32X8 => TX_8X8,
            TX_16X16 | TX_16X32 | TX_32X16 | TX_16X64 | TX_64X16 => TX_16X16,
            TX_32X32 | TX_32X64 | TX_64X32 => TX_32X32,
            TX_64X64 => TX_64X64,
        }
    }

    pub fn sqr_up(self) -> TxSize {
        match self {
            TX_4X4 => TX_4X4,
            TX_8X8 | TX_4X8 | TX_8X4 => TX_8X8,
            TX_16X16 | TX_8X16 | TX_16X8 | TX_4X16 | TX_16X4 => TX_16X16,
            TX_32X32 | TX_16X32 | TX_32X16 | TX_8X32 | TX_32X8 => TX_32X32,
            TX_64X64 | TX_32X64 | TX_64X32 | TX_16X64 | TX_64X16 => TX_64X64,
        }
    }

    pub fn by_dims(w: usize, h: usize) -> TxSize {
        match (w, h) {
            (4, 4) => TX_4X4,
            (8, 8) => TX_8X8,
            (16, 16) => TX_16X16,
            (32, 32) => TX_32X32,
            (64, 64) => TX_64X64,
            (4, 8) => TX_4X8,
            (8, 4) => TX_8X4,
            (8, 16) => TX_8X16,
            (16, 8) => TX_16X8,
            (16, 32) => TX_16X32,
            (32, 16) => TX_32X16,
            (32, 64) => TX_32X64,
            (64, 32) => TX_64X32,
            (4, 16) => TX_4X16,
            (16, 4) => TX_16X4,
            (8, 32) => TX_8X32,
            (32, 8) => TX_32X8,
            (16, 64) => TX_16X64,
            (64, 16) => TX_64X16,
            _ => unreachable!(),
        }
    }

    pub fn is_rect(self) -> bool {
        self.width_log2() != self.height_log2()
    }
}

pub const TX_TYPES: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub enum TxType {
    DCT_DCT = 0,   // DCT  in both horizontal and vertical
    ADST_DCT = 1,  // ADST in vertical, DCT in horizontal
    DCT_ADST = 2,  // DCT  in vertical, ADST in horizontal
    ADST_ADST = 3, // ADST in both directions
    FLIPADST_DCT = 4,
    DCT_FLIPADST = 5,
    FLIPADST_FLIPADST = 6,
    ADST_FLIPADST = 7,
    FLIPADST_ADST = 8,
    IDTX = 9,
    V_DCT = 10,
    H_DCT = 11,
    V_ADST = 12,
    H_ADST = 13,
    V_FLIPADST = 14,
    H_FLIPADST = 15,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PredictionMode {
    DC_PRED,     // Average of above and left pixels
    V_PRED,      // Vertical
    H_PRED,      // Horizontal
    D45_PRED,    // Directional 45  deg = round(arctan(1/1) * 180/pi)
    D135_PRED,   // Directional 135 deg = 180 - 45
    D117_PRED,   // Directional 117 deg = 180 - 63
    D153_PRED,   // Directional 153 deg = 180 - 27
    D207_PRED,   // Directional 207 deg = 180 + 27
    D63_PRED,    // Directional 63  deg = round(arctan(2/1) * 180/pi)
    SMOOTH_PRED, // Combination of horizontal and vertical interpolation
    SMOOTH_V_PRED,
    SMOOTH_H_PRED,
    PAETH_PRED,
    UV_CFL_PRED,
    NEARESTMV,
    NEAR0MV,
    NEAR1MV,
    NEAR2MV,
    GLOBALMV,
    NEWMV,
    // Compound ref compound modes
    NEAREST_NEARESTMV,
    NEAR_NEARMV,
    NEAREST_NEWMV,
    NEW_NEARESTMV,
    NEAR_NEWMV,
    NEW_NEARMV,
    GLOBAL_GLOBALMV,
    NEW_NEWMV
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum TxSet {
    // DCT only
    TX_SET_DCTONLY,
    // DCT + Identity only
    TX_SET_DCT_IDTX,
    // Discrete Trig transforms w/o flip (4) + Identity (1)
    TX_SET_DTT4_IDTX,
    // Discrete Trig transforms w/o flip (4) + Identity (1) + 1D Hor/vert DCT (2)
    // for 16x16 only
    TX_SET_DTT4_IDTX_1DDCT_16X16,
    // Discrete Trig transforms w/o flip (4) + Identity (1) + 1D Hor/vert DCT (2)
    TX_SET_DTT4_IDTX_1DDCT,
    // Discrete Trig transforms w/ flip (9) + Identity (1)
    TX_SET_DTT9_IDTX,
    // Discrete Trig transforms w/ flip (9) + Identity (1) + 1D Hor/Ver DCT (2)
    TX_SET_DTT9_IDTX_1DDCT,
    // Discrete Trig transforms w/ flip (9) + Identity (1) + 1D Hor/Ver (6)
    // for 16x16 only
    TX_SET_ALL16_16X16,
    // Discrete Trig transforms w/ flip (9) + Identity (1) + 1D Hor/Ver (6)
    TX_SET_ALL16
}



const SUPERBLOCK_TO_PLANE_SHIFT: usize = MAX_SB_SIZE_LOG2;
const SUPERBLOCK_TO_BLOCK_SHIFT: usize = MAX_MIB_SIZE_LOG2;
pub const BLOCK_TO_PLANE_SHIFT: usize = MI_SIZE_LOG2;
pub const LOCAL_BLOCK_MASK: usize = (1 << SUPERBLOCK_TO_BLOCK_SHIFT) - 1;

/// Absolute offset in superblocks inside a plane, where a superblock is defined
/// to be an N*N square where N = (1 << SUPERBLOCK_TO_PLANE_SHIFT).
#[derive(Clone, Copy, Debug)]
pub struct SuperBlockOffset {
    pub x: usize,
    pub y: usize
}

impl SuperBlockOffset {
    /// Offset of a block inside the current superblock.
    pub fn block_offset(self, block_x: usize, block_y: usize) -> BlockOffset {
        BlockOffset {
            x: (self.x << SUPERBLOCK_TO_BLOCK_SHIFT) + block_x,
            y: (self.y << SUPERBLOCK_TO_BLOCK_SHIFT) + block_y
        }
    }

    /// Offset of the top-left pixel of this block.
    pub fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
        PlaneOffset {
            x: (self.x as isize) << (SUPERBLOCK_TO_PLANE_SHIFT - plane.xdec),
            y: (self.y as isize) << (SUPERBLOCK_TO_PLANE_SHIFT - plane.ydec)
        }
    }
}

/// Absolute offset in blocks inside a plane, where a block is defined
/// to be an N*N square where N = (1 << BLOCK_TO_PLANE_SHIFT).
#[derive(Clone, Copy, Debug)]
pub struct BlockOffset {
    pub x: usize,
    pub y: usize
}

impl BlockOffset {
    /// Offset of the superblock in which this block is located.
    pub fn sb_offset(self) -> SuperBlockOffset {
        SuperBlockOffset {
            x: self.x >> SUPERBLOCK_TO_BLOCK_SHIFT,
            y: self.y >> SUPERBLOCK_TO_BLOCK_SHIFT
        }
    }

    /// Offset of the top-left pixel of this block.
    pub fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
        PlaneOffset {
            x: (self.x >> plane.xdec << BLOCK_TO_PLANE_SHIFT) as isize,
            y: (self.y >> plane.ydec << BLOCK_TO_PLANE_SHIFT) as isize,
        }
    }

    /// Convert to plane offset without decimation
    #[inline]
    pub fn to_luma_plane_offset(self) -> PlaneOffset {
        PlaneOffset {
            x: (self.x as isize) << BLOCK_TO_PLANE_SHIFT,
            y: (self.y as isize) << BLOCK_TO_PLANE_SHIFT,
        }
    }

    pub fn y_in_sb(self) -> usize {
        self.y % MAX_MIB_SIZE
    }

    pub fn with_offset(self, col_offset: isize, row_offset: isize) -> BlockOffset {
        let x = self.x as isize + col_offset;
        let y = self.y as isize + row_offset;
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);

        BlockOffset {
            x: x as usize,
            y: y as usize
        }
    }
}

pub fn av1_get_coded_tx_size(tx_size: TxSize) -> TxSize {
    if tx_size == TX_64X64 || tx_size == TX_64X32 || tx_size == TX_32X64 {
        return TX_32X32
    }
    if tx_size == TX_16X64 {
        return TX_16X32
    }
    if tx_size == TX_64X16 {
        return TX_32X16
    }

    tx_size
}
