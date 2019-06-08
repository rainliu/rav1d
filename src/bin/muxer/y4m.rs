use super::Muxer;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};

use crate::common::*;
use rav1d::picture::*;
use y4m::*;

pub struct Y4mMuxer {
    output: Box<dyn Write>,
    y_len: usize,
    u_len: usize,
    v_len: usize,
}

impl Y4mMuxer {
    pub fn new(path: &str) -> Box<dyn Muxer> {
        let y4m = Y4mMuxer {
            output: match path {
                "-" => Box::new(io::stdout()),
                f => Box::new(File::create(&f).unwrap()),
            },
            y_len: 0,
            u_len: 0,
            v_len: 0,
        };
        Box::new(y4m)
    }
}

impl Muxer for Y4mMuxer {
    fn open(&mut self, p: &VideoDetails) -> io::Result<()> {
        match EncoderBuilder::new(
            p.width,
            p.height,
            Ratio::new(p.time_base.num as usize, p.time_base.den as usize),
        )
        .write_header(&mut self.output)
        {
            Ok(enc) => enc,
            Err(_) => return Err(Error::new(ErrorKind::Other, "error in y4m write_header")),
        };

        let (y_len, u_len, v_len) = get_plane_sizes(p.width, p.height, Colorspace::C420);
        self.y_len = y_len;
        self.u_len = u_len;
        self.v_len = v_len;

        Ok(())
    }

    fn write(&mut self, p: &Rav1dPicture) {
        let mut enc = Encoder {
            writer: &mut self.output,
            y_len:self.y_len,
            u_len:self.u_len,
            v_len:self.v_len,
        };

    }

    fn close(&mut self) {}
}
