use rav1d::picture::*;

use std::io;

mod y4m;
use self::y4m::Y4mMuxer;

use crate::common::*;

pub trait Muxer {
    fn open(
        &mut self,
        p: &VideoDetails,
    ) -> io::Result<()>;

    fn write(&mut self, p: &Rav1dPicture);

    fn close(&mut self);
}

pub fn new(filename: &str) -> Box<dyn Muxer> {
    Y4mMuxer::new(filename)
}