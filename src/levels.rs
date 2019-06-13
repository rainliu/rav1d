use crate::headers::*;

use arg_enum_proc_macro::ArgEnum;
use num_derive::*;

#[derive(ArgEnum, Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum ObuType {
    OBU_SEQ_HDR = 1,
    OBU_TD = 2,
    OBU_FRAME_HDR = 3,
    OBU_TILE_GRP = 4,
    OBU_METADATA = 5,
    OBU_FRAME = 6,
    OBU_REDUNDANT_FRAME_HDR = 7,
    OBU_PADDING = 15,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum ObuMetaType {
    OBU_META_HDR_CLL = 1,
    OBU_META_HDR_MDCV = 2,
    OBU_META_SCALABILITY = 3,
    OBU_META_ITUT_T35 = 4,
    OBU_META_TIMECODE = 5,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum TxfmSize {
    TX_4X4,
    TX_8X8,
    TX_16X16,
    TX_32X32,
    TX_64X64,
    N_TX_SIZES,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum BlockLevel {
    BL_128X128,
    BL_64X64,
    BL_32X32,
    BL_16X16,
    BL_8X8,
    N_BL_LEVELS,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum RectTxfmSize {
    RTX_4X8 = TxfmSize::N_TX_SIZES as isize,
    RTX_8X4,
    RTX_8X16,
    RTX_16X8,
    RTX_16X32,
    RTX_32X16,
    RTX_32X64,
    RTX_64X32,
    RTX_4X16,
    RTX_16X4,
    RTX_8X32,
    RTX_32X8,
    RTX_16X64,
    RTX_64X16,
    N_RECT_TX_SIZES,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum TxfmType {
    DCT_DCT,   // DCT  in both horizontal and vertical
    ADST_DCT,  // ADST in vertical, DCT in horizontal
    DCT_ADST,  // DCT  in vertical, ADST in horizontal
    ADST_ADST, // ADST in both directions
    FLIPADST_DCT,
    DCT_FLIPADST,
    FLIPADST_FLIPADST,
    ADST_FLIPADST,
    FLIPADST_ADST,
    IDTX,
    V_DCT,
    H_DCT,
    V_ADST,
    H_ADST,
    V_FLIPADST,
    H_FLIPADST,
    N_TX_TYPES_OR_WHT_WHT,
    N_TX_TYPES_PLUS_LL,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum TxfmTypeSet {
    TXTP_SET_DCT,
    TXTP_SET_DCT_ID,
    TXTP_SET_DT4_ID,
    TXTP_SET_DT4_ID_1D,
    TXTP_SET_DT9_ID_1D,
    TXTP_SET_ALL,
    TXTP_SET_LOSSLESS,
    N_TXTP_SETS,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum TxClass {
    TX_CLASS_2D,
    TX_CLASS_H,
    TX_CLASS_V,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum IntraPredMode {
    DC_PRED,
    VERT_PRED,
    HOR_PRED,
    DIAG_DOWN_LEFT_PRED,
    DIAG_DOWN_RIGHT_PRED,
    VERT_RIGHT_PRED,
    HOR_DOWN_PRED,
    HOR_UP_PRED,
    VERT_LEFT_PRED,
    SMOOTH_PRED,
    SMOOTH_V_PRED,
    SMOOTH_H_PRED,
    PAETH_PRED,
    N_INTRA_PRED_MODES_OR_CFL_PRED_OR_FILTER_PRED,
    N_UV_INTRA_PRED_MODES_OR_N_IMPL_INTRA_PRED_MODES,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum IntraPredModeExt {
    LEFT_DC_PRED = IntraPredMode::DIAG_DOWN_LEFT_PRED as isize,
    TOP_DC_PRED,
    DC_128_PRED,
    Z1_PRED,
    Z2_PRED,
    Z3_PRED,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum InterIntraPredMode {
    II_DC_PRED,
    II_VERT_PRED,
    II_HOR_PRED,
    II_SMOOTH_PRED,
    N_INTER_INTRA_PRED_MODES,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum BlockPartition {
    PARTITION_NONE,           // [ ] <-.
    PARTITION_H,              // [-]   |
    PARTITION_V,              // [|]   |
    PARTITION_SPLIT,          // [+] --'
    PARTITION_T_TOP_SPLIT_OR_N_SUB8X8_PARTITIONS,    // [⊥] i.e. split top, H bottom
    PARTITION_T_BOTTOM_SPLIT, // [т] i.e. H top, split bottom
    PARTITION_T_LEFT_SPLIT,   // [-|] i.e. split left, V right
    PARTITION_T_RIGHT_SPLIT,  // [|-] i.e. V left, split right
    PARTITION_H4,             // [Ⲷ]
    PARTITION_V4,             // [Ⲽ]
    N_PARTITIONS,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum BlockSize {
    BS_128x128,
    BS_128x64,
    BS_64x128,
    BS_64x64,
    BS_64x32,
    BS_64x16,
    BS_32x64,
    BS_32x32,
    BS_32x16,
    BS_32x8,
    BS_16x64,
    BS_16x32,
    BS_16x16,
    BS_16x8,
    BS_16x4,
    BS_8x32,
    BS_8x16,
    BS_8x8,
    BS_8x4,
    BS_4x16,
    BS_4x8,
    BS_4x4,
    N_BS_SIZES,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum Filter2d {
    // order is horizontal, vertical
    FILTER_2D_8TAP_REGULAR,
    FILTER_2D_8TAP_REGULAR_SMOOTH,
    FILTER_2D_8TAP_REGULAR_SHARP,
    FILTER_2D_8TAP_SHARP_REGULAR,
    FILTER_2D_8TAP_SHARP_SMOOTH,
    FILTER_2D_8TAP_SHARP,
    FILTER_2D_8TAP_SMOOTH_REGULAR,
    FILTER_2D_8TAP_SMOOTH,
    FILTER_2D_8TAP_SMOOTH_SHARP,
    FILTER_2D_BILINEAR,
    N_2D_FILTERS,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum MVJoint {
    MV_JOINT_ZERO,
    MV_JOINT_H,
    MV_JOINT_V,
    MV_JOINT_HV,
    N_MV_JOINTS,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum InterPredMode {
    NEARESTMV,
    NEARMV,
    GLOBALMV,
    NEWMV,
    N_INTER_PRED_MODES,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum CompInterPredMode {
    NEARESTMV_NEARESTMV,
    NEARMV_NEARMV,
    NEARESTMV_NEWMV,
    NEWMV_NEARESTMV,
    NEARMV_NEWMV,
    NEWMV_NEARMV,
    GLOBALMV_GLOBALMV,
    NEWMV_NEWMV,
    N_COMP_INTER_PRED_MODES,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum CompInterType {
    COMP_INTER_NONE,
    COMP_INTER_WEIGHTED_AVG,
    COMP_INTER_AVG,
    COMP_INTER_SEG,
    COMP_INTER_WEDGE,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum InterIntraType {
    INTER_INTRA_NONE,
    INTER_INTRA_BLEND,
    INTER_INTRA_WEDGE,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MV {
    y: i16,
    x: i16,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum MotionMode {
    MM_TRANSLATION,
    MM_OBMC,
    MM_WARP,
}

pub const QINDEX_RANGE: usize = 256;

pub struct Intra {
    y_mode: u8,
    uv_mode: u8,
    tx: u8,
    pal_sz: [u8; 2],
    y_angle: i8,
    uv_angle: i8,
    cfl_alpha: [i8; 2],
}

pub struct Inter {
    ref_frame: [i8; 2],
    comp_type: u8,
    wedge_idx: u8,
    mask_sign: u8,
    inter_mode: u8,
    drl_idx: u8,
    interintra_type: u8,
    interintra_mode: u8,
    motion_mode: u8,

    max_ytx: u8,
    filter2d: u8,
    tx_split: [u16; 2],
    mv: [MV; 2],
}

pub enum IntraInter {
    Intra(Intra),
    Inter(Inter),
}

pub struct Av1Block {
    bl: u8,
    bs: u8,
    bp: u8,
    intra: u8,
    seg_id: u8,
    skip_mode: u8,
    skip: u8,
    uvtx: u8,
    intra_inter: IntraInter,
}
