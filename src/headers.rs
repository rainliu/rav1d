// Constants from Section 3. "Symbols and abbreviated terms"
pub const RAV1D_MAX_CDEF_STRENGTHS: usize = 8;
pub const RAV1D_MAX_OPERATING_POINTS: usize = 32;
pub const RAV1D_MAX_TILE_COLS: usize = 64;
pub const RAV1D_MAX_TILE_ROWS: usize = 64;
pub const RAV1D_MAX_SEGMENTS: usize = 8;
pub const RAV1D_NUM_REF_FRAMES: usize = 8;
pub const RAV1D_PRIMARY_REF_NONE: usize = 7;
pub const RAV1D_REFS_PER_FRAME: usize = 7;
pub const RAV1D_TOTAL_REFS_PER_FRAME: usize = (RAV1D_REFS_PER_FRAME + 1);

pub enum Rav1dTxfmMode {
    RAV1D_TX_4X4_ONLY,
    RAV1D_TX_LARGEST,
    RAV1D_TX_SWITCHABLE,
    RAV1D_N_TX_MODES,
}

pub enum Rav1dFilterMode {
    RAV1D_FILTER_8TAP_REGULAR,
    RAV1D_FILTER_8TAP_SMOOTH,
    RAV1D_FILTER_8TAP_SHARP,
    RAV1D_N_SWITCHABLE_FILTERS,
    //RAV1D_FILTER_BILINEAR = RAV1D_N_SWITCHABLE_FILTERS,
    RAV1D_N_FILTERS,
    //RAV1D_FILTER_SWITCHABLE = RAV1D_N_FILTERS,
}

pub enum Rav1dAdaptiveBoolean {
    RAV1D_OFF = 0,
    RAV1D_ON = 1,
    RAV1D_ADAPTIVE = 2,
}

pub enum Rav1dRestorationType {
    RAV1D_RESTORATION_NONE,
    RAV1D_RESTORATION_SWITCHABLE,
    RAV1D_RESTORATION_WIENER,
    RAV1D_RESTORATION_SGRPROJ,
}

pub enum Rav1dWarpedMotionType {
    RAV1D_WM_TYPE_IDENTITY,
    RAV1D_WM_TYPE_TRANSLATION,
    RAV1D_WM_TYPE_ROT_ZOOM,
    RAV1D_WM_TYPE_AFFINE,
}

pub struct Rav1dWarpedMotionParamsStruct {
    alpha: i16,
    beta: i16,
    gamma: i16,
    delta: i16,
}
pub enum Rav1dWarpedMotionParamsUnion {
    Abgd(Rav1dWarpedMotionParamsStruct),
    Abcd([i16; 4]),
}

pub struct Rav1dWarpedMotionParams {
    t: Rav1dWarpedMotionType,
    matrix: [i32; 6],
    u: Rav1dWarpedMotionParamsUnion,
}

pub enum Rav1dPixelLayout {
    RAV1D_PIXEL_LAYOUT_I400, // monochrome
    RAV1D_PIXEL_LAYOUT_I420, // 4:2:0 planar
    RAV1D_PIXEL_LAYOUT_I422, // 4:2:2 planar
    RAV1D_PIXEL_LAYOUT_I444, // 4:4:4 planar
}

pub enum Rav1dFrameType {
    RAV1D_FRAME_TYPE_KEY = 0,    // Key Intra frame
    RAV1D_FRAME_TYPE_INTER = 1,  // Inter frame
    RAV1D_FRAME_TYPE_INTRA = 2,  // Non key Intra frame
    RAV1D_FRAME_TYPE_SWITCH = 3, // Switch Inter frame
}

pub enum Rav1dColorPrimaries {
    RAV1D_COLOR_PRI_BT709 = 1,
    RAV1D_COLOR_PRI_UNKNOWN = 2,
    RAV1D_COLOR_PRI_BT470M = 4,
    RAV1D_COLOR_PRI_BT470BG = 5,
    RAV1D_COLOR_PRI_BT601 = 6,
    RAV1D_COLOR_PRI_SMPTE240 = 7,
    RAV1D_COLOR_PRI_FILM = 8,
    RAV1D_COLOR_PRI_BT2020 = 9,
    RAV1D_COLOR_PRI_XYZ = 10,
    RAV1D_COLOR_PRI_SMPTE431 = 11,
    RAV1D_COLOR_PRI_SMPTE432 = 12,
    RAV1D_COLOR_PRI_EBU3213 = 22,
}

pub enum Rav1dTransferCharacteristics {
    RAV1D_TRC_BT709 = 1,
    RAV1D_TRC_UNKNOWN = 2,
    RAV1D_TRC_BT470M = 4,
    RAV1D_TRC_BT470BG = 5,
    RAV1D_TRC_BT601 = 6,
    RAV1D_TRC_SMPTE240 = 7,
    RAV1D_TRC_LINEAR = 8,
    RAV1D_TRC_LOG100 = 9,         // logarithmic (100:1 range)
    RAV1D_TRC_LOG100_SQRT10 = 10, // lograithmic (100*sqrt(10):1 range)
    RAV1D_TRC_IEC61966 = 11,
    RAV1D_TRC_BT1361 = 12,
    RAV1D_TRC_SRGB = 13,
    RAV1D_TRC_BT2020_10BIT = 14,
    RAV1D_TRC_BT2020_12BIT = 15,
    RAV1D_TRC_SMPTE2084 = 16, // PQ
    RAV1D_TRC_SMPTE428 = 17,
    RAV1D_TRC_HLG = 18, // hybrid log/gamma (BT.2100 / ARIB STD-B67)
}

pub enum Rav1dMatrixCoefficients {
    RAV1D_MC_IDENTITY = 0,
    RAV1D_MC_BT709 = 1,
    RAV1D_MC_UNKNOWN = 2,
    RAV1D_MC_FCC = 4,
    RAV1D_MC_BT470BG = 5,
    RAV1D_MC_BT601 = 6,
    RAV1D_MC_SMPTE240 = 7,
    RAV1D_MC_SMPTE_YCGCO = 8,
    RAV1D_MC_BT2020_NCL = 9,
    RAV1D_MC_BT2020_CL = 10,
    RAV1D_MC_SMPTE2085 = 11,
    RAV1D_MC_CHROMAT_NCL = 12, // Chromaticity-derived
    RAV1D_MC_CHROMAT_CL = 13,
    RAV1D_MC_ICTCP = 14,
}

pub enum Rav1dChromaSamplePosition {
    RAV1D_CHR_UNKNOWN = 0,
    RAV1D_CHR_VERTICAL = 1, // Horizontally co-located with luma(0, 0)
    // sample, between two vertical samples
    RAV1D_CHR_COLOCATED = 2, // Co-located with luma(0, 0) sample
}

pub struct Rav1dContentLightLevel {
    max_content_light_level: isize,
    max_frame_average_light_level: isize,
}

pub struct Rav1dMasteringDisplay {
    // 0.16 fixed point
    primaries: [[u16; 3]; 2], //TODO: confirm [3][2]?
    // 0.16 fixed point
    white_point: [u16; 2],
    // 24.8 fixed point
    max_luminance: u32,
    // 18.14 fixed point
    min_luminance: u32,
}

pub struct Rav1dSequenceHeaderOperatingPoint {
    major_level: isize,
    minor_level: isize,
    initial_display_delay: isize,
    idc: isize,
    tier: isize,
    decoder_model_param_present: isize,
    display_model_param_present: isize,
}

pub struct Rav1dSequenceHeaderOperatingParameterInfo {
    decoder_buffer_delay: isize,
    encoder_buffer_delay: isize,
    low_delay_mode: isize,
}

pub struct Rav1dSequenceHeader {
    /**
     * Stream profile, 0 for 8-10 bits/component 4:2:0 or monochrome;
     * 1 for 8-10 bits/component 4:4:4; 2 for 4:2:2 at any bits/component,
     * or 12 bits/component at any chroma subsampling.
     */
    profile: isize,
    /**
     * Maximum dimensions for this stream. In non-scalable streams, these
     * are often the actual dimensions of the stream, although that is not
     * a normative requirement.
     */
    max_width: isize,
    max_height: isize,
    layout: Rav1dPixelLayout,          // format of the picture
    pri: Rav1dColorPrimaries,          // color primaries (av1)
    trc: Rav1dTransferCharacteristics, // transfer characteristics (av1)
    mtrx: Rav1dMatrixCoefficients,     // matrix coefficients (av1)
    chr: Rav1dChromaSamplePosition,    // chroma sample position (av1)
    /**
     * 0, 1 and 2 mean 8, 10 or 12 bits/component, respectively. This is not
     * exactly the same as 'hbd' from the spec; the spec's hbd distinguishes
     * between 8 (0) and 10-12 (1) bits/component, and another element
     * (twelve_bit) to distinguish between 10 and 12 bits/component. To get
     * the spec's hbd, use !!our_hbd, and to get twelve_bit, use hbd == 2.
     */
    hbd: isize,
    /**
     * Pixel data uses JPEG pixel range ([0,255] for 8bits) instead of
     * MPEG pixel range ([16,235] for 8bits luma, [16,240] for 8bits chroma).
     */
    color_range: isize,

    num_operating_points: usize,
    operating_points: [Rav1dSequenceHeaderOperatingPoint; RAV1D_MAX_OPERATING_POINTS],

    still_picture: isize,
    reduced_still_picture_header: isize,
    timing_info_present: isize,
    num_units_in_tick: isize,
    time_scale: isize,
    equal_picture_interval: isize,
    num_ticks_per_picture: usize,
    decoder_model_info_present: isize,
    encoder_decoder_buffer_delay_length: isize,
    num_units_in_decoding_tick: isize,
    buffer_removal_delay_length: isize,
    frame_presentation_delay_length: isize,
    display_model_info_present: isize,
    width_n_bits: isize,
    height_n_bits: isize,
    frame_id_numbers_present: isize,
    delta_frame_id_n_bits: isize,
    frame_id_n_bits: isize,
    sb128: isize,
    filter_intra: isize,
    intra_edge_filter: isize,
    inter_intra: isize,
    masked_compound: isize,
    warped_motion: isize,
    dual_filter: isize,
    order_hint: isize,
    jnt_comp: isize,
    ref_frame_mvs: isize,
    screen_content_tools: Rav1dAdaptiveBoolean,
    force_integer_mv: Rav1dAdaptiveBoolean,
    order_hint_n_bits: isize,
    super_res: isize,
    cdef: isize,
    restoration: isize,
    ss_hor: isize,
    ss_ver: isize,
    monochrome: isize,
    color_description_present: isize,
    separate_uv_delta_q: isize,
    film_grain_present: isize,

    // Rav1dSequenceHeaders of the same sequence are required to be
    // bit-identical until this offset. See 7.5 "Ordering of OBUs":
    //   Within a particular coded video sequence, the contents of
    //   sequence_header_obu must be bit-identical each time the
    //   sequence header appears except for the contents of
    //   operating_parameters_info.
    operating_parameter_info:
        [Rav1dSequenceHeaderOperatingParameterInfo; RAV1D_MAX_OPERATING_POINTS],
}

pub struct Rav1dSegmentationData {
    delta_q: isize,
    delta_lf_y_v: isize,
    delta_lf_y_h: isize,
    delta_lf_u: isize,
    delta_lf_v: isize,
    ref_frame: isize,
    skip: isize,
    globalmv: isize,
}

pub struct Rav1dSegmentationDataSet {
    d: [Rav1dSegmentationData; RAV1D_MAX_SEGMENTS],
    preskip: isize,
    last_active_segid: isize,
}

pub struct Rav1dLoopfilterModeRefDeltas {
    mode_delta: [isize; 2],
    ref_delta: [isize; RAV1D_TOTAL_REFS_PER_FRAME],
}

pub struct Rav1dFilmGrainData {
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

pub struct Rav1dFilmGrain {
    present: isize,
    update: isize,
    data: Rav1dFilmGrainData,
}

pub struct Rav1dFrameHeaderOperatingPoint {
    buffer_removal_time: isize,
}

pub struct Rav1dSuperResolution {
    width_scale_denominator: isize,
    enabled: isize,
}

pub struct Rav1dTiling {
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
    col_start_sb: [u16; RAV1D_MAX_TILE_COLS + 1],
    row_start_sb: [u16; RAV1D_MAX_TILE_ROWS + 1],
    update: isize,
}

pub struct Rav1dQuant {
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

pub struct Rav1dSegmentation {
    enabled: isize,
    update_map: isize,
    temporal: isize,
    update_data: isize,
    seg_data: Rav1dSegmentationDataSet,
    lossless: [isize; RAV1D_MAX_SEGMENTS],
    qidx: [isize; RAV1D_MAX_SEGMENTS],
}

pub struct Rav1dQ {
    present: isize,
    res_log2: isize,
}

pub struct Rav1dLF {
    present: isize,
    res_log2: isize,
    multi: isize,
}

pub struct Rav1dDelta {
    q: Rav1dQ,
    lf: Rav1dLF,
}

pub struct Rav1dLoopFilter {
    level_y: [isize; 2],
    level_u: isize,
    level_v: isize,
    mode_ref_delta_enabled: isize,
    mode_ref_delta_update: isize,
    mode_ref_deltas: Rav1dLoopfilterModeRefDeltas,
    sharpness: isize,
}

pub struct Rav1dCDEF {
    damping: isize,
    n_bits: isize,
    y_strength: [isize; RAV1D_MAX_CDEF_STRENGTHS],
    uv_strength: [isize; RAV1D_MAX_CDEF_STRENGTHS],
}

pub struct Rav1dRestoration {
    t: [Rav1dRestorationType; 3],
    unit_size: [isize; 2],
}

pub struct Rav1dFrameHeader {
    frame_type: Rav1dFrameType, // type of the picture
    width: [isize; 2],
    height: isize,
    frame_offset: isize,        // frame number
    film_grain: Rav1dFilmGrain, // film grain parameters
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
    operating_points: [Rav1dFrameHeaderOperatingPoint; RAV1D_MAX_OPERATING_POINTS],
    refresh_frame_flags: isize,
    render_width: isize,
    render_height: isize,
    super_res: Rav1dSuperResolution,
    have_render_size: isize,
    allow_intrabc: isize,
    frame_ref_short_signaling: isize,
    refidx: [isize; RAV1D_REFS_PER_FRAME],
    hp: isize,
    subpel_filter_mode: Rav1dFilterMode,
    switchable_motion_mode: isize,
    use_ref_frame_mvs: isize,
    refresh_context: isize,
    tiling: Rav1dTiling,
    quant: Rav1dQuant,
    segmentation: Rav1dSegmentation,
    delta: Rav1dDelta,
    all_lossless: isize,
    loopfilter: Rav1dLoopFilter,
    cdef: Rav1dCDEF,
    restoration: Rav1dRestoration,
    txfm_mode: Rav1dTxfmMode,
    switchable_comp_refs: isize,
    skip_mode_allowed: isize,
    skip_mode_enabled: isize,
    skip_mode_refs: [isize; 2],
    warp_motion: isize,
    reduced_txtp_set: isize,
    gmv: [Rav1dWarpedMotionParams; RAV1D_REFS_PER_FRAME],
}
