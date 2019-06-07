// Constants from Section 3. "Symbols and abbreviated terms"
const RAV1D_MAX_CDEF_STRENGTHS: usize = 8;
const RAV1D_MAX_OPERATING_POINTS: usize = 32;
const RAV1D_MAX_TILE_COLS: usize = 64;
const RAV1D_MAX_TILE_ROWS: usize = 64;
const RAV1D_MAX_SEGMENTS: usize = 8;
const RAV1D_NUM_REF_FRAMES: usize = 8;
const RAV1D_PRIMARY_REF_NONE: usize = 7;
const RAV1D_REFS_PER_FRAME: usize = 7;
const RAV1D_TOTAL_REFS_PER_FRAME: usize = (RAV1D_REFS_PER_FRAME + 1);

enum Rav1dTxfmMode {
    RAV1D_TX_4X4_ONLY,
    RAV1D_TX_LARGEST,
    RAV1D_TX_SWITCHABLE,
    RAV1D_N_TX_MODES,
}

enum Rav1dFilterMode {
    RAV1D_FILTER_8TAP_REGULAR,
    RAV1D_FILTER_8TAP_SMOOTH,
    RAV1D_FILTER_8TAP_SHARP,
    RAV1D_N_SWITCHABLE_FILTERS,
    //RAV1D_FILTER_BILINEAR = RAV1D_N_SWITCHABLE_FILTERS,
    RAV1D_N_FILTERS,
    //RAV1D_FILTER_SWITCHABLE = RAV1D_N_FILTERS,
}

enum Rav1dAdaptiveBoolean {
    RAV1D_OFF = 0,
    RAV1D_ON = 1,
    RAV1D_ADAPTIVE = 2,
}

enum Rav1dRestorationType {
    RAV1D_RESTORATION_NONE,
    RAV1D_RESTORATION_SWITCHABLE,
    RAV1D_RESTORATION_WIENER,
    RAV1D_RESTORATION_SGRPROJ,
}

enum Rav1dWarpedMotionType {
    RAV1D_WM_TYPE_IDENTITY,
    RAV1D_WM_TYPE_TRANSLATION,
    RAV1D_WM_TYPE_ROT_ZOOM,
    RAV1D_WM_TYPE_AFFINE,
}

struct Rav1dWarpedMotionParamsStruct {
    alpha: i16,
    beta: i16,
    gamma: i16,
    delta: i16,
}
enum Rav1dWarpedMotionParamsUnion {
    Abgd(Rav1dWarpedMotionParamsStruct),
    Abcd([i16; 4]),
}
struct Rav1dWarpedMotionParams {
    t: Rav1dWarpedMotionType,
    matrix: [i32; 6],
    u: Rav1dWarpedMotionParamsUnion,
}

enum Rav1dPixelLayout {
    RAV1D_PIXEL_LAYOUT_I400, // monochrome
    RAV1D_PIXEL_LAYOUT_I420, // 4:2:0 planar
    RAV1D_PIXEL_LAYOUT_I422, // 4:2:2 planar
    RAV1D_PIXEL_LAYOUT_I444, // 4:4:4 planar
}

enum Rav1dFrameType {
    RAV1D_FRAME_TYPE_KEY = 0,    // Key Intra frame
    RAV1D_FRAME_TYPE_INTER = 1,  // Inter frame
    RAV1D_FRAME_TYPE_INTRA = 2,  // Non key Intra frame
    RAV1D_FRAME_TYPE_SWITCH = 3, // Switch Inter frame
}

enum Rav1dColorPrimaries {
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

enum Rav1dTransferCharacteristics {
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

enum Rav1dMatrixCoefficients {
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

enum Rav1dChromaSamplePosition {
    RAV1D_CHR_UNKNOWN = 0,
    RAV1D_CHR_VERTICAL = 1, // Horizontally co-located with luma(0, 0)
    // sample, between two vertical samples
    RAV1D_CHR_COLOCATED = 2, // Co-located with luma(0, 0) sample
}

struct Rav1dContentLightLevel {
    max_content_light_level: isize,
    max_frame_average_light_level: isize,
}

struct Rav1dMasteringDisplay {
    ///< 0.16 fixed point
    primaries: [[u16; 3]; 2], //TODO: confirm [3][2]?
    ///< 0.16 fixed point
    white_point: [u16; 2],
    ///< 24.8 fixed point
    max_luminance: u32,
    ///< 18.14 fixed point
    min_luminance: u32,
}
/*
typedef struct Rav1dSequenceHeader {
    /**
     * Stream profile, 0 for 8-10 bits/component 4:2:0 or monochrome;
     * 1 for 8-10 bits/component 4:4:4; 2 for 4:2:2 at any bits/component,
     * or 12 bits/component at any chroma subsampling.
     */
    int profile;
    /**
     * Maximum dimensions for this stream. In non-scalable streams, these
     * are often the actual dimensions of the stream, although that is not
     * a normative requirement.
     */
    int max_width, max_height;
    enum Rav1dPixelLayout layout; ///< format of the picture
    enum Rav1dColorPrimaries pri; ///< color primaries (av1)
    enum Rav1dTransferCharacteristics trc; ///< transfer characteristics (av1)
    enum Rav1dMatrixCoefficients mtrx; ///< matrix coefficients (av1)
    enum Rav1dChromaSamplePosition chr; ///< chroma sample position (av1)
    /**
     * 0, 1 and 2 mean 8, 10 or 12 bits/component, respectively. This is not
     * exactly the same as 'hbd' from the spec; the spec's hbd distinguishes
     * between 8 (0) and 10-12 (1) bits/component, and another element
     * (twelve_bit) to distinguish between 10 and 12 bits/component. To get
     * the spec's hbd, use !!our_hbd, and to get twelve_bit, use hbd == 2.
     */
    int hbd;
    /**
     * Pixel data uses JPEG pixel range ([0,255] for 8bits) instead of
     * MPEG pixel range ([16,235] for 8bits luma, [16,240] for 8bits chroma).
     */
    int color_range;

    int num_operating_points;
    struct Rav1dSequenceHeaderOperatingPoint {
        int major_level, minor_level;
        int initial_display_delay;
        int idc;
        int tier;
        int decoder_model_param_present;
        int display_model_param_present;
    } operating_points[RAV1D_MAX_OPERATING_POINTS];

    int still_picture;
    int reduced_still_picture_header;
    int timing_info_present;
    int num_units_in_tick;
    int time_scale;
    int equal_picture_interval;
    unsigned num_ticks_per_picture;
    int decoder_model_info_present;
    int encoder_decoder_buffer_delay_length;
    int num_units_in_decoding_tick;
    int buffer_removal_delay_length;
    int frame_presentation_delay_length;
    int display_model_info_present;
    int width_n_bits, height_n_bits;
    int frame_id_numbers_present;
    int delta_frame_id_n_bits;
    int frame_id_n_bits;
    int sb128;
    int filter_intra;
    int intra_edge_filter;
    int inter_intra;
    int masked_compound;
    int warped_motion;
    int dual_filter;
    int order_hint;
    int jnt_comp;
    int ref_frame_mvs;
    enum Rav1dAdaptiveBoolean screen_content_tools;
    enum Rav1dAdaptiveBoolean force_integer_mv;
    int order_hint_n_bits;
    int super_res;
    int cdef;
    int restoration;
    int ss_hor, ss_ver, monochrome;
    int color_description_present;
    int separate_uv_delta_q;
    int film_grain_present;

    // Rav1dSequenceHeaders of the same sequence are required to be
    // bit-identical until this offset. See 7.5 "Ordering of OBUs":
    //   Within a particular coded video sequence, the contents of
    //   sequence_header_obu must be bit-identical each time the
    //   sequence header appears except for the contents of
    //   operating_parameters_info.
    struct Rav1dSequenceHeaderOperatingParameterInfo {
        int decoder_buffer_delay;
        int encoder_buffer_delay;
        int low_delay_mode;
    } operating_parameter_info[RAV1D_MAX_OPERATING_POINTS];
} Rav1dSequenceHeader;

typedef struct Rav1dSegmentationData {
    int delta_q;
    int delta_lf_y_v, delta_lf_y_h, delta_lf_u, delta_lf_v;
    int ref;
    int skip;
    int globalmv;
} Rav1dSegmentationData;

typedef struct Rav1dSegmentationDataSet {
    Rav1dSegmentationData d[RAV1D_MAX_SEGMENTS];
    int preskip;
    int last_active_segid;
} Rav1dSegmentationDataSet;

typedef struct Rav1dLoopfilterModeRefDeltas {
    int mode_delta[2 /* is_zeromv */];
    int ref_delta[RAV1D_TOTAL_REFS_PER_FRAME];
} Rav1dLoopfilterModeRefDeltas;

typedef struct Rav1dFilmGrainData {
    uint16_t seed;
    int num_y_points;
    uint8_t y_points[14][2 /* value, scaling */];
    int chroma_scaling_from_luma;
    int num_uv_points[2];
    uint8_t uv_points[2][10][2 /* value, scaling */];
    int scaling_shift;
    int ar_coeff_lag;
    int8_t ar_coeffs_y[24];
    int8_t ar_coeffs_uv[2][25];
    int ar_coeff_shift;
    int grain_scale_shift;
    int uv_mult[2];
    int uv_luma_mult[2];
    int uv_offset[2];
    int overlap_flag;
    int clip_to_restricted_range;
} Rav1dFilmGrainData;

typedef struct Rav1dFrameHeader {
    enum Rav1dFrameType frame_type; ///< type of the picture
    int width[2 /* { coded_width, superresolution_upscaled_width } */], height;
    int frame_offset; ///< frame number
    struct {
        int present, update;
        Rav1dFilmGrainData data;
    } film_grain; ///< film grain parameters
    int temporal_id, spatial_id; ///< spatial and temporal id of the frame for SVC

    int show_existing_frame;
    int existing_frame_idx;
    int frame_id;
    int frame_presentation_delay;
    int show_frame;
    int showable_frame;
    int error_resilient_mode;
    int disable_cdf_update;
    int allow_screen_content_tools;
    int force_integer_mv;
    int frame_size_override;
    int primary_ref_frame;
    int buffer_removal_time_present;
    struct Rav1dFrameHeaderOperatingPoint {
        int buffer_removal_time;
    } operating_points[RAV1D_MAX_OPERATING_POINTS];
    int refresh_frame_flags;
    int render_width, render_height;
    struct {
        int width_scale_denominator;
        int enabled;
    } super_res;
    int have_render_size;
    int allow_intrabc;
    int frame_ref_short_signaling;
    int refidx[RAV1D_REFS_PER_FRAME];
    int hp;
    enum Rav1dFilterMode subpel_filter_mode;
    int switchable_motion_mode;
    int use_ref_frame_mvs;
    int refresh_context;
    struct {
        int uniform;
        unsigned n_bytes;
        int min_log2_cols, max_log2_cols, log2_cols, cols;
        int min_log2_rows, max_log2_rows, log2_rows, rows;
        uint16_t col_start_sb[RAV1D_MAX_TILE_COLS + 1];
        uint16_t row_start_sb[RAV1D_MAX_TILE_ROWS + 1];
        int update;
    } tiling;
    struct {
        int yac;
        int ydc_delta;
        int udc_delta, uac_delta, vdc_delta, vac_delta;
        int qm, qm_y, qm_u, qm_v;
    } quant;
    struct {
        int enabled, update_map, temporal, update_data;
        Rav1dSegmentationDataSet seg_data;
        int lossless[RAV1D_MAX_SEGMENTS], qidx[RAV1D_MAX_SEGMENTS];
    } segmentation;
    struct {
        struct {
            int present;
            int res_log2;
        } q;
        struct {
            int present;
            int res_log2;
            int multi;
        } lf;
    } delta;
    int all_lossless;
    struct {
        int level_y[2 /* dir */];
        int level_u, level_v;
        int mode_ref_delta_enabled;
        int mode_ref_delta_update;
        Rav1dLoopfilterModeRefDeltas mode_ref_deltas;
        int sharpness;
    } loopfilter;
    struct {
        int damping;
        int n_bits;
        int y_strength[RAV1D_MAX_CDEF_STRENGTHS];
        int uv_strength[RAV1D_MAX_CDEF_STRENGTHS];
    } cdef;
    struct {
        enum Rav1dRestorationType type[3 /* plane */];
        int unit_size[2 /* y, uv */];
    } restoration;
    enum Rav1dTxfmMode txfm_mode;
    int switchable_comp_refs;
    int skip_mode_allowed, skip_mode_enabled, skip_mode_refs[2];
    int warp_motion;
    int reduced_txtp_set;
    Rav1dWarpedMotionParams gmv[RAV1D_REFS_PER_FRAME];
} Rav1dFrameHeader;
*/
