use crate::frame::Frame;
use crate::headers::*;
use crate::obu::*;
use crate::util::Pixel;
use crate::internal::*;

use std::rc::Rc;
use std::{cmp, fmt, io};
use std::vec::Vec;

use arg_enum_proc_macro::ArgEnum;
use num_derive::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
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

impl Default for CodecStatus {
    fn default() -> Self {
        CodecStatus::NeedMoreData
    }
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
    pub n_frame_threads: usize,
    pub n_tile_threads: usize,
    pub apply_grain: isize,
    pub operating_point: isize, // select an operating point for scalable AV1 bitstreams (0 - 31)
    pub all_layers: isize,      // output all spatial layers of a scalable AV1 biststream
    pub frame_size_limit: usize, // maximum frame size, in pixels (0 = unlimited)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            n_frame_threads: 1,
            n_tile_threads: 1,
            apply_grain: 0,
            operating_point: 0,
            all_layers: 1, // just until the tests are adjusted
            frame_size_limit: 0,
        }
    }
}

// reference/entropy state
pub(crate) struct RefState{
    //Dav1dThreadPicture p;
    //Dav1dRef *segmap;
    //Dav1dRef *refmvs;
    refpoc:[u32; 7],
}

pub struct Context<T: Pixel> {
    pub(crate) n_fc: usize,
    pub(crate) fc: Vec<FrameContext>,

    pub(crate) seq_hdr: Option<Rc<SequenceHeader>>,
    pub(crate) frame_hdr: Option<Rc<FrameHeader>>,
    pub(crate) tile: Vec<TileGroup>,
    pub(crate) n_tiles: i32,
    //pub(crate) refs: [RefState; 8],

    pub(crate) apply_grain: bool,
    pub(crate) operating_point: usize,
    pub(crate) operating_point_idc: u32,
    pub(crate) all_layers: i32,
    pub(crate) frame_size_limit: usize,
    pub(crate) drain: bool,
    pub(crate) frame: Option<Frame<T>>,
    pub(crate) packet: Option<Packet>,
    //pub(crate) pool: rayon::ThreadPool,
}

impl<T: Pixel> Context<T> {
    pub fn new(cfg: &Config) -> Self {
        debug_assert!(cfg.n_frame_threads > 0);

        Context {
            n_fc: cfg.n_frame_threads,
            fc: vec![FrameContext::default(); cfg.n_frame_threads],

            seq_hdr: None,
            frame_hdr: None,
            tile: vec![],
            n_tiles: 0,

            apply_grain: false,
            operating_point: 0,
            operating_point_idc: 0,
            all_layers: 0,
            frame_size_limit: 0,
            drain: false,
            frame: None,
            packet: None,
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
