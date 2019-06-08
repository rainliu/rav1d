use rav1d::api::*;

use std::io;

mod ivf;
use self::ivf::IvfDemuxer;

use crate::common::*;

pub trait Demuxer {
    fn open(&mut self) -> io::Result<VideoDetails>;

    fn read(&mut self) -> io::Result<Rav1dData>;

    fn close(&mut self);
}

pub fn new(filename: &str) -> Box<dyn Demuxer> {
    IvfDemuxer::new(filename)
}
