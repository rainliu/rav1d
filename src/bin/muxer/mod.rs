use std::io;

mod y4m;
use self::y4m::Y4mMuxer;

use crate::common::*;
use rav1d::frame::Frame;

pub trait Muxer {
    fn open(&mut self, d: &VideoDetails) -> io::Result<()>;

    fn write(&mut self, f: &Frame<u8>);

    fn close(&mut self);
}

pub fn new(filename: &str) -> Box<dyn Muxer> {
    Y4mMuxer::new(filename)
}
