use super::Demuxer;
use ivf::*;
use std::fs::File;
use std::io;
use std::io::Read;

use rav1d::api::*;
use crate::common::*;

pub struct IvfDemuxer {
    input: Box<dyn Read>,
}

impl IvfDemuxer {
    pub fn new(path: &str) -> Box<dyn Demuxer> {
        let ivf = IvfDemuxer {
            input: match path {
                "-" => Box::new(io::stdin()),
                f => Box::new(File::open(&f).unwrap()),
            },
        };
        Box::new(ivf)
    }
}

impl Demuxer for IvfDemuxer {
    fn open(&mut self) -> io::Result<VideoDetails> {
        let hdr = read_header(&mut self.input)?;

        Ok(VideoDetails {
            width: hdr.w as usize,
            height: hdr.h as usize,
            bit_depth: 8,
            chroma_sampling: ChromaSampling::Cs420,
            chroma_sample_position: ChromaSamplePosition::Unknown,
            time_base: Rational {
                num: hdr.timebase_num as u64,
                den: hdr.timebase_den as u64,
            },
            num_frames: hdr.num_frames as usize,
        })
    }

    fn read(&mut self) -> io::Result<Rav1dData> {
        let pkt = read_packet(&mut self.input)?;

        Ok(Rav1dData {
            data: pkt.data,
            m: Rav1dDataProps {
                timestamp: pkt.pts,
                duration: 0,
                offset: 0,
                size: 0,
            },
        })
    }

    fn close(&mut self) {}
}
