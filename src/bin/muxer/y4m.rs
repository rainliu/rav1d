use super::Muxer;
use y4m;

use std::fs::File;
use std::io;
use std::io::Write;
use std::slice;

use crate::common::*;
use rav1d::frame::*;
use rav1d::util::*;

pub struct Y4mMuxer {
    output: Box<dyn Write>,
    y_len: usize,
    u_len: usize,
    v_len: usize,
    y4m_details: VideoDetails,
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
            y4m_details: VideoDetails {
                ..Default::default()
            },
        };
        Box::new(y4m)
    }
}

impl Muxer for Y4mMuxer {
    fn open(&mut self, p: &VideoDetails) -> io::Result<()> {
        match y4m::EncoderBuilder::new(
            p.width,
            p.height,
            y4m::Ratio::new(p.time_base.num as usize, p.time_base.den as usize),
        )
        .write_header(&mut self.output)
        {
            Ok(enc) => enc,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "error in y4m write_header",
                ))
            }
        };

        let (y_len, u_len, v_len) = y4m::get_plane_sizes(p.width, p.height, y4m::Colorspace::C420);
        self.y_len = y_len;
        self.u_len = u_len;
        self.v_len = v_len;
        self.y4m_details = *p;

        Ok(())
    }

    fn write(&mut self, f: &Frame<u8>) {
        let mut y4m_enc = y4m::Encoder {
            writer: &mut self.output,
            y_len: self.y_len,
            u_len: self.u_len,
            v_len: self.v_len,
        };

        let pitch_y = if self.y4m_details.bit_depth > 8 {
            self.y4m_details.width * 2
        } else {
            self.y4m_details.width
        };
        let chroma_sampling_period = self.y4m_details.chroma_sampling.sampling_period();
        let (pitch_uv, height_uv) = (
            pitch_y / chroma_sampling_period.0,
            self.y4m_details.height / chroma_sampling_period.1,
        );

        let (mut rec_y, mut rec_u, mut rec_v) = (
            vec![128u8; pitch_y * self.y4m_details.height],
            vec![128u8; pitch_uv * height_uv],
            vec![128u8; pitch_uv * height_uv],
        );

        let (stride_y, stride_u, stride_v) = (
            f.planes[0].cfg.stride,
            f.planes[1].cfg.stride,
            f.planes[2].cfg.stride,
        );

        for (line, line_out) in f.planes[0]
            .data_origin()
            .chunks(stride_y)
            .zip(rec_y.chunks_mut(pitch_y))
        {
            if self.y4m_details.bit_depth > 8 {
                unsafe {
                    line_out.copy_from_slice(slice::from_raw_parts::<u8>(
                        line.as_ptr() as (*const u8),
                        pitch_y,
                    ));
                }
            } else {
                line_out.copy_from_slice(
                    &line.iter().map(|&v| u8::cast_from(v)).collect::<Vec<u8>>()[..pitch_y],
                );
            }
        }
        for (line, line_out) in f.planes[1]
            .data_origin()
            .chunks(stride_u)
            .zip(rec_u.chunks_mut(pitch_uv))
        {
            if self.y4m_details.bit_depth > 8 {
                unsafe {
                    line_out.copy_from_slice(slice::from_raw_parts::<u8>(
                        line.as_ptr() as (*const u8),
                        pitch_uv,
                    ));
                }
            } else {
                line_out.copy_from_slice(
                    &line.iter().map(|&v| u8::cast_from(v)).collect::<Vec<u8>>()[..pitch_uv],
                );
            }
        }
        for (line, line_out) in f.planes[2]
            .data_origin()
            .chunks(stride_v)
            .zip(rec_v.chunks_mut(pitch_uv))
        {
            if self.y4m_details.bit_depth > 8 {
                unsafe {
                    line_out.copy_from_slice(slice::from_raw_parts::<u8>(
                        line.as_ptr() as (*const u8),
                        pitch_uv,
                    ));
                }
            } else {
                line_out.copy_from_slice(
                    &line.iter().map(|&v| u8::cast_from(v)).collect::<Vec<u8>>()[..pitch_uv],
                );
            }
        }

        let rec_frame = y4m::Frame::new([&rec_y, &rec_u, &rec_v], None);
        y4m_enc.write_frame(&rec_frame).unwrap();
    }

    fn close(&mut self) {}
}
