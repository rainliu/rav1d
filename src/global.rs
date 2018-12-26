pub enum Rav1dCodecErr {
    Rav1dCodecOk,
    Rav1dCodecError,
    Rav1dCodecMemError,
    Rav1dCodecAbiMismatch,
    Rav1dCodecIncapable,
    Rav1dCodecUnsupBitstream,
    Rav1dCodecUnsupFeature,
    Rav1dCodecCorruptFrame,
    Rav1dCodecInvalidParam,
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrixCoefficients {
    Identity = 0,
    BT709,
    Unspecified,
    BT470M = 4,
    BT470BG,
    ST170M,
    ST240M,
    YCgCo,
    BT2020NonConstantLuminance,
    BT2020ConstantLuminance,
    ST2085,
    ChromaticityDerivedNonConstantLuminance,
    ChromaticityDerivedConstantLuminance,
    ICtCp,
}
impl Default for MatrixCoefficients {
    fn default() -> Self {
        MatrixCoefficients::Unspecified
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum ColorPrimaries {
    BT709 = 1,
    Unspecified,
    BT470M = 4,
    BT470BG,
    ST170M,
    ST240M,
    Film,
    BT2020,
    ST428,
    P3DCI,
    P3Display,
    Tech3213 = 22,
}
impl Default for ColorPrimaries {
    fn default() -> Self {
        ColorPrimaries::Unspecified
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum TransferCharacteristics {
    BT1886 = 1,
    Unspecified,
    BT470M = 4,
    BT470BG,
    ST170M,
    ST240M,
    Linear,
    Logarithmic100,
    Logarithmic316,
    XVYCC,
    BT1361E,
    SRGB,
    BT2020Ten,
    BT2020Twelve,
    PerceptualQuantizer,
    ST428,
    HybridLogGamma,
}
impl Default for TransferCharacteristics {
    fn default() -> Self {
        TransferCharacteristics::Unspecified
    }
}

#[derive(Default)]
pub struct SequenceHeader {
    pub seq_profile: u8,
    pub still_picture: bool,
    pub reduced_still_picture_header: bool,

    pub timing_info_present_flag: bool,
    pub timing_info: TimingInfo,

    pub decoder_model_info_present_flag: bool,
    pub decoder_model_info: DecoderModelInfo,

    pub initial_display_delay_present_flag: bool,

    pub operating_points_cnt_minus_1: u8,
    pub operating_points: [OperatingPoint; RAV1D_MAX_OPERATING_POINTS],

    pub frame_width_bits_minus_1: u8,
    pub frame_height_bits_minus_1: u8,

    pub max_frame_width_minus_1: u16,
    pub max_frame_height_minus_1: u16,

    pub frame_id_numbers_present_flag: bool,
    pub delta_frame_id_length_minus_2: u8,
    pub additional_frame_id_length_minus_1: u8,
    pub frame_id_length: u8,

    pub use_128x128_superblock: bool,
    pub enable_filter_intra: bool,
    pub enable_intra_edge_filter: bool,

    pub enable_interintra_compound: bool,
    pub enable_masked_compound: bool,
    pub enable_warped_motion: bool,
    pub enable_dual_filter: bool,
    pub enable_order_hint: bool,
    pub enable_jnt_comp: bool,
    pub enable_ref_frame_mvs: bool,
    pub seq_choose_screen_content_tools: bool,
    pub seq_force_screen_content_tools: u8,
    pub seq_choose_integer_mv: bool,
    pub seq_force_integer_mv: u8,
    pub order_hint_bits: u8,

    pub enable_superres: bool,
    pub enable_cdef: bool,
    pub enable_restoration: bool,

    pub color_config: ColorConfig,

    pub film_grain_params_present: bool,
}

#[derive(Default)]
pub struct TimingInfo {
    pub num_units_in_display_tick: u32,
    pub time_scale: u32,
    pub equal_picture_interval: bool,
    pub num_ticks_per_picture_minus_1: u32,
}

#[derive(Default)]
pub struct DecoderModelInfo {
    pub buffer_delay_length_minus_1: u8,
    pub num_units_in_decoding_tick: u32,
    pub buffer_removal_time_length_minus_1: u8,
    pub frame_presentation_time_length: u8,
}

#[derive(Default)]
pub struct OperatingParametersInfo {
    pub decoder_buffer_delay: u32,
    pub encoder_buffer_dealy: u32,
    pub low_delay_mode_flag: bool,
}

#[derive(Default)]
pub struct OperatingPoint {
    pub operating_point_idc: u16,
    pub seq_level_idx: u8,
    pub seq_tier: bool,
    pub decoder_model_present_for_this_op: bool,
    pub operating_parameters_info: OperatingParametersInfo,
    pub initial_display_delay_present_for_this_op: bool,
    pub initial_display_delay_minus_1: u8,
}

#[derive(Default)]
pub struct ColorConfig {
    pub high_bitdepth: bool,
    pub twelve_bit: bool,
    pub mono_chrome: bool,
    pub color_description_present_flag: bool,
    pub color_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub color_range: bool,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub chroma_sample_position: u8,
    pub separate_uv_delta_q: bool,

    pub bit_depth: u8,
}
