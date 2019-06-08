use rav1d::api::*;

use std::io;

pub trait Demuxer {
    fn open(&mut self, filename: &str, fps: [usize; 2]) -> io::Result<usize>;

    fn read(&mut self, d: &Rav1dData);

    fn close(&mut self);
}
