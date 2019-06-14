use crate::frame::Frame;
use crate::headers::*;
use crate::obu::*;
use crate::util::Pixel;

use std::{cmp, fmt, io};

use arg_enum_proc_macro::ArgEnum;
use num_derive::*;

#[derive(Clone, Copy, Debug)]
pub enum CodecStatus {
    /// The codec needs more data to produce an output Packet/Frame
    NeedMoreData,
    /// There are enough Frames/Packets queue
    EnoughData,
    /// The codec already produced the number of frames/packets requested
    LimitReached,
    /// A Frame had been decoded but not emitted yet
    //Decoded,
    /// Generic fatal error
    Failure,
}

pub struct Packet {
    pub data: Vec<u8>,
    pub offset: usize,
    pub pts: u64,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Packet {} - {} bytes", self.pts, self.data.len())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum ChromaSampling {
    Cs420,
    Cs422,
    Cs444,
    Cs400,
}

impl Default for ChromaSampling {
    fn default() -> Self {
        ChromaSampling::Cs420
    }
}

impl ChromaSampling {
    // Provides the sampling period in the horizontal and vertical axes.
    pub fn sampling_period(self) -> (usize, usize) {
        use self::ChromaSampling::*;
        match self {
            Cs420 => (2, 2),
            Cs422 => (2, 1),
            Cs444 => (1, 1),
            Cs400 => (2, 2),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum ChromaSamplePosition {
    Unknown,
    Vertical,
    Colocated,
}

impl Default for ChromaSamplePosition {
    fn default() -> Self {
        ChromaSamplePosition::Unknown
    }
}

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum PixelRange {
    Unspecified = 0,
    Limited,
    Full,
}

impl Default for PixelRange {
    fn default() -> Self {
        PixelRange::Unspecified
    }
}

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(C)]
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

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(C)]
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

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(C)]
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

#[derive(Copy, Clone, Debug)]
pub struct ColorDescription {
    pub color_primaries: ColorPrimaries,
    pub transfer_characteristics: TransferCharacteristics,
    pub matrix_coefficients: MatrixCoefficients,
}

#[derive(Copy, Clone, Debug)]
pub struct MasteringDisplay {
    pub primaries: [Point; 3],
    pub white_point: Point,
    pub max_luminance: u32,
    pub min_luminance: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct ContentLight {
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Rational {
    pub num: u64,
    pub den: u64,
}

impl Rational {
    pub fn new(num: u64, den: u64) -> Self {
        Rational { num, den }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub threads: usize, // The number of threads in the threadpool.
    pub apply_grain: isize,
    pub operating_point: isize, // select an operating point for scalable AV1 bitstreams (0 - 31)
    pub all_layers: isize,      // output all spatial layers of a scalable AV1 biststream
    pub frame_size_limit: usize, // maximum frame size, in pixels (0 = unlimited)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            threads: 1,
            apply_grain: 0,
            operating_point: 0,
            all_layers: 1, // just until the tests are adjusted
            frame_size_limit: 0,
        }
    }
}

impl Config {
    pub fn new_context<T: Pixel>(&self) -> Context<T> {
        //assert!(8 * std::mem::size_of::<T>() >= self.enc.bit_depth, "The Pixel u{} does not match the Config bit_depth {}",
        //        8 * std::mem::size_of::<T>(), self.enc.bit_depth);

        //let pool = rayon::ThreadPoolBuilder::new().num_threads(self.threads).build().unwrap();

        let mut config = self.clone();

        // FIXME: inter unsupported with 4:2:2 and 4:4:4 chroma sampling
        /*let chroma_sampling = config.chroma_sampling;
        let keyframe_only = chroma_sampling == ChromaSampling::Cs444 ||
            chroma_sampling == ChromaSampling::Cs422;
        if keyframe_only {
            config.max_key_frame_interval = 1;
            config.min_key_frame_interval = 1;
        }
        // FIXME: tx partition for intra not supported for chroma 422
        if chroma_sampling == ChromaSampling::Cs422 {
            config.speed_settings.rdo_tx_decision = false;
        }*/

        Context::new(config)
    }
}

pub struct Context<T: Pixel> {
    pub(crate) apply_grain: bool,
    pub(crate) operating_point: i32,
    pub(crate) operating_point_idc: u32,
    pub(crate) all_layers: i32,
    //frame_size_limit: usize,
    pub(crate) drain: bool,
    pub(crate) frame: Option<Frame<T>>,
    pub(crate) packet: Option<Packet>,
    config: Config,
    //pub(crate) pool: rayon::ThreadPool,
}

impl<T: Pixel> Context<T> {
    pub fn new(config: Config) -> Self {
        Context {
            apply_grain: false,
            operating_point: 0,
            operating_point_idc: 0,
            all_layers: 0,
            //frame_size_limit: 0,
            drain: false,
            frame: None,
            packet: None,
            config: config,
        }
    }

    pub fn send_packet(&mut self, pkt: &mut Option<Packet>) -> Result<(), CodecStatus> {
        if pkt.is_none() {
            return Err(CodecStatus::NeedMoreData);
        }

        self.drain = false;

        if self.packet.is_some() {
            return Err(CodecStatus::EnoughData);
        }

        self.packet = pkt.take();

        Ok(())
    }

    pub fn receive_frame(&mut self) -> Result<Frame<T>, CodecStatus> {
        if self.drain {
            return self.drain_frame();
        }

        if self.packet.is_none() {
            return Err(CodecStatus::NeedMoreData);
        }

        let pkt = self.packet.as_ref().unwrap();
        let (mut offset, size) = (pkt.offset, pkt.data.len());

        while offset < size {
            let res = self.parse_obus(offset, false);
            let err = res.is_err();
            if err {
                self.packet.take(); // all packet data are consumed, then release it
            } else {
                offset += res.unwrap();
                if offset >= size {
                    self.packet.take();
                }
            }
            if self.frame.is_some() {
                break;
            } else if err {
                return Err(CodecStatus::Failure);
            }
        }

        if self.packet.is_some() {
            self.packet.as_mut().unwrap().offset = offset;
        }

        let frame = self.frame.take();
        match frame {
            Some(f) => Ok(f),
            None => Err(CodecStatus::NeedMoreData),
        }
    }

    pub fn flush(&mut self) {
        self.drain = true;
    }

    fn drain_frame(&mut self) -> Result<Frame<T>, CodecStatus> {
        Err(CodecStatus::LimitReached)
    }
}
