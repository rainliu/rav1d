use std::fmt;

use num_derive::*;

// Constants from Section 3. "Symbols and abbreviated terms"
pub const MAX_CDEF_STRENGTHS: usize = 8;
pub const MAX_OPERATING_POINTS: usize = 32;
pub const MAX_TILE_COLS: usize = 64;
pub const MAX_TILE_ROWS: usize = 64;
pub const MAX_SEGMENTS: usize = 8;
pub const NUM_REF_FRAMES: usize = 8;
pub const PRIMARY_REF_NONE: usize = 7;
pub const REFS_PER_FRAME: usize = 7;
pub const TOTAL_REFS_PER_FRAME: usize = (REFS_PER_FRAME + 1);

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum TxfmMode {
    TX_4X4_ONLY,
    TX_LARGEST,
    TX_SWITCHABLE,
    N_TX_MODES,
}

impl Default for TxfmMode {
    fn default() -> Self {
        TxfmMode::TX_4X4_ONLY
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum FilterMode {
    FILTER_8TAP_REGULAR,
    FILTER_8TAP_SMOOTH,
    FILTER_8TAP_SHARP,
    N_SWITCHABLE_FILTERS,
    //FILTER_BILINEAR = N_SWITCHABLE_FILTERS,
    N_FILTERS,
    //FILTER_SWITCHABLE = N_FILTERS,
}

impl Default for FilterMode {
    fn default() -> Self {
        FilterMode::FILTER_8TAP_REGULAR
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum AdaptiveBoolean {
    OFF = 0,
    ON = 1,
    ADAPTIVE = 2,
}

impl Default for AdaptiveBoolean {
    fn default() -> Self {
        AdaptiveBoolean::OFF
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum RestorationType {
    RESTORATION_NONE,
    RESTORATION_SWITCHABLE,
    RESTORATION_WIENER,
    RESTORATION_SGRPROJ,
}

impl Default for RestorationType {
    fn default() -> Self {
        RestorationType::RESTORATION_NONE
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum WarpedMotionType {
    WM_TYPE_IDENTITY,
    WM_TYPE_TRANSLATION,
    WM_TYPE_ROT_ZOOM,
    WM_TYPE_AFFINE,
}

impl Default for WarpedMotionType {
    fn default() -> Self {
        WarpedMotionType::WM_TYPE_IDENTITY
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct WarpedMotionParamsStruct {
    alpha: i16,
    beta: i16,
    gamma: i16,
    delta: i16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum WarpedMotionParamsUnion {
    Abgd(WarpedMotionParamsStruct),
    Abcd([i16; 4]),
}

impl Default for WarpedMotionParamsUnion {
    fn default() -> Self {
        WarpedMotionParamsUnion::Abgd(WarpedMotionParamsStruct {
            ..Default::default()
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct WarpedMotionParams {
    t: WarpedMotionType,
    matrix: [i32; 6],
    u: WarpedMotionParamsUnion,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum PixelLayout {
    PIXEL_LAYOUT_I400, // monochrome
    PIXEL_LAYOUT_I420, // 4:2:0 planar
    PIXEL_LAYOUT_I422, // 4:2:2 planar
    PIXEL_LAYOUT_I444, // 4:4:4 planar
}

impl Default for PixelLayout {
    fn default() -> Self {
        PixelLayout::PIXEL_LAYOUT_I400
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum FrameType {
    FRAME_TYPE_KEY = 0,    // Key Intra frame
    FRAME_TYPE_INTER = 1,  // Inter frame
    FRAME_TYPE_INTRA = 2,  // Non key Intra frame
    FRAME_TYPE_SWITCH = 3, // Switch Inter frame
}

impl Default for FrameType {
    fn default() -> Self {
        FrameType::FRAME_TYPE_KEY
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum ColorPrimaries {
    COLOR_PRI_BT709 = 1,
    COLOR_PRI_UNKNOWN = 2,
    COLOR_PRI_BT470M = 4,
    COLOR_PRI_BT470BG = 5,
    COLOR_PRI_BT601 = 6,
    COLOR_PRI_SMPTE240 = 7,
    COLOR_PRI_FILM = 8,
    COLOR_PRI_BT2020 = 9,
    COLOR_PRI_XYZ = 10,
    COLOR_PRI_SMPTE431 = 11,
    COLOR_PRI_SMPTE432 = 12,
    COLOR_PRI_EBU3213 = 22,
}

impl Default for ColorPrimaries {
    fn default() -> Self {
        ColorPrimaries::COLOR_PRI_BT709
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum TransferCharacteristics {
    TRC_BT709 = 1,
    TRC_UNKNOWN = 2,
    TRC_BT470M = 4,
    TRC_BT470BG = 5,
    TRC_BT601 = 6,
    TRC_SMPTE240 = 7,
    TRC_LINEAR = 8,
    TRC_LOG100 = 9,         // logarithmic (100:1 range)
    TRC_LOG100_SQRT10 = 10, // lograithmic (100*sqrt(10):1 range)
    TRC_IEC61966 = 11,
    TRC_BT1361 = 12,
    TRC_SRGB = 13,
    TRC_BT2020_10BIT = 14,
    TRC_BT2020_12BIT = 15,
    TRC_SMPTE2084 = 16, // PQ
    TRC_SMPTE428 = 17,
    TRC_HLG = 18, // hybrid log/gamma (BT.2100 / ARIB STD-B67)
}

impl Default for TransferCharacteristics {
    fn default() -> Self {
        TransferCharacteristics::TRC_BT709
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum MatrixCoefficients {
    MC_IDENTITY = 0,
    MC_BT709 = 1,
    MC_UNKNOWN = 2,
    MC_FCC = 4,
    MC_BT470BG = 5,
    MC_BT601 = 6,
    MC_SMPTE240 = 7,
    MC_SMPTE_YCGCO = 8,
    MC_BT2020_NCL = 9,
    MC_BT2020_CL = 10,
    MC_SMPTE2085 = 11,
    MC_CHROMAT_NCL = 12, // Chromaticity-derived
    MC_CHROMAT_CL = 13,
    MC_ICTCP = 14,
}

impl Default for MatrixCoefficients {
    fn default() -> Self {
        MatrixCoefficients::MC_IDENTITY
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum ChromaSamplePosition {
    CHR_UNKNOWN = 0,
    CHR_VERTICAL = 1, // Horizontally co-located with luma(0, 0)
    // sample, between two vertical samples
    CHR_COLOCATED = 2, // Co-located with luma(0, 0) sample
}

impl Default for ChromaSamplePosition {
    fn default() -> Self {
        ChromaSamplePosition::CHR_UNKNOWN
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct ContentLightLevel {
    max_content_light_level: isize,
    max_frame_average_light_level: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct MasteringDisplay {
    // 0.16 fixed point
    primaries: [[u16; 3]; 2], //TODO: confirm [3][2]?
    // 0.16 fixed point
    white_point: [u16; 2],
    // 24.8 fixed point
    max_luminance: u32,
    // 18.14 fixed point
    min_luminance: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SequenceHeaderOperatingPoint {
    major_level: isize,
    minor_level: isize,
    initial_display_delay: isize,
    idc: isize,
    tier: isize,
    decoder_model_param_present: isize,
    display_model_param_present: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SequenceHeaderOperatingParameterInfo {
    decoder_buffer_delay: isize,
    encoder_buffer_delay: isize,
    low_delay_mode: isize,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct SequenceHeader {
    /**
     * Stream profile, 0 for 8-10 bits/component 4:2:0 or monochrome;
     * 1 for 8-10 bits/component 4:4:4; 2 for 4:2:2 at any bits/component,
     * or 12 bits/component at any chroma subsampling.
     */
    pub(crate) profile: u32,
    /**
     * Maximum dimensions for this stream. In non-scalable streams, these
     * are often the actual dimensions of the stream, although that is not
     * a normative requirement.
     */
    pub(crate) max_width: u32,
    pub(crate) max_height: u32,
    pub(crate) layout: PixelLayout,          // format of the picture
    pub(crate) pri: ColorPrimaries,          // color primaries (av1)
    pub(crate) trc: TransferCharacteristics, // transfer characteristics (av1)
    pub(crate) mtrx: MatrixCoefficients,     // matrix coefficients (av1)
    pub(crate) chr: ChromaSamplePosition,    // chroma sample position (av1)
    /**
     * 0, 1 and 2 mean 8, 10 or 12 bits/component, respectively. This is not
     * exactly the same as 'hbd' from the spec; the spec's hbd distinguishes
     * between 8 (0) and 10-12 (1) bits/component, and another element
     * (twelve_bit) to distinguish between 10 and 12 bits/component. To get
     * the spec's hbd, use !!our_hbd, and to get twelve_bit, use hbd == 2.
     */
    pub(crate) hbd: u32,
    /**
     * Pixel data uses JPEG pixel range ([0,255] for 8bits) instead of
     * MPEG pixel range ([16,235] for 8bits luma, [16,240] for 8bits chroma).
     */
    pub(crate) color_range: u32,

    pub(crate) num_operating_points: usize,
    pub(crate) operating_points: [SequenceHeaderOperatingPoint; MAX_OPERATING_POINTS],

    pub(crate) still_picture: bool,
    pub(crate) reduced_still_picture_header: bool,
    pub(crate) timing_info_present: isize,
    pub(crate) num_units_in_tick: isize,
    pub(crate) time_scale: isize,
    pub(crate) equal_picture_interval: isize,
    pub(crate) num_ticks_per_picture: usize,
    pub(crate) decoder_model_info_present: isize,
    pub(crate) encoder_decoder_buffer_delay_length: isize,
    pub(crate) num_units_in_decoding_tick: isize,
    pub(crate) buffer_removal_delay_length: isize,
    pub(crate) frame_presentation_delay_length: isize,
    pub(crate) display_model_info_present: isize,
    pub(crate) width_n_bits: isize,
    pub(crate) height_n_bits: isize,
    pub(crate) frame_id_numbers_present: isize,
    pub(crate) delta_frame_id_n_bits: isize,
    pub(crate) frame_id_n_bits: isize,
    pub(crate) sb128: isize,
    pub(crate) filter_intra: isize,
    pub(crate) intra_edge_filter: isize,
    pub(crate) inter_intra: isize,
    pub(crate) masked_compound: isize,
    pub(crate) warped_motion: isize,
    pub(crate) dual_filter: isize,
    pub(crate) order_hint: isize,
    pub(crate) jnt_comp: isize,
    pub(crate) ref_frame_mvs: isize,
    pub(crate) screen_content_tools: AdaptiveBoolean,
    pub(crate) force_integer_mv: AdaptiveBoolean,
    pub(crate) order_hint_n_bits: isize,
    pub(crate) super_res: isize,
    pub(crate) cdef: isize,
    pub(crate) restoration: isize,
    pub(crate) ss_hor: isize,
    pub(crate) ss_ver: isize,
    pub(crate) monochrome: isize,
    pub(crate) color_description_present: isize,
    pub(crate) separate_uv_delta_q: isize,
    pub(crate) film_grain_present: isize,

    // SequenceHeaders of the same sequence are required to be
    // bit-identical until this offset. See 7.5 "Ordering of OBUs":
    //   Within a particular coded video sequence, the contents of
    //   sequence_header_obu must be bit-identical each time the
    //   sequence header appears except for the contents of
    //   operating_parameters_info.
    pub(crate) operating_parameter_info: [SequenceHeaderOperatingParameterInfo; MAX_OPERATING_POINTS],
}

impl SequenceHeader {
    pub fn new() -> Self {
        SequenceHeader {
            ..Default::default()
        }
    }
}

impl PartialEq for SequenceHeader {
    fn eq(&self, other: &Self) -> bool {
        self.profile == other.profile
            && self.max_width == other.max_width
            && self.max_height == other.max_height
            && self.layout == other.layout
            && self.pri == other.pri
            && self.trc == other.trc
            && self.mtrx == other.mtrx
            && self.chr == other.chr
            && self.hbd == other.hbd
            && self.color_range == other.color_range
            && self.num_operating_points == other.num_operating_points
            && self.operating_points == other.operating_points
            && self.still_picture == other.still_picture
            && self.reduced_still_picture_header == other.reduced_still_picture_header
            && self.timing_info_present == other.timing_info_present
            && self.num_units_in_tick == other.num_units_in_tick
            && self.time_scale == other.time_scale
            && self.equal_picture_interval == other.equal_picture_interval
            && self.num_ticks_per_picture == other.num_ticks_per_picture
            && self.decoder_model_info_present == other.decoder_model_info_present
            && self.encoder_decoder_buffer_delay_length == other.encoder_decoder_buffer_delay_length
            && self.num_units_in_decoding_tick == other.num_units_in_decoding_tick
            && self.buffer_removal_delay_length == other.buffer_removal_delay_length
            && self.frame_presentation_delay_length == other.frame_presentation_delay_length
            && self.display_model_info_present == other.display_model_info_present
            && self.width_n_bits == other.width_n_bits
            && self.height_n_bits == other.height_n_bits
            && self.frame_id_numbers_present == other.frame_id_numbers_present
            && self.delta_frame_id_n_bits == other.delta_frame_id_n_bits
            && self.frame_id_n_bits == other.frame_id_n_bits
            && self.sb128 == other.sb128
            && self.filter_intra == other.filter_intra
            && self.intra_edge_filter == other.intra_edge_filter
            && self.inter_intra == other.inter_intra
            && self.masked_compound == other.masked_compound
            && self.warped_motion == other.warped_motion
            && self.dual_filter == other.dual_filter
            && self.order_hint == other.order_hint
            && self.jnt_comp == other.jnt_comp
            && self.ref_frame_mvs == other.ref_frame_mvs
            && self.screen_content_tools == other.screen_content_tools
            && self.force_integer_mv == other.force_integer_mv
            && self.order_hint_n_bits == other.order_hint_n_bits
            && self.super_res == other.super_res
            && self.cdef == other.cdef
            && self.restoration == other.restoration
            && self.ss_hor == other.ss_hor
            && self.ss_ver == other.ss_ver
            && self.monochrome == other.monochrome
            && self.color_description_present == other.color_description_present
            && self.separate_uv_delta_q == other.separate_uv_delta_q
            && self.film_grain_present == other.film_grain_present

        // SequenceHeaders of the same sequence are required to be
        // bit-identical until this offset. See 7.5 "Ordering of OBUs":
        //   Within a particular coded video sequence, the contents of
        //   sequence_header_obu must be bit-identical each time the
        //   sequence header appears except for the contents of
        //   operating_parameters_info.
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SegmentationData {
    delta_q: isize,
    delta_lf_y_v: isize,
    delta_lf_y_h: isize,
    delta_lf_u: isize,
    delta_lf_v: isize,
    ref_frame: isize,
    skip: isize,
    globalmv: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SegmentationDataSet {
    d: [SegmentationData; MAX_SEGMENTS],
    preskip: isize,
    last_active_segid: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct LoopfilterModeRefDeltas {
    mode_delta: [isize; 2],
    ref_delta: [isize; TOTAL_REFS_PER_FRAME],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FilmGrainData {
    seed: u16,
    num_y_points: isize,
    y_points: [[u8; 14]; 2], //TODO: [14][2]
    chroma_scaling_from_luma: isize,
    num_uv_points: [isize; 2],
    uv_points: [[[u8; 2]; 10]; 2], //TODO: [2][10][2]
    scaling_shift: isize,
    ar_coeff_lag: isize,
    ar_coeffs_y: [i8; 24],
    ar_coeffs_uv: [[i8; 2]; 25], //TODO: [2][25]
    ar_coeff_shift: isize,
    grain_scale_shift: isize,
    uv_mult: [isize; 2],
    uv_luma_mult: [isize; 2],
    uv_offset: [isize; 2],
    overlap_flag: isize,
    clip_to_restricted_range: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FilmGrain {
    present: isize,
    update: isize,
    data: FilmGrainData,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FrameHeaderOperatingPoint {
    buffer_removal_time: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SuperResolution {
    width_scale_denominator: isize,
    enabled: isize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tiling {
    uniform: isize,
    n_bytes: usize,
    min_log2_cols: isize,
    max_log2_cols: isize,
    log2_cols: isize,
    cols: isize,
    min_log2_rows: isize,
    max_log2_rows: isize,
    log2_rows: isize,
    rows: isize,
    col_start_sb: [u16; MAX_TILE_COLS + 1],
    row_start_sb: [u16; MAX_TILE_ROWS + 1],
    update: isize,
}

impl Default for Tiling {
    fn default() -> Self {
        Tiling {
            uniform: 0,
            n_bytes: 0,
            min_log2_cols: 0,
            max_log2_cols: 0,
            log2_cols: 0,
            cols: 0,
            min_log2_rows: 0,
            max_log2_rows: 0,
            log2_rows: 0,
            rows: 0,
            col_start_sb: [0; MAX_TILE_COLS + 1],
            row_start_sb: [0; MAX_TILE_ROWS + 1],
            update: 0,
        }
    }
}

impl PartialEq for Tiling {
    fn eq(&self, other: &Self) -> bool {
        self.uniform == other.uniform
            && self.n_bytes == other.n_bytes
            && self.min_log2_cols == other.min_log2_cols
            && self.max_log2_cols == other.max_log2_cols
            && self.log2_cols == other.log2_cols
            && self.cols == other.cols
            && self.min_log2_rows == other.min_log2_rows
            && self.max_log2_rows == other.max_log2_rows
            && self.log2_rows == other.log2_rows
            && self.rows == other.rows
            && self.update == other.update
            && self
                .col_start_sb
                .iter()
                .zip(other.col_start_sb.iter())
                .all(|(a, b)| a == b)
            && self
                .row_start_sb
                .iter()
                .zip(other.row_start_sb.iter())
                .all(|(a, b)| a == b)
    }
}

impl fmt::Debug for Tiling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tiling - \
             uniform: {} \
             n_bytes: {} \
             min_log2_cols: {} \
             max_log2_cols: {} \
             log2_cols: {} \
             cols:  {} \
             min_log2_rows: {} \
             max_log2_rows: {} \
             log2_rows: {} \
             rows: {} \
             col_start_sb: {} \
             row_start_sb: {} \
             update: {}",
            self.uniform,
            self.n_bytes,
            self.min_log2_cols,
            self.max_log2_cols,
            self.log2_cols,
            self.cols,
            self.min_log2_rows,
            self.max_log2_rows,
            self.log2_rows,
            self.rows,
            self.col_start_sb.len(),
            self.row_start_sb.len(),
            self.update,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Quant {
    yac: isize,
    ydc_delta: isize,
    udc_delta: isize,
    uac_delta: isize,
    vdc_delta: isize,
    vac_delta: isize,
    qm: isize,
    qm_y: isize,
    qm_u: isize,
    qm_v: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Segmentation {
    enabled: isize,
    update_map: isize,
    temporal: isize,
    update_data: isize,
    seg_data: SegmentationDataSet,
    lossless: [isize; MAX_SEGMENTS],
    qidx: [isize; MAX_SEGMENTS],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Q {
    present: isize,
    res_log2: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct LF {
    present: isize,
    res_log2: isize,
    multi: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Delta {
    q: Q,
    lf: LF,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct LoopFilter {
    level_y: [isize; 2],
    level_u: isize,
    level_v: isize,
    mode_ref_delta_enabled: isize,
    mode_ref_delta_update: isize,
    mode_ref_deltas: LoopfilterModeRefDeltas,
    sharpness: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct CDEF {
    damping: isize,
    n_bits: isize,
    y_strength: [isize; MAX_CDEF_STRENGTHS],
    uv_strength: [isize; MAX_CDEF_STRENGTHS],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Restoration {
    t: [RestorationType; 3],
    unit_size: [isize; 2],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FrameHeader {
    frame_type: FrameType, // type of the picture
    width: [isize; 2],
    height: isize,
    frame_offset: isize,   // frame number
    film_grain: FilmGrain, // film grain parameters
    temporal_id: isize,
    spatial_id: isize, // spatial and temporal id of the frame for SVC
    show_existing_frame: isize,
    existing_frame_idx: isize,
    frame_id: isize,
    frame_presentation_delay: isize,
    show_frame: isize,
    showable_frame: isize,
    error_resilient_mode: isize,
    disable_cdf_update: isize,
    allow_screen_content_tools: isize,
    force_integer_mv: isize,
    frame_size_override: isize,
    primary_ref_frame: isize,
    buffer_removal_time_present: isize,
    operating_points: [FrameHeaderOperatingPoint; MAX_OPERATING_POINTS],
    refresh_frame_flags: isize,
    render_width: isize,
    render_height: isize,
    super_res: SuperResolution,
    have_render_size: isize,
    allow_intrabc: isize,
    frame_ref_short_signaling: isize,
    refidx: [isize; REFS_PER_FRAME],
    hp: isize,
    subpel_filter_mode: FilterMode,
    switchable_motion_mode: isize,
    use_ref_frame_mvs: isize,
    refresh_context: isize,
    tiling: Tiling,
    quant: Quant,
    segmentation: Segmentation,
    delta: Delta,
    all_lossless: isize,
    loopfilter: LoopFilter,
    cdef: CDEF,
    restoration: Restoration,
    txfm_mode: TxfmMode,
    switchable_comp_refs: isize,
    skip_mode_allowed: isize,
    skip_mode_enabled: isize,
    skip_mode_refs: [isize; 2],
    warp_motion: isize,
    reduced_txtp_set: isize,
    gmv: [WarpedMotionParams; REFS_PER_FRAME],
}

impl FrameHeader {
    pub fn new() -> Self {
        FrameHeader {
            ..Default::default()
        }
    }
}
