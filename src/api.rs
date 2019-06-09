use crate::headers::*;
use crate::util::Pixel;

use std::{cmp, fmt, io};

#[derive(Copy, Clone, Debug, PartialEq)] //, FromPrimitive
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

#[derive(Copy, Clone, Debug, PartialEq)] //, FromPrimitive
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

const RAV1D_MAX_FRAME_THREADS: usize = 256;
const RAV1D_MAX_TILE_THREADS: usize = 64;

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
    /*
    pub fn new_context<T: Pixel>(&self) -> Context<T> {
        assert!(8 * std::mem::size_of::<T>() >= self.enc.bit_depth, "The Pixel u{} does not match the Config bit_depth {}",
                8 * std::mem::size_of::<T>(), self.enc.bit_depth);

        let pool = rayon::ThreadPoolBuilder::new().num_threads(self.threads).build().unwrap();

        let mut config = self.enc.clone();

        // FIXME: inter unsupported with 4:2:2 and 4:4:4 chroma sampling
        let chroma_sampling = config.chroma_sampling;
        let keyframe_only = chroma_sampling == ChromaSampling::Cs444 ||
            chroma_sampling == ChromaSampling::Cs422;
        if keyframe_only {
            config.max_key_frame_interval = 1;
            config.min_key_frame_interval = 1;
        }
        // FIXME: tx partition for intra not supported for chroma 422
        if chroma_sampling == ChromaSampling::Cs422 {
            config.speed_settings.rdo_tx_decision = false;
        }

        let inner = ContextInner::new(&config);

        Context {
            inner,
            pool,
            config
        }
    }*/
}

pub struct ContextInner {
    //<T: Pixel>
/*frame_count: u64,
limit: u64,
pub(crate) idx: u64,
frames_processed: u64,
/// Maps frame *number* to frames
frame_q: BTreeMap<u64, Option<Arc<Frame<T>>>>, //    packet_q: VecDeque<Packet>
/// Maps frame *idx* to frame data
frame_invariants: BTreeMap<u64, FrameInvariants<T>>,
/// A list of keyframe *numbers* in this encode. Needed so that we don't
/// need to keep all of the frame_invariants in memory for the whole life of the encode.
keyframes: BTreeSet<u64>,
/// A storage space for reordered frames.
packet_data: Vec<u8>,
segment_start_idx: u64,
segment_start_frame: u64,
keyframe_detector: SceneChangeDetector<T>,
pub(crate) config: EncoderConfig,
rc_state: RCState,
maybe_prev_log_base_q: Option<i64>,
pub first_pass_data: FirstPassData,*/}

pub struct Context {
    //<T: Pixel>
    inner: ContextInner, //<T>,
    config: Config,
    //pool: rayon::ThreadPool,
}

#[derive(Clone, Copy, Debug)]
pub enum CodecStatus {
    /// The codec needs more data to produce an output Packet/Frame
    NeedMoreData,
    /// There are enough Frames/Packets queue
    EnoughData,
    /// The codec already produced the number of frames/packets requested
    LimitReached,
    /// A Frame had been encoded/decoded but not emitted yet
    Encoded,
    /// Generic fatal error
    Failure,
    /// A Frame had been decoded but not emitted yet
    Decoded,
}

pub struct Packet {
    pub data: Vec<u8>,
    pub pts: u64,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Frame {} - {} bytes", self.pts, self.data.len())
    }
}
