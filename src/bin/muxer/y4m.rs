use super::Muxer;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};

use crate::common::*;
use rav1d::picture::*;
use y4m;

pub struct Y4mMuxer<'a> {
    output: Box<dyn Write>,
    encoder: Option<y4m::Encoder<'a, Box<dyn Write>>>,
}

impl<'a> Y4mMuxer<'a> {
    pub fn new(path: &str) -> Box<dyn Muxer> {
        let y4m = Y4mMuxer {
            output: match path {
                "-" => Box::new(io::stdout()),
                f => Box::new(File::create(&f).unwrap()),
            },
            encoder: None,
        };
        Box::new(y4m)
    }
}

impl<'a> Muxer for Y4mMuxer<'a> {
    fn open(&mut self, p: &VideoDetails) -> io::Result<()> {
        /*let enc = match y4m::EncoderBuilder::new(
            p.width,
            p.height,
            y4m::Ratio::new(p.time_base.num as usize, p.time_base.den as usize),
        ).write_header(&mut self.output) {
            Ok(enc) => enc,
            Err(err) => return Err(Error::new(ErrorKind::Other, "error in y4m write_header")),
        };
        self.encoder = Some(enc);
        */
        Ok(())
    }

    fn write(&mut self, p: &Rav1dPicture) {}

    fn close(&mut self) {}
}
