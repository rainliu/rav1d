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
    pub(crate) alpha: i16,
    pub(crate) beta: i16,
    pub(crate) gamma: i16,
    pub(crate) delta: i16,
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
    pub(crate) t: WarpedMotionType,
    pub(crate) matrix: [i32; 6],
    pub(crate) u: WarpedMotionParamsUnion,
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
    CHR_VERTICAL = 1,
    CHR_COLOCATED = 2,
    CHR_RESERVED = 3,
}

impl Default for ChromaSamplePosition {
    fn default() -> Self {
        ChromaSamplePosition::CHR_UNKNOWN
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct ContentLightLevel {
    pub(crate) max_content_light_level: isize,
    pub(crate) max_frame_average_light_level: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct MasteringDisplay {
    // 0.16 fixed point
    pub(crate) primaries: [[u16; 3]; 2], //TODO: confirm [3][2]?
    // 0.16 fixed point
    pub(crate) white_point: [u16; 2],
    // 24.8 fixed point
    pub(crate) max_luminance: u32,
    // 18.14 fixed point
    pub(crate) min_luminance: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SequenceHeaderOperatingPoint {
    pub(crate) major_level: u32,
    pub(crate) minor_level: u32,
    pub(crate) initial_display_delay: u32,
    pub(crate) idc: u32,
    pub(crate) tier: u32,
    pub(crate) decoder_model_param_present: bool,
    pub(crate) display_model_param_present: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SequenceHeaderOperatingParameterInfo {
    pub(crate) decoder_buffer_delay: u32,
    pub(crate) encoder_buffer_delay: u32,
    pub(crate) low_delay_mode: bool,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct SequenceHeader {
    /**
     * Stream profile, 0 for 8-10 bits/component 4:2:0 or monochrome;
     * 1 for 8-10 bits/component 4:4:4; 2 for 4:2:2 at any bits/component,
     * or 12 bits/component at any chroma subsampling.
     */
    pub(crate) profile: u8,
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
    pub(crate) color_range: bool,

    pub(crate) num_operating_points: usize,
    pub(crate) operating_points: [SequenceHeaderOperatingPoint; MAX_OPERATING_POINTS],

    pub(crate) still_picture: bool,
    pub(crate) reduced_still_picture_header: bool,
    pub(crate) timing_info_present: bool,
    pub(crate) num_units_in_tick: u32,
    pub(crate) time_scale: u32,
    pub(crate) equal_picture_interval: bool,
    pub(crate) num_ticks_per_picture: u32,
    pub(crate) decoder_model_info_present: bool,
    pub(crate) encoder_decoder_buffer_delay_length: u32,
    pub(crate) num_units_in_decoding_tick: u32,
    pub(crate) buffer_removal_delay_length: u32,
    pub(crate) frame_presentation_delay_length: u32,
    pub(crate) display_model_info_present: bool,
    pub(crate) width_n_bits: u32,
    pub(crate) height_n_bits: u32,
    pub(crate) frame_id_numbers_present: bool,
    pub(crate) delta_frame_id_n_bits: u32,
    pub(crate) frame_id_n_bits: u32,
    pub(crate) sb128: bool,
    pub(crate) filter_intra: bool,
    pub(crate) intra_edge_filter: bool,
    pub(crate) inter_intra: bool,
    pub(crate) masked_compound: bool,
    pub(crate) warped_motion: bool,
    pub(crate) dual_filter: bool,
    pub(crate) order_hint: bool,
    pub(crate) jnt_comp: bool,
    pub(crate) ref_frame_mvs: bool,
    pub(crate) screen_content_tools: AdaptiveBoolean,
    pub(crate) force_integer_mv: AdaptiveBoolean,
    pub(crate) order_hint_n_bits: u32,
    pub(crate) super_res: bool,
    pub(crate) cdef: bool,
    pub(crate) restoration: bool,
    pub(crate) ss_hor: u32,
    pub(crate) ss_ver: u32,
    pub(crate) monochrome: bool,
    pub(crate) color_description_present: bool,
    pub(crate) separate_uv_delta_q: bool,
    pub(crate) film_grain_present: bool,

    // SequenceHeaders of the same sequence are required to be
    // bit-identical until this offset. See 7.5 "Ordering of OBUs":
    //   Within a particular coded video sequence, the contents of
    //   sequence_header_obu must be bit-identical each time the
    //   sequence header appears except for the contents of
    //   operating_parameters_info.
    pub(crate) operating_parameter_info:
        [SequenceHeaderOperatingParameterInfo; MAX_OPERATING_POINTS],
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
    pub(crate) delta_q: isize,
    pub(crate) delta_lf_y_v: isize,
    pub(crate) delta_lf_y_h: isize,
    pub(crate) delta_lf_u: isize,
    pub(crate) delta_lf_v: isize,
    pub(crate) ref_frame: isize,
    pub(crate) skip: isize,
    pub(crate) globalmv: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SegmentationDataSet {
    pub(crate) d: [SegmentationData; MAX_SEGMENTS],
    pub(crate) preskip: isize,
    pub(crate) last_active_segid: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct LoopfilterModeRefDeltas {
    pub(crate) mode_delta: [isize; 2],
    pub(crate) ref_delta: [isize; TOTAL_REFS_PER_FRAME],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FilmGrainData {
    pub(crate) seed: u16,
    pub(crate) num_y_points: isize,
    pub(crate) y_points: [[u8; 14]; 2], //TODO: [14][2]
    pub(crate) chroma_scaling_from_luma: isize,
    pub(crate) num_uv_points: [isize; 2],
    pub(crate) uv_points: [[[u8; 2]; 10]; 2], //TODO: [2][10][2]
    pub(crate) scaling_shift: isize,
    pub(crate) ar_coeff_lag: isize,
    pub(crate) ar_coeffs_y: [i8; 24],
    pub(crate) ar_coeffs_uv: [[i8; 2]; 25], //TODO: [2][25]
    pub(crate) ar_coeff_shift: isize,
    pub(crate) grain_scale_shift: isize,
    pub(crate) uv_mult: [isize; 2],
    pub(crate) uv_luma_mult: [isize; 2],
    pub(crate) uv_offset: [isize; 2],
    pub(crate) overlap_flag: isize,
    pub(crate) clip_to_restricted_range: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FilmGrain {
    pub(crate) present: isize,
    pub(crate) update: isize,
    pub(crate) data: FilmGrainData,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FrameHeaderOperatingPoint {
    pub(crate) buffer_removal_time: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct SuperResolution {
    pub(crate) width_scale_denominator: isize,
    pub(crate) enabled: isize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tiling {
    pub(crate) uniform: isize,
    pub(crate) n_bytes: usize,
    pub(crate) min_log2_cols: isize,
    pub(crate) max_log2_cols: isize,
    pub(crate) log2_cols: isize,
    pub(crate) cols: isize,
    pub(crate) min_log2_rows: isize,
    pub(crate) max_log2_rows: isize,
    pub(crate) log2_rows: isize,
    pub(crate) rows: isize,
    pub(crate) col_start_sb: [u16; MAX_TILE_COLS + 1],
    pub(crate) row_start_sb: [u16; MAX_TILE_ROWS + 1],
    pub(crate) update: isize,
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
    pub(crate) yac: isize,
    pub(crate) ydc_delta: isize,
    pub(crate) udc_delta: isize,
    pub(crate) uac_delta: isize,
    pub(crate) vdc_delta: isize,
    pub(crate) vac_delta: isize,
    pub(crate) qm: isize,
    pub(crate) qm_y: isize,
    pub(crate) qm_u: isize,
    pub(crate) qm_v: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Segmentation {
    pub(crate) enabled: isize,
    pub(crate) update_map: isize,
    pub(crate) temporal: isize,
    pub(crate) update_data: isize,
    pub(crate) seg_data: SegmentationDataSet,
    pub(crate) lossless: [isize; MAX_SEGMENTS],
    pub(crate) qidx: [isize; MAX_SEGMENTS],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Q {
    pub(crate) present: isize,
    pub(crate) res_log2: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct LF {
    pub(crate) present: isize,
    pub(crate) res_log2: isize,
    pub(crate) multi: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Delta {
    pub(crate) q: Q,
    pub(crate) lf: LF,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct LoopFilter {
    pub(crate) level_y: [isize; 2],
    pub(crate) level_u: isize,
    pub(crate) level_v: isize,
    pub(crate) mode_ref_delta_enabled: isize,
    pub(crate) mode_ref_delta_update: isize,
    pub(crate) mode_ref_deltas: LoopfilterModeRefDeltas,
    pub(crate) sharpness: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct CDEF {
    pub(crate) damping: isize,
    pub(crate) n_bits: isize,
    pub(crate) y_strength: [isize; MAX_CDEF_STRENGTHS],
    pub(crate) uv_strength: [isize; MAX_CDEF_STRENGTHS],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Restoration {
    pub(crate) t: [RestorationType; 3],
    pub(crate) unit_size: [isize; 2],
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct FrameHeader {
    pub(crate) frame_type: FrameType, // type of the picture
    pub(crate) width: [isize; 2],
    pub(crate) height: isize,
    pub(crate) frame_offset: isize,   // frame number
    pub(crate) film_grain: FilmGrain, // film grain parameters
    pub(crate) temporal_id: isize,
    pub(crate) spatial_id: isize, // spatial and temporal id of the frame for SVC
    pub(crate) show_existing_frame: isize,
    pub(crate) existing_frame_idx: isize,
    pub(crate) frame_id: isize,
    pub(crate) frame_presentation_delay: isize,
    pub(crate) show_frame: isize,
    pub(crate) showable_frame: isize,
    pub(crate) error_resilient_mode: isize,
    pub(crate) disable_cdf_update: isize,
    pub(crate) allow_screen_content_tools: isize,
    pub(crate) force_integer_mv: isize,
    pub(crate) frame_size_override: isize,
    pub(crate) primary_ref_frame: isize,
    pub(crate) buffer_removal_time_present: isize,
    pub(crate) operating_points: [FrameHeaderOperatingPoint; MAX_OPERATING_POINTS],
    pub(crate) refresh_frame_flags: isize,
    pub(crate) render_width: isize,
    pub(crate) render_height: isize,
    pub(crate) super_res: SuperResolution,
    pub(crate) have_render_size: isize,
    pub(crate) allow_intrabc: isize,
    pub(crate) frame_ref_short_signaling: isize,
    pub(crate) refidx: [isize; REFS_PER_FRAME],
    pub(crate) hp: isize,
    pub(crate) subpel_filter_mode: FilterMode,
    pub(crate) switchable_motion_mode: isize,
    pub(crate) use_ref_frame_mvs: isize,
    pub(crate) refresh_context: isize,
    pub(crate) tiling: Tiling,
    pub(crate) quant: Quant,
    pub(crate) segmentation: Segmentation,
    pub(crate) delta: Delta,
    pub(crate) all_lossless: isize,
    pub(crate) loopfilter: LoopFilter,
    pub(crate) cdef: CDEF,
    pub(crate) restoration: Restoration,
    pub(crate) txfm_mode: TxfmMode,
    pub(crate) switchable_comp_refs: isize,
    pub(crate) skip_mode_allowed: isize,
    pub(crate) skip_mode_enabled: isize,
    pub(crate) skip_mode_refs: [isize; 2],
    pub(crate) warp_motion: isize,
    pub(crate) reduced_txtp_set: isize,
    pub(crate) gmv: [WarpedMotionParams; REFS_PER_FRAME],
}

impl FrameHeader {
    pub fn new() -> Self {
        FrameHeader {
            ..Default::default()
        }
    }
}
