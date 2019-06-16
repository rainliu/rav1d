use crate::api::*;
use crate::frame::Frame;
use crate::getbits::*;
use crate::headers::*;
use crate::internal::*;
use crate::levels::*;
use crate::util::Pixel;

use std::rc::Rc;
use std::slice;
use std::vec::Vec;
use std::{cmp, io};

impl<T: Pixel> Context<T> {
    pub fn submit_frame(&mut self) -> io::Result<()> {
        // TODO:
        // initialize context
        let fc = if self.n_fc > 1 {
            // multi-threading
            unimplemented!()
        } else {
            self.fcs.first().unwrap()
        };

        if self.n_fc == 1 {
            // single threading
            self.decode_frame(fc)?;
            //if ((res = dav1d_decode_frame(f)) < 0) {
            /*let refresh_frame_flags = f->frame_hdr->refresh_frame_flags;
            dav1d_picture_unref_internal(&c->out);
            for (int i = 0; i < 8; i++) {
                if (refresh_frame_flags & (1 << i)) {
                    if (c->refs[i].p.p.data[0])
                    dav1d_thread_picture_unref(&c->refs[i].p);
                    dav1d_cdf_thread_unref(&c->cdf[i]);
                    dav1d_ref_dec(&c->refs[i].segmap);
                    dav1d_ref_dec(&c->refs[i].refmvs);
                }
            }
            return res;*/
            //}
        } else {
            // multi-threading
            unimplemented!();
        }

        Ok(())
    }

    fn decode_frame(&self, fc: &FrameContext) -> io::Result<()> {
        let seq_hdr = fc.seq_hdr.as_ref().unwrap();
        let frame_hdr = fc.frame_hdr.as_ref().unwrap();

        if fc.n_tc > 1 {
            unimplemented!();
        }

        if frame_hdr.tiling.cols * frame_hdr.tiling.rows > fc.n_ts {

        }

        Ok(())
    }
}
