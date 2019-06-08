use rav1d::picture::*;

use std::io;

pub trait Muxer {
    fn open(
        &mut self,
        filename: &str,
        fps: [usize; 2],
        p: &Rav1dPictureParameters,
    ) -> io::Result<()>;

    fn write<T>(&mut self, p: &Rav1dPicture<T>);

    fn close(&mut self);
}
